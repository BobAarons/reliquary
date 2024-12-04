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

//! Generated file from `JNJKMBIGMPE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:JNJKMBIGMPE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JNJKMBIGMPE {
    // message fields
    // @@protoc_insertion_point(field:JNJKMBIGMPE.PNIDOBOEIEP)
    pub PNIDOBOEIEP: ::std::vec::Vec<super::MEAEGFBOGAK::MEAEGFBOGAK>,
    // @@protoc_insertion_point(field:JNJKMBIGMPE.KIPKPPEDOJP)
    pub KIPKPPEDOJP: u32,
    // @@protoc_insertion_point(field:JNJKMBIGMPE.JIBCJFPFHKO)
    pub JIBCJFPFHKO: ::std::vec::Vec<super::DBDLFHLLJLM::DBDLFHLLJLM>,
    // special fields
    // @@protoc_insertion_point(special_field:JNJKMBIGMPE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JNJKMBIGMPE {
    fn default() -> &'a JNJKMBIGMPE {
        <JNJKMBIGMPE as ::protobuf::Message>::default_instance()
    }
}

impl JNJKMBIGMPE {
    pub fn new() -> JNJKMBIGMPE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PNIDOBOEIEP",
            |m: &JNJKMBIGMPE| { &m.PNIDOBOEIEP },
            |m: &mut JNJKMBIGMPE| { &mut m.PNIDOBOEIEP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KIPKPPEDOJP",
            |m: &JNJKMBIGMPE| { &m.KIPKPPEDOJP },
            |m: &mut JNJKMBIGMPE| { &mut m.KIPKPPEDOJP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JIBCJFPFHKO",
            |m: &JNJKMBIGMPE| { &m.JIBCJFPFHKO },
            |m: &mut JNJKMBIGMPE| { &mut m.JIBCJFPFHKO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JNJKMBIGMPE>(
            "JNJKMBIGMPE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JNJKMBIGMPE {
    const NAME: &'static str = "JNJKMBIGMPE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    self.PNIDOBOEIEP.push(is.read_message()?);
                },
                120 => {
                    self.KIPKPPEDOJP = is.read_uint32()?;
                },
                66 => {
                    self.JIBCJFPFHKO.push(is.read_message()?);
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
        for value in &self.PNIDOBOEIEP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.KIPKPPEDOJP != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.KIPKPPEDOJP);
        }
        for value in &self.JIBCJFPFHKO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.PNIDOBOEIEP {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if self.KIPKPPEDOJP != 0 {
            os.write_uint32(15, self.KIPKPPEDOJP)?;
        }
        for v in &self.JIBCJFPFHKO {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
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

    fn new() -> JNJKMBIGMPE {
        JNJKMBIGMPE::new()
    }

    fn clear(&mut self) {
        self.PNIDOBOEIEP.clear();
        self.KIPKPPEDOJP = 0;
        self.JIBCJFPFHKO.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JNJKMBIGMPE {
        static instance: JNJKMBIGMPE = JNJKMBIGMPE {
            PNIDOBOEIEP: ::std::vec::Vec::new(),
            KIPKPPEDOJP: 0,
            JIBCJFPFHKO: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JNJKMBIGMPE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JNJKMBIGMPE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JNJKMBIGMPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JNJKMBIGMPE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JNJKMBIGMPE.proto\x1a\x11DBDLFHLLJLM.proto\x1a\x11MEAEGFBOGAK.prot\
    o\"\x8f\x01\n\x0bJNJKMBIGMPE\x12.\n\x0bPNIDOBOEIEP\x18\n\x20\x03(\x0b2\
    \x0c.MEAEGFBOGAKR\x0bPNIDOBOEIEP\x12\x20\n\x0bKIPKPPEDOJP\x18\x0f\x20\
    \x01(\rR\x0bKIPKPPEDOJP\x12.\n\x0bJIBCJFPFHKO\x18\x08\x20\x03(\x0b2\x0c.\
    DBDLFHLLJLMR\x0bJIBCJFPFHKOb\x06proto3\
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
            deps.push(super::DBDLFHLLJLM::file_descriptor().clone());
            deps.push(super::MEAEGFBOGAK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JNJKMBIGMPE::generated_message_descriptor_data());
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