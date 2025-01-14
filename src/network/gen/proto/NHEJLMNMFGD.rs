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

//! Generated file from `NHEJLMNMFGD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NHEJLMNMFGD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NHEJLMNMFGD {
    // message fields
    // @@protoc_insertion_point(field:NHEJLMNMFGD.GEDDCNNDIPL)
    pub GEDDCNNDIPL: u32,
    // @@protoc_insertion_point(field:NHEJLMNMFGD.ODOOPNFLDJO)
    pub ODOOPNFLDJO: u32,
    // @@protoc_insertion_point(field:NHEJLMNMFGD.KGPCPPPJPBF)
    pub KGPCPPPJPBF: u32,
    // @@protoc_insertion_point(field:NHEJLMNMFGD.BDGCMDCGGGJ)
    pub BDGCMDCGGGJ: u32,
    // special fields
    // @@protoc_insertion_point(special_field:NHEJLMNMFGD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NHEJLMNMFGD {
    fn default() -> &'a NHEJLMNMFGD {
        <NHEJLMNMFGD as ::protobuf::Message>::default_instance()
    }
}

impl NHEJLMNMFGD {
    pub fn new() -> NHEJLMNMFGD {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GEDDCNNDIPL",
            |m: &NHEJLMNMFGD| { &m.GEDDCNNDIPL },
            |m: &mut NHEJLMNMFGD| { &mut m.GEDDCNNDIPL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ODOOPNFLDJO",
            |m: &NHEJLMNMFGD| { &m.ODOOPNFLDJO },
            |m: &mut NHEJLMNMFGD| { &mut m.ODOOPNFLDJO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGPCPPPJPBF",
            |m: &NHEJLMNMFGD| { &m.KGPCPPPJPBF },
            |m: &mut NHEJLMNMFGD| { &mut m.KGPCPPPJPBF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BDGCMDCGGGJ",
            |m: &NHEJLMNMFGD| { &m.BDGCMDCGGGJ },
            |m: &mut NHEJLMNMFGD| { &mut m.BDGCMDCGGGJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NHEJLMNMFGD>(
            "NHEJLMNMFGD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NHEJLMNMFGD {
    const NAME: &'static str = "NHEJLMNMFGD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.GEDDCNNDIPL = is.read_uint32()?;
                },
                8 => {
                    self.ODOOPNFLDJO = is.read_uint32()?;
                },
                80 => {
                    self.KGPCPPPJPBF = is.read_uint32()?;
                },
                72 => {
                    self.BDGCMDCGGGJ = is.read_uint32()?;
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
        if self.GEDDCNNDIPL != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.GEDDCNNDIPL);
        }
        if self.ODOOPNFLDJO != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.ODOOPNFLDJO);
        }
        if self.KGPCPPPJPBF != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.KGPCPPPJPBF);
        }
        if self.BDGCMDCGGGJ != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.BDGCMDCGGGJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GEDDCNNDIPL != 0 {
            os.write_uint32(12, self.GEDDCNNDIPL)?;
        }
        if self.ODOOPNFLDJO != 0 {
            os.write_uint32(1, self.ODOOPNFLDJO)?;
        }
        if self.KGPCPPPJPBF != 0 {
            os.write_uint32(10, self.KGPCPPPJPBF)?;
        }
        if self.BDGCMDCGGGJ != 0 {
            os.write_uint32(9, self.BDGCMDCGGGJ)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> NHEJLMNMFGD {
        NHEJLMNMFGD::new()
    }

    fn clear(&mut self) {
        self.GEDDCNNDIPL = 0;
        self.ODOOPNFLDJO = 0;
        self.KGPCPPPJPBF = 0;
        self.BDGCMDCGGGJ = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NHEJLMNMFGD {
        static instance: NHEJLMNMFGD = NHEJLMNMFGD {
            GEDDCNNDIPL: 0,
            ODOOPNFLDJO: 0,
            KGPCPPPJPBF: 0,
            BDGCMDCGGGJ: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NHEJLMNMFGD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NHEJLMNMFGD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NHEJLMNMFGD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NHEJLMNMFGD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NHEJLMNMFGD.proto\"\x95\x01\n\x0bNHEJLMNMFGD\x12\x20\n\x0bGEDDCNND\
    IPL\x18\x0c\x20\x01(\rR\x0bGEDDCNNDIPL\x12\x20\n\x0bODOOPNFLDJO\x18\x01\
    \x20\x01(\rR\x0bODOOPNFLDJO\x12\x20\n\x0bKGPCPPPJPBF\x18\n\x20\x01(\rR\
    \x0bKGPCPPPJPBF\x12\x20\n\x0bBDGCMDCGGGJ\x18\t\x20\x01(\rR\x0bBDGCMDCGGG\
    Jb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(NHEJLMNMFGD::generated_message_descriptor_data());
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
