// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `CNNDEHKLFFF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CNNDEHKLFFF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CNNDEHKLFFF {
    // message fields
    // @@protoc_insertion_point(field:CNNDEHKLFFF.LGOAHPCLIFI)
    pub LGOAHPCLIFI: u32,
    // @@protoc_insertion_point(field:CNNDEHKLFFF.BOIEEFIPNEG)
    pub BOIEEFIPNEG: u32,
    // @@protoc_insertion_point(field:CNNDEHKLFFF.EABJLPEGNML)
    pub EABJLPEGNML: u32,
    // @@protoc_insertion_point(field:CNNDEHKLFFF.APEDOJKNIMO)
    pub APEDOJKNIMO: ::protobuf::MessageField<super::FDMIHCJCAEM::FDMIHCJCAEM>,
    // @@protoc_insertion_point(field:CNNDEHKLFFF.CJLFMAIJMMB)
    pub CJLFMAIJMMB: ::std::vec::Vec<super::FDMIHCJCAEM::FDMIHCJCAEM>,
    // special fields
    // @@protoc_insertion_point(special_field:CNNDEHKLFFF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CNNDEHKLFFF {
    fn default() -> &'a CNNDEHKLFFF {
        <CNNDEHKLFFF as ::protobuf::Message>::default_instance()
    }
}

impl CNNDEHKLFFF {
    pub fn new() -> CNNDEHKLFFF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LGOAHPCLIFI",
            |m: &CNNDEHKLFFF| { &m.LGOAHPCLIFI },
            |m: &mut CNNDEHKLFFF| { &mut m.LGOAHPCLIFI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BOIEEFIPNEG",
            |m: &CNNDEHKLFFF| { &m.BOIEEFIPNEG },
            |m: &mut CNNDEHKLFFF| { &mut m.BOIEEFIPNEG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EABJLPEGNML",
            |m: &CNNDEHKLFFF| { &m.EABJLPEGNML },
            |m: &mut CNNDEHKLFFF| { &mut m.EABJLPEGNML },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FDMIHCJCAEM::FDMIHCJCAEM>(
            "APEDOJKNIMO",
            |m: &CNNDEHKLFFF| { &m.APEDOJKNIMO },
            |m: &mut CNNDEHKLFFF| { &mut m.APEDOJKNIMO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CJLFMAIJMMB",
            |m: &CNNDEHKLFFF| { &m.CJLFMAIJMMB },
            |m: &mut CNNDEHKLFFF| { &mut m.CJLFMAIJMMB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CNNDEHKLFFF>(
            "CNNDEHKLFFF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CNNDEHKLFFF {
    const NAME: &'static str = "CNNDEHKLFFF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.LGOAHPCLIFI = is.read_uint32()?;
                },
                120 => {
                    self.BOIEEFIPNEG = is.read_uint32()?;
                },
                32 => {
                    self.EABJLPEGNML = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.APEDOJKNIMO)?;
                },
                18 => {
                    self.CJLFMAIJMMB.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.LGOAHPCLIFI != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.LGOAHPCLIFI);
        }
        if self.BOIEEFIPNEG != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.BOIEEFIPNEG);
        }
        if self.EABJLPEGNML != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.EABJLPEGNML);
        }
        if let Some(v) = self.APEDOJKNIMO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.CJLFMAIJMMB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LGOAHPCLIFI != 0 {
            os.write_uint32(9, self.LGOAHPCLIFI)?;
        }
        if self.BOIEEFIPNEG != 0 {
            os.write_uint32(15, self.BOIEEFIPNEG)?;
        }
        if self.EABJLPEGNML != 0 {
            os.write_uint32(4, self.EABJLPEGNML)?;
        }
        if let Some(v) = self.APEDOJKNIMO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        for v in &self.CJLFMAIJMMB {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CNNDEHKLFFF {
        CNNDEHKLFFF::new()
    }

    fn clear(&mut self) {
        self.LGOAHPCLIFI = 0;
        self.BOIEEFIPNEG = 0;
        self.EABJLPEGNML = 0;
        self.APEDOJKNIMO.clear();
        self.CJLFMAIJMMB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CNNDEHKLFFF {
        static instance: CNNDEHKLFFF = CNNDEHKLFFF {
            LGOAHPCLIFI: 0,
            BOIEEFIPNEG: 0,
            EABJLPEGNML: 0,
            APEDOJKNIMO: ::protobuf::MessageField::none(),
            CJLFMAIJMMB: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CNNDEHKLFFF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CNNDEHKLFFF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CNNDEHKLFFF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNNDEHKLFFF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CNNDEHKLFFF.proto\x1a\x11FDMIHCJCAEM.proto\"\xd3\x01\n\x0bCNNDEHKL\
    FFF\x12\x20\n\x0bLGOAHPCLIFI\x18\t\x20\x01(\rR\x0bLGOAHPCLIFI\x12\x20\n\
    \x0bBOIEEFIPNEG\x18\x0f\x20\x01(\rR\x0bBOIEEFIPNEG\x12\x20\n\x0bEABJLPEG\
    NML\x18\x04\x20\x01(\rR\x0bEABJLPEGNML\x12.\n\x0bAPEDOJKNIMO\x18\x07\x20\
    \x01(\x0b2\x0c.FDMIHCJCAEMR\x0bAPEDOJKNIMO\x12.\n\x0bCJLFMAIJMMB\x18\x02\
    \x20\x03(\x0b2\x0c.FDMIHCJCAEMR\x0bCJLFMAIJMMBb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::FDMIHCJCAEM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CNNDEHKLFFF::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}