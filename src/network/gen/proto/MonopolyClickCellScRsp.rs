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

//! Generated file from `MonopolyClickCellScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MonopolyClickCellScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MonopolyClickCellScRsp {
    // message fields
    // @@protoc_insertion_point(field:MonopolyClickCellScRsp.NAIMNIFDKFJ)
    pub NAIMNIFDKFJ: u32,
    // @@protoc_insertion_point(field:MonopolyClickCellScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:MonopolyClickCellScRsp.LNGJKFGLHBE)
    pub LNGJKFGLHBE: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MonopolyClickCellScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MonopolyClickCellScRsp {
    fn default() -> &'a MonopolyClickCellScRsp {
        <MonopolyClickCellScRsp as ::protobuf::Message>::default_instance()
    }
}

impl MonopolyClickCellScRsp {
    pub fn new() -> MonopolyClickCellScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NAIMNIFDKFJ",
            |m: &MonopolyClickCellScRsp| { &m.NAIMNIFDKFJ },
            |m: &mut MonopolyClickCellScRsp| { &mut m.NAIMNIFDKFJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &MonopolyClickCellScRsp| { &m.ADADHIHDHJC },
            |m: &mut MonopolyClickCellScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LNGJKFGLHBE",
            |m: &MonopolyClickCellScRsp| { &m.LNGJKFGLHBE },
            |m: &mut MonopolyClickCellScRsp| { &mut m.LNGJKFGLHBE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MonopolyClickCellScRsp>(
            "MonopolyClickCellScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MonopolyClickCellScRsp {
    const NAME: &'static str = "MonopolyClickCellScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.NAIMNIFDKFJ = is.read_uint32()?;
                },
                120 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                88 => {
                    self.LNGJKFGLHBE = is.read_uint32()?;
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
        if self.NAIMNIFDKFJ != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.NAIMNIFDKFJ);
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.ADADHIHDHJC);
        }
        if self.LNGJKFGLHBE != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.LNGJKFGLHBE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NAIMNIFDKFJ != 0 {
            os.write_uint32(14, self.NAIMNIFDKFJ)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(15, self.ADADHIHDHJC)?;
        }
        if self.LNGJKFGLHBE != 0 {
            os.write_uint32(11, self.LNGJKFGLHBE)?;
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

    fn new() -> MonopolyClickCellScRsp {
        MonopolyClickCellScRsp::new()
    }

    fn clear(&mut self) {
        self.NAIMNIFDKFJ = 0;
        self.ADADHIHDHJC = 0;
        self.LNGJKFGLHBE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MonopolyClickCellScRsp {
        static instance: MonopolyClickCellScRsp = MonopolyClickCellScRsp {
            NAIMNIFDKFJ: 0,
            ADADHIHDHJC: 0,
            LNGJKFGLHBE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MonopolyClickCellScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MonopolyClickCellScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MonopolyClickCellScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MonopolyClickCellScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cMonopolyClickCellScRsp.proto\"~\n\x16MonopolyClickCellScRsp\x12\
    \x20\n\x0bNAIMNIFDKFJ\x18\x0e\x20\x01(\rR\x0bNAIMNIFDKFJ\x12\x20\n\x0bAD\
    ADHIHDHJC\x18\x0f\x20\x01(\rR\x0bADADHIHDHJC\x12\x20\n\x0bLNGJKFGLHBE\
    \x18\x0b\x20\x01(\rR\x0bLNGJKFGLHBEb\x06proto3\
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
            messages.push(MonopolyClickCellScRsp::generated_message_descriptor_data());
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