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

//! Generated file from `PJDIBBHNNIO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PJDIBBHNNIO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PJDIBBHNNIO {
    // message fields
    // @@protoc_insertion_point(field:PJDIBBHNNIO.IBGJNDJHDFJ)
    pub IBGJNDJHDFJ: u32,
    // @@protoc_insertion_point(field:PJDIBBHNNIO.OJBAILGKLBM)
    pub OJBAILGKLBM: ::protobuf::MessageField<super::CGHFMJBOKGK::CGHFMJBOKGK>,
    // @@protoc_insertion_point(field:PJDIBBHNNIO.KKIKGPPKHIP)
    pub KKIKGPPKHIP: ::std::vec::Vec<super::KMADMFOAJDL::KMADMFOAJDL>,
    // special fields
    // @@protoc_insertion_point(special_field:PJDIBBHNNIO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PJDIBBHNNIO {
    fn default() -> &'a PJDIBBHNNIO {
        <PJDIBBHNNIO as ::protobuf::Message>::default_instance()
    }
}

impl PJDIBBHNNIO {
    pub fn new() -> PJDIBBHNNIO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IBGJNDJHDFJ",
            |m: &PJDIBBHNNIO| { &m.IBGJNDJHDFJ },
            |m: &mut PJDIBBHNNIO| { &mut m.IBGJNDJHDFJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CGHFMJBOKGK::CGHFMJBOKGK>(
            "OJBAILGKLBM",
            |m: &PJDIBBHNNIO| { &m.OJBAILGKLBM },
            |m: &mut PJDIBBHNNIO| { &mut m.OJBAILGKLBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KKIKGPPKHIP",
            |m: &PJDIBBHNNIO| { &m.KKIKGPPKHIP },
            |m: &mut PJDIBBHNNIO| { &mut m.KKIKGPPKHIP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PJDIBBHNNIO>(
            "PJDIBBHNNIO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PJDIBBHNNIO {
    const NAME: &'static str = "PJDIBBHNNIO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.IBGJNDJHDFJ = is.read_uint32()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OJBAILGKLBM)?;
                },
                26 => {
                    self.KKIKGPPKHIP.push(is.read_message()?);
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
        if self.IBGJNDJHDFJ != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.IBGJNDJHDFJ);
        }
        if let Some(v) = self.OJBAILGKLBM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.KKIKGPPKHIP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IBGJNDJHDFJ != 0 {
            os.write_uint32(1, self.IBGJNDJHDFJ)?;
        }
        if let Some(v) = self.OJBAILGKLBM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        for v in &self.KKIKGPPKHIP {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> PJDIBBHNNIO {
        PJDIBBHNNIO::new()
    }

    fn clear(&mut self) {
        self.IBGJNDJHDFJ = 0;
        self.OJBAILGKLBM.clear();
        self.KKIKGPPKHIP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PJDIBBHNNIO {
        static instance: PJDIBBHNNIO = PJDIBBHNNIO {
            IBGJNDJHDFJ: 0,
            OJBAILGKLBM: ::protobuf::MessageField::none(),
            KKIKGPPKHIP: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PJDIBBHNNIO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PJDIBBHNNIO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PJDIBBHNNIO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PJDIBBHNNIO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PJDIBBHNNIO.proto\x1a\x11CGHFMJBOKGK.proto\x1a\x11KMADMFOAJDL.prot\
    o\"\x8f\x01\n\x0bPJDIBBHNNIO\x12\x20\n\x0bIBGJNDJHDFJ\x18\x01\x20\x01(\r\
    R\x0bIBGJNDJHDFJ\x12.\n\x0bOJBAILGKLBM\x18\x02\x20\x01(\x0b2\x0c.CGHFMJB\
    OKGKR\x0bOJBAILGKLBM\x12.\n\x0bKKIKGPPKHIP\x18\x03\x20\x03(\x0b2\x0c.KMA\
    DMFOAJDLR\x0bKKIKGPPKHIPb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::CGHFMJBOKGK::file_descriptor().clone());
            deps.push(super::KMADMFOAJDL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PJDIBBHNNIO::generated_message_descriptor_data());
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