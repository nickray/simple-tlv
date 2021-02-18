var searchIndex = JSON.parse('{\
"simple_tlv":{"doc":"simple-tlvImplementation of the SIMPLE-TLV serialization …","i":[[3,"Decoder","simple_tlv","SIMPLE-TLV decoder.",null,null],[3,"Encoder","","SIMPLE-TLV encoder.",null,null],[3,"Error","","Error type.",null,null],[4,"ErrorKind","","Error type.",null,null],[13,"Failed","","Operation failed due to previous error",0,null],[13,"InvalidTag","","Invalid tag",0,null],[12,"byte","simple_tlv::ErrorKind","Raw byte value of the tag",1,null],[13,"InvalidLength","simple_tlv","Length greater than u16::MAX",0,null],[13,"Length","","Incorrect length for a given field",0,null],[12,"tag","simple_tlv::ErrorKind","Tag type of the value being decoded",2,null],[13,"Overflow","simple_tlv","Integer overflow occurred (library bug!)",0,null],[13,"Overlength","","Message is longer than SIMPLE-TLV\'s limits support",0,null],[13,"TrailingData","","Undecoded trailing data at end of message",0,null],[12,"decoded","simple_tlv::ErrorKind","Length of the decoded data",3,null],[12,"remaining","","Total length of the remaining data left in the buffer",3,null],[13,"Truncated","simple_tlv","Unexpected end-of-message/nested field when decoding",0,null],[13,"Underlength","","Encoded message is shorter than the expected length (i.e. …",0,null],[12,"expected","simple_tlv::ErrorKind","Expected length",4,null],[12,"actual","","Actual length",4,null],[13,"UnexpectedTag","simple_tlv","Unexpected tag",0,null],[12,"expected","simple_tlv::ErrorKind","Tag the decoder was expecting (if there is a single such …",5,null],[12,"actual","","Actual tag encountered in the message",5,null],[6,"Result","simple_tlv","Result type.",null,null],[3,"Length","","SIMPLE-TLV-encoded length.",null,null],[3,"Slice","","Slice of at most <code>Length::max()</code> bytes.",null,null],[3,"Tag","","The tag field consists of a single byte encoding a tag …",null,null],[6,"TaggedSlice","","Raw SIMPLE-TLV data object <code>TaggedValue<Slice<\'_>></code>.",null,null],[3,"TaggedValue","","SIMPLE-TLV data object.",null,null],[8,"Container","","Multiple encodables in a container.",null,null],[10,"fields","","Call the provided function with a slice of [<code>Encodable</code>] …",6,[[],["result",6]]],[8,"Decodable","","Decoding trait.",null,null],[10,"decode","","Attempt to decode this message using the provided decoder.",7,[[["decoder",3]],["result",6]]],[11,"from_bytes","","Parse <code>Self</code> from the provided byte slice.",7,[[],["result",6]]],[8,"Encodable","","Encoding trait.",null,null],[10,"encoded_length","","Compute the length of this value in bytes when encoded as …",8,[[],[["result",6],["length",3]]]],[10,"encode","","Encode this value as SIMPLE-TLV using the provided […",8,[[["encoder",3]],["result",6]]],[11,"encode_to_slice","","Encode this value to the provided byte slice, returning a …",8,[[],["result",6]]],[8,"Tagged","","Types with an associated SIMPLE-TLV [<code>Tag</code>].",null,null],[10,"tag","","The tag",9,[[],["tag",3]]],[11,"from","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"into","","",10,[[]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"into","","",11,[[]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"from","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"into","","",12,[[]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"from","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"into","","",0,[[]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"into","","",13,[[]]],[11,"try_into","","",13,[[],["result",4]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"from","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"into","","",14,[[]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"from","","",15,[[]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"into","","",15,[[]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"from","","",16,[[]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"into","","",16,[[]]],[11,"try_into","","",16,[[],["result",4]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"decode","","",13,[[["decoder",3]],[["result",6],["length",3]]]],[11,"decode","","",15,[[["decoder",3]],["result",6]]],[11,"decode","","",17,[[["decoder",3]],[["result",6],["taggedslice",6]]]],[11,"encoded_length","","",13,[[],[["result",6],["length",3]]]],[11,"encode","","",13,[[["encoder",3]],["result",6]]],[11,"encoded_length","","",15,[[],[["result",6],["length",3]]]],[11,"encode","","",15,[[["encoder",3]],["result",6]]],[11,"encoded_length","","",16,[[],[["result",6],["length",3]]]],[11,"encode","","",16,[[["encoder",3]],["result",6]]],[11,"encoded_length","","",17,[[],[["result",6],["length",3]]]],[11,"encode","","",17,[[["encoder",3]],["result",6]]],[11,"from","","",10,[[],["decoder",3]]],[11,"from","","",12,[[["errorkind",4]],["error",3]]],[11,"from","","",12,[[["infallible",4]],["error",3]]],[11,"from","","",13,[[],["length",3]]],[11,"from","","",13,[[],["length",3]]],[11,"fmt","","",10,[[["formatter",3]],["result",6]]],[11,"fmt","","",11,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",13,[[["formatter",3]],["result",6]]],[11,"fmt","","",14,[[["formatter",3]],["result",6]]],[11,"fmt","","",15,[[["formatter",3]],["result",6]]],[11,"fmt","","",16,[[["formatter",3]],["result",6]]],[11,"fmt","","",12,[[["formatter",3]],["result",6]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",13,[[["formatter",3]],["result",6]]],[11,"fmt","","",15,[[["formatter",3]],["result",6]]],[11,"eq","","",12,[[["error",3]]]],[11,"ne","","",12,[[["error",3]]]],[11,"eq","","",0,[[["errorkind",4]]]],[11,"ne","","",0,[[["errorkind",4]]]],[11,"eq","","",13,[[["length",3]]]],[11,"ne","","",13,[[["length",3]]]],[11,"eq","","",14,[[["slice",3]]]],[11,"ne","","",14,[[["slice",3]]]],[11,"eq","","",15,[[["tag",3]]]],[11,"ne","","",15,[[["tag",3]]]],[11,"eq","","",16,[[["taggedvalue",3]]]],[11,"ne","","",16,[[["taggedvalue",3]]]],[11,"cmp","","",13,[[["length",3]],["ordering",4]]],[11,"partial_cmp","","",13,[[["length",3]],[["option",4],["ordering",4]]]],[11,"lt","","",13,[[["length",3]]]],[11,"le","","",13,[[["length",3]]]],[11,"gt","","",13,[[["length",3]]]],[11,"ge","","",13,[[["length",3]]]],[11,"add","","",13,[[],["result",6]]],[11,"add","","",13,[[],["result",6]]],[11,"add","","",13,[[],["result",6]]],[11,"add","","",13,[[],["result",6]]],[11,"add","","",18,[[["length",3]]]],[11,"try_from","","",13,[[],[["result",6],["length",3]]]],[11,"try_from","","",13,[[],[["result",6],["length",3]]]],[11,"try_from","","",15,[[],["result",6]]],[11,"as_ref","","",14,[[]]],[11,"clone","","",12,[[],["error",3]]],[11,"clone","","",0,[[],["errorkind",4]]],[11,"clone","","",13,[[],["length",3]]],[11,"clone","","",14,[[],["slice",3]]],[11,"clone","","",15,[[],["tag",3]]],[11,"clone","","",16,[[],["taggedvalue",3]]],[11,"default","","",13,[[],["length",3]]],[11,"new","","Create a new decoder for the given byte slice.",10,[[]]],[11,"decode","","Decode a value which impls the [<code>Decodable</code>] trait.",10,[[],[["result",6],["decodable",8]]]],[11,"decode_tagged_value","","Decode a TaggedValue with tag checked to be as expected, …",10,[[["tag",3]],[["decodable",8],["result",6]]]],[11,"decode_tagged_slice","","Decode a TaggedSlice with tag checked to be as expected, …",10,[[["tag",3]],["result",6]]],[11,"error","","Return an error with the given [<code>ErrorKind</code>], annotating it …",10,[[["errorkind",4]],["result",6]]],[11,"is_failed","","Did the decoding operation fail due to an error?",10,[[]]],[11,"finish","","Finish decoding, returning the given value if there is no …",10,[[],["result",6]]],[11,"is_finished","","Have we decoded all of the bytes in this [<code>Decoder</code>]?",10,[[]]],[11,"new","","Create a new encoder with the given byte slice as a …",11,[[]]],[11,"encode","","Encode a value which impls the [<code>Encodable</code>] trait.",11,[[],["result",6]]],[11,"error","","Return an error with the given [<code>ErrorKind</code>], annotating it …",11,[[["errorkind",4]],["result",6]]],[11,"is_failed","","Did the decoding operation fail due to an error?",11,[[]]],[11,"finish","","Finish encoding to the buffer, returning a slice …",11,[[],["result",6]]],[11,"encode_tagged_collection","","Encode a collection of values which impl the [<code>Encodable</code>] …",11,[[["tag",3]],["result",6]]],[11,"new","","Create a new [<code>Error</code>]",12,[[["errorkind",4],["length",3]],["error",3]]],[11,"kind","","Get the [<code>ErrorKind</code>] which occurred.",12,[[],["errorkind",4]]],[11,"position","","Get the position inside of the message where the error …",12,[[],[["option",4],["length",3]]]],[11,"nested","","For errors occurring inside of a nested message, extend …",12,[[["length",3]]]],[11,"at","","Annotate an [<code>ErrorKind</code>] with context about where it …",0,[[["length",3]],["error",3]]],[11,"zero","","Return a length of <code>0</code>.",13,[[]]],[11,"max","","Get the maximum length supported by SIMPLE-TLV: 65,535.",13,[[]]],[11,"to_usize","","Convert length to <code>usize</code>.",13,[[]]],[11,"new","","Create a new [<code>Slice</code>], ensuring that the provided <code>slice</code> …",14,[[],["result",6]]],[11,"as_bytes","","Borrow the inner byte slice",14,[[]]],[11,"length","","Get the [<code>Length</code>] of this [<code>Slice</code>]",14,[[],["length",3]]],[11,"is_empty","","Is this [<code>Slice</code>] empty?",14,[[]]],[11,"assert_eq","","Assert that this [<code>Tag</code>] matches the provided expected tag.",15,[[["tag",3]],[["result",6],["tag",3]]]],[11,"with_value","","",15,[[],["taggedvalue",3]]],[11,"new","","",16,[[["tag",3]]]],[11,"tag","","",16,[[],["tag",3]]],[11,"from","","Create a new tagged slice, checking lengths.",16,[[["tag",3]],["result",6]]],[11,"as_bytes","","Borrow the inner byte slice.",16,[[]]],[11,"length","","Get the length of the inner byte slice.",16,[[],["length",3]]],[11,"is_empty","","Is the inner byte slice empty?",16,[[]]],[11,"decode_nested","","Decode nested values, creating a new [<code>Decoder</code>] for the …",16,[[],["result",6]]],[11,"from","","Create a new tagged slice, checking lengths.",17,[[["tag",3]],["result",6]]],[11,"as_bytes","","Borrow the inner byte slice.",17,[[]]],[11,"length","","Get the length of the inner byte slice.",17,[[],["length",3]]],[11,"is_empty","","Is the inner byte slice empty?",17,[[]]],[11,"decode_nested","","Decode nested values, creating a new [<code>Decoder</code>] for the …",17,[[],["result",6]]],[11,"from_bytes","","Parse <code>Self</code> from the provided byte slice.",7,[[],["result",6]]],[11,"encode_to_slice","","Encode this value to the provided byte slice, returning a …",8,[[],["result",6]]]],"p":[[4,"ErrorKind"],[13,"InvalidTag"],[13,"Length"],[13,"TrailingData"],[13,"Underlength"],[13,"UnexpectedTag"],[8,"Container"],[8,"Decodable"],[8,"Encodable"],[8,"Tagged"],[3,"Decoder"],[3,"Encoder"],[3,"Error"],[3,"Length"],[3,"Slice"],[3,"Tag"],[3,"TaggedValue"],[6,"TaggedSlice"],[6,"Result"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);