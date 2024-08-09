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

//! Generated file from `EnhanceRogueBuffScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EnhanceRogueBuffScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EnhanceRogueBuffScRsp {
    // message fields
    // @@protoc_insertion_point(field:EnhanceRogueBuffScRsp.rogue_buff)
    pub rogue_buff: ::protobuf::MessageField<super::RogueBuff::RogueBuff>,
    // @@protoc_insertion_point(field:EnhanceRogueBuffScRsp.is_success)
    pub is_success: bool,
    // @@protoc_insertion_point(field:EnhanceRogueBuffScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EnhanceRogueBuffScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnhanceRogueBuffScRsp {
    fn default() -> &'a EnhanceRogueBuffScRsp {
        <EnhanceRogueBuffScRsp as ::protobuf::Message>::default_instance()
    }
}

impl EnhanceRogueBuffScRsp {
    pub fn new() -> EnhanceRogueBuffScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueBuff::RogueBuff>(
            "rogue_buff",
            |m: &EnhanceRogueBuffScRsp| { &m.rogue_buff },
            |m: &mut EnhanceRogueBuffScRsp| { &mut m.rogue_buff },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_success",
            |m: &EnhanceRogueBuffScRsp| { &m.is_success },
            |m: &mut EnhanceRogueBuffScRsp| { &mut m.is_success },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &EnhanceRogueBuffScRsp| { &m.retcode },
            |m: &mut EnhanceRogueBuffScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnhanceRogueBuffScRsp>(
            "EnhanceRogueBuffScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnhanceRogueBuffScRsp {
    const NAME: &'static str = "EnhanceRogueBuffScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_buff)?;
                },
                24 => {
                    self.is_success = is.read_bool()?;
                },
                64 => {
                    self.retcode = is.read_uint32()?;
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
        if let Some(v) = self.rogue_buff.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.is_success != false {
            my_size += 1 + 1;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.rogue_buff.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.is_success != false {
            os.write_bool(3, self.is_success)?;
        }
        if self.retcode != 0 {
            os.write_uint32(8, self.retcode)?;
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

    fn new() -> EnhanceRogueBuffScRsp {
        EnhanceRogueBuffScRsp::new()
    }

    fn clear(&mut self) {
        self.rogue_buff.clear();
        self.is_success = false;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnhanceRogueBuffScRsp {
        static instance: EnhanceRogueBuffScRsp = EnhanceRogueBuffScRsp {
            rogue_buff: ::protobuf::MessageField::none(),
            is_success: false,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnhanceRogueBuffScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnhanceRogueBuffScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnhanceRogueBuffScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnhanceRogueBuffScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bEnhanceRogueBuffScRsp.proto\x1a\x0fRogueBuff.proto\"{\n\x15Enhance\
    RogueBuffScRsp\x12)\n\nrogue_buff\x18\x0b\x20\x01(\x0b2\n.RogueBuffR\tro\
    gueBuff\x12\x1d\n\nis_success\x18\x03\x20\x01(\x08R\tisSuccess\x12\x18\n\
    \x07retcode\x18\x08\x20\x01(\rR\x07retcodeB\x15\n\x13emu.lunarcore.proto\
    b\x06proto3\
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
            deps.push(super::RogueBuff::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EnhanceRogueBuffScRsp::generated_message_descriptor_data());
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
