// pub use der::{Decodable, Encodable};
//! Trait definitions

use core::convert::TryFrom;
use crate::{Decoder, Encoder, Error, header::Header, Length, Result, Tag, TaggedSlice, TaggedValue};

#[cfg(feature = "alloc")]
use {
    alloc::vec::Vec,
    core::{convert::TryInto, iter},
    crate::ErrorKind,
};

#[cfg(feature = "heapless")]
use {
    core::convert::TryInto,
    crate::{Error, ErrorKind},
};

/// Decoding trait:
///
/// Decode out of decoder, which essentially is a slice of bytes.
pub trait Decodable<'a>: Sized {
    /// Attempt to decode this message using the provided decoder.
    fn decode(decoder: &mut Decoder<'a>) -> Result<Self>;

    /// Parse `Self` from the provided byte slice.
    fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        let mut decoder = Decoder::new(bytes);
        let result = Self::decode(&mut decoder)?;
        decoder.finish(result)
    }
}

impl<'a, T> Decodable<'a> for T
where
    T: TryFrom<TaggedSlice<'a>, Error = Error>,
{
    fn decode(decoder: &mut Decoder<'a>) -> Result<T> {
        TaggedSlice::decode(decoder)
            .and_then(Self::try_from)
            .or_else(|e| decoder.error(e.kind()))
    }
}

/// Encoding trait
///
/// Encode into encoder, which essentially is a mutable slice of bytes.
///
/// Additionally, the encoded length needs to be known without actually encoding.
pub trait Encodable {
    /// Compute the length of this value in bytes when encoded as SIMPLE-TLV
    fn encoded_length(&self) -> Result<Length>;

    /// Encode this value as SIMPLE-TLV using the provided [`Encoder`].
    fn encode(&self, encoder: &mut Encoder<'_>) -> Result<()>;

    /// Encode this value to the provided byte slice, returning a sub-slice
    /// containing the encoded message.
    fn encode_to_slice<'a>(&self, buf: &'a mut [u8]) -> Result<&'a [u8]> {
        let mut encoder = Encoder::new(buf);
        self.encode(&mut encoder)?;
        Ok(encoder.finish()?)
    }

    /// Encode this message as SIMPLE-TLV, appending it to the provided
    /// byte vector.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    fn encode_to_vec(&self, buf: &mut Vec<u8>) -> Result<Length> {
        let expected_len = self.encoded_length()?.to_usize();
        let current_len = buf.len();
        buf.reserve(expected_len);
        buf.extend(iter::repeat(0).take(expected_len));

        // TODO(nickray): seems the original in `der` is incorrect here?
        // let mut encoder = Encoder::new(buf);
        let mut encoder = Encoder::new(&mut buf[current_len..]);
        self.encode(&mut encoder)?;
        let actual_len = encoder.finish()?.len();

        if expected_len != actual_len {
            return Err(ErrorKind::Underlength {
                expected: expected_len.try_into()?,
                actual: actual_len.try_into()?,
            }
            .into());
        }

        actual_len.try_into()
    }

    /// Serialize this message as a byte vector.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    fn to_vec(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.encode_to_vec(&mut buf)?;
        Ok(buf)
    }

    /// Encode this message as SIMPLE-TLV, appending it to the provided
    /// heapless byte vector.
    #[cfg(feature = "heapless")]
    #[cfg_attr(docsrs, doc(cfg(feature = "heapless")))]
    fn encode_to_heapless_vec<N: heapless::ArrayLength<u8>>(&self, buf: &mut heapless::Vec<u8, N>) -> Result<Length> {
        let expected_len = self.encoded_length()?.to_usize();
        let current_len = buf.len();
        // TODO(nickray): add a specific error for "Overcapacity" conditional on heapless feature?
        buf.resize_default(current_len + expected_len).map_err(|_| Error::from(ErrorKind::Overlength))?;

        let mut encoder = Encoder::new(&mut buf[current_len..]);
        self.encode(&mut encoder)?;
        let actual_len = encoder.finish()?.len();

        if expected_len != actual_len {
            return Err(ErrorKind::Underlength {
                expected: expected_len.try_into()?,
                actual: actual_len.try_into()?,
            }
            .into());
        }

        actual_len.try_into()
    }

    /// Serialize this message as a byte vector.
    #[cfg(feature = "heapless")]
    #[cfg_attr(docsrs, doc(cfg(feature = "heapless")))]
    fn to_heapless_vec<N: heapless::ArrayLength<u8>>(&self) -> Result<heapless::Vec<u8, N>> {
        let mut buf = heapless::Vec::new();
        self.encode_to_heapless_vec(&mut buf)?;
        Ok(buf)
    }
}

/// Types that have can be tagged.
pub trait Taggable: Sized {
    fn tagged(&self, tag: Tag) -> TaggedValue<&Self> {
        TaggedValue::new(tag, self)
    }
}

impl<X> Taggable for X where X: Sized {}

// /// Types with an associated SIMPLE-TLV [`Tag`].
// pub trait Tagged {
//     /// SIMPLE-TLV tag
//     const TAG: Tag;
// }

/// Messages encoded nested SIMPLE-TLV.
///
/// This wraps up a common pattern for SIMPLE-TLV encoding.
/// - implement `TryFrom<TaggedSlice<'a>, Error = Error>` to get a default `Decodable` implementation
/// - implement `Message` to get a default `Encodable` implementation
///
/// Types which impl this trait receive blanket impls for the [`Encodable`] trait.
pub trait Message<'a>: Decodable<'a> {
    /// The tag for the message itself
    fn tag() -> Tag;

    /// Call the provided function with a slice of [`Encodable`] trait objects
    /// representing the fields of this message.
    ///
    /// This method uses a callback because structs with fields which aren't
    /// directly [`Encodable`] may need to construct temporary values from
    /// their fields prior to encoding.
    fn fields<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&[&dyn Encodable]) -> Result<T>;
}

impl<'a, M> Encodable for M
where
    M: Message<'a>,
{
    fn encoded_length(&self) -> Result<Length> {
        let value_length = self.fields(|encodables| Length::try_from(encodables))?;
        Header::new(Self::tag(), value_length)?.encoded_length() + value_length
    }

    fn encode(&self, encoder: &mut Encoder<'_>) -> Result<()> {
        self.fields(|fields| encoder.nested(Self::tag(), fields))
    }
}

#[cfg(test)]
mod tests {

    use core::convert::{TryFrom, TryInto};
    use crate::{Decodable, Decoder, Encodable, Encoder, Error, Length, Result, Tag, Taggable, TaggedSlice};
    use super::Message;

    impl Encodable for [u8; 2] {
        fn encoded_length(&self) -> Result<Length> {
            Ok(2u8.into())
        }

        /// Encode this value as SIMPLE-TLV using the provided [`Encoder`].
        fn encode(&self, encoder: &mut Encoder<'_>) -> Result<()> {
            encoder.bytes(self.as_ref())
        }
    }

    impl Encodable for [u8; 3] {
        fn encoded_length(&self) -> Result<Length> {
            Ok(3u8.into())
        }

        /// Encode this value as SIMPLE-TLV using the provided [`Encoder`].
        fn encode(&self, encoder: &mut Encoder<'_>) -> Result<()> {
            encoder.bytes(self.as_ref())
        }
    }

    impl Encodable for [u8; 4] {
        fn encoded_length(&self) -> Result<Length> {
            Ok(4u8.into())
        }

        /// Encode this value as SIMPLE-TLV using the provided [`Encoder`].
        fn encode(&self, encoder: &mut Encoder<'_>) -> Result<()> {
            encoder.bytes(self.as_ref())
        }
    }

    impl Decodable<'_> for [u8; 2] {
        fn decode(decoder: &mut Decoder<'_>) -> Result<Self> {
            let bytes: &[u8] = decoder.bytes(2u8)?;
            Ok(bytes.try_into().unwrap())
        }
    }

    impl Decodable<'_> for [u8; 3] {
        fn decode(decoder: &mut Decoder<'_>) -> Result<Self> {
            let bytes: &[u8] = decoder.bytes(3u8)?;
            Ok(bytes.try_into().unwrap())
        }
    }

    impl Decodable<'_> for [u8; 4] {
        fn decode(decoder: &mut Decoder<'_>) -> Result<Self> {
            let bytes: &[u8] = decoder.bytes(4u8)?;
            Ok(bytes.try_into().unwrap())
        }
    }

    // The types [u8; 2], [u8; 3], [u8; 4] stand in here for any types for the fields
    // of a struct that are Decodable + Encodable. This means they can decode to/encode from
    // a byte slice, but also that thye can declare their encoded length.
    //
    // The goal then is to tag the struct definition for a proc-macro that implements
    // nested SIMPLE-TLV objects (as this is what we need in PIV return values)

    // tag 0xAA
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct S {
        // tag 0x11
        x: [u8; 2],
        // tag 0x22
        y: [u8; 3],
        // tag 0x33
        z: [u8; 4],
    }

    // this is what needs to be done to get `Decodable`
    impl<'a> TryFrom<TaggedSlice<'a>> for S {
        type Error = Error;

        fn try_from(tagged_slice: TaggedSlice<'a>) -> Result<S> {
            tagged_slice.tag().assert_eq(Tag::try_from(0xAA).unwrap())?;
            tagged_slice.decode_nested(|decoder| {
                let x = decoder.decode_tag(Tag::try_from(0x11).unwrap())?;
                let y = decoder.decode_tag(Tag::try_from(0x22).unwrap())?;
                let z = decoder.decode_tag(Tag::try_from(0x33).unwrap())?;

                Ok(Self { x, y, z })
            })
        }
    }

    // this is what needs to be done to get `Encodable`
    impl<'a> Message<'a> for S {
        fn tag() -> Tag {
            Tag::try_from(0xAA).unwrap()
        }

        fn fields<F, T>(&self, field_encoder: F) -> Result<T>
        where
            F: FnOnce(&[&dyn Encodable]) -> Result<T>,
        {
            field_encoder(&[
                &self.x.tagged(Tag::try_from(0x11).unwrap()),
                &self.y.tagged(Tag::try_from(0x22).unwrap()),
                &self.z.tagged(Tag::try_from(0x33).unwrap()),

            ])
        }
    }

    #[test]
    fn reconstruct() {
        let s = S { x: [1,2], y: [3,4,5], z: [6,7,8,9] };
        let mut buf = [0u8; 1024];

        let encoded = s.encode_to_slice(&mut buf).unwrap();

        assert_eq!(encoded,
            &[0xAA, 15,
                0x11, 2, 1, 2,
                0x22, 3, 3, 4, 5,
                0x33, 4, 6, 7, 8, 9,
            ],
        );

        let s2 = S::from_bytes(encoded).unwrap();

        assert_eq!(s, s2);
    }

    // tag 0xBB
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct T {
        // tag 0x01
        s: S,
        // tag 0x02
        t: [u8; 3],
    }

    impl<'a> TryFrom<TaggedSlice<'a>> for T {
        type Error = Error;

        fn try_from(tagged_slice: TaggedSlice<'a>) -> Result<Self> {
            tagged_slice.tag().assert_eq(Tag::try_from(0xBB).unwrap())?;
            tagged_slice.decode_nested(|decoder| {
                let s = decoder.decode_tag(Tag::try_from(0x01).unwrap())?;
                let t = decoder.decode_tag(Tag::try_from(0x02).unwrap())?;

                Ok(Self { s, t })
            })
        }
    }

    impl<'a> Message<'a> for T {
        fn tag() -> Tag {
            Tag::try_from(0xBB).unwrap()
        }

        fn fields<F, Z>(&self, field_encoder: F) -> Result<Z>
        where
            F: FnOnce(&[&dyn Encodable]) -> Result<Z>,
        {
            field_encoder(&[
                &self.s.tagged(Tag::try_from(0x1).unwrap()),
                &self.t.tagged(Tag::try_from(0x2).unwrap()),
            ])
        }
    }


    #[test]
    fn nesty() {
        let s = S { x: [1,2], y: [3,4,5], z: [6,7,8,9] };
        let t = T { s, t: [0xA, 0xB, 0xC] };

        let mut buf = [0u8; 1024];

        let encoded = t.encode_to_slice(&mut buf).unwrap();

        assert_eq!(encoded,
            &[0xBB, 24,
                0x1, 17,
                    0xAA, 15,
                        0x11, 2, 1, 2,
                        0x22, 3, 3, 4, 5,
                        0x33, 4, 6, 7, 8, 9,
                0x2, 3,
                   0xA, 0xB, 0xC
            ],
        );

        let t2 = T::from_bytes(encoded).unwrap();

        assert_eq!(t, t2);
    }

    // tag 0xCC
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct T2 {
        // no tag
        s: S,
        // tag 0x02
        t: [u8; 3],
    }

    impl<'a> TryFrom<TaggedSlice<'a>> for T2 {
        type Error = Error;

        fn try_from(tagged_slice: TaggedSlice<'a>) -> Result<Self> {
            tagged_slice.tag().assert_eq(Tag::try_from(0xCC).unwrap())?;
            tagged_slice.decode_nested(|decoder| {
                let s = decoder.decode()?;
                let t = decoder.decode_tag(Tag::try_from(0x02).unwrap())?;

                Ok(Self { s, t })
            })
        }
    }

    impl<'a> Message<'a> for T2 {
        fn tag() -> Tag {
            Tag::try_from(0xCC).unwrap()
        }

        fn fields<F, Z>(&self, field_encoder: F) -> Result<Z>
        where
            F: FnOnce(&[&dyn Encodable]) -> Result<Z>,
        {
            field_encoder(&[
                &self.s,
                &self.t.tagged(Tag::try_from(0x2).unwrap()),
            ])
        }
    }


    #[test]
    fn nesty2() {
        let s = S { x: [1,2], y: [3,4,5], z: [6,7,8,9] };
        let t = T2 { s, t: [0xA, 0xB, 0xC] };

        let mut buf = [0u8; 1024];

        let encoded = t.encode_to_slice(&mut buf).unwrap();

        assert_eq!(encoded,
            // &[0xBB, 24,
            &[0xCC, 22,
                // 0x1, 17,
                    0xAA, 15,
                        0x11, 2, 1, 2,
                        0x22, 3, 3, 4, 5,
                        0x33, 4, 6, 7, 8, 9,
                0x2, 3,
                   0xA, 0xB, 0xC
            ],
        );

        let t2 = T2::from_bytes(encoded).unwrap();

        assert_eq!(t, t2);
    }
}