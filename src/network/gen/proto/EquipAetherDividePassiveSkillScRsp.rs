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

//! Generated file from `EquipAetherDividePassiveSkillScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EquipAetherDividePassiveSkillScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EquipAetherDividePassiveSkillScRsp {
    // message fields
    // @@protoc_insertion_point(field:EquipAetherDividePassiveSkillScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:EquipAetherDividePassiveSkillScRsp.HBOCMBOBJNG)
    pub HBOCMBOBJNG: ::protobuf::MessageField<super::HHBFGOMBIMA::HHBFGOMBIMA>,
    // @@protoc_insertion_point(field:EquipAetherDividePassiveSkillScRsp.AEABHIJBFCJ)
    pub AEABHIJBFCJ: ::protobuf::MessageField<super::ADDCJFIBCFF::ADDCJFIBCFF>,
    // special fields
    // @@protoc_insertion_point(special_field:EquipAetherDividePassiveSkillScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EquipAetherDividePassiveSkillScRsp {
    fn default() -> &'a EquipAetherDividePassiveSkillScRsp {
        <EquipAetherDividePassiveSkillScRsp as ::protobuf::Message>::default_instance()
    }
}

impl EquipAetherDividePassiveSkillScRsp {
    pub fn new() -> EquipAetherDividePassiveSkillScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &EquipAetherDividePassiveSkillScRsp| { &m.ADADHIHDHJC },
            |m: &mut EquipAetherDividePassiveSkillScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HHBFGOMBIMA::HHBFGOMBIMA>(
            "HBOCMBOBJNG",
            |m: &EquipAetherDividePassiveSkillScRsp| { &m.HBOCMBOBJNG },
            |m: &mut EquipAetherDividePassiveSkillScRsp| { &mut m.HBOCMBOBJNG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ADDCJFIBCFF::ADDCJFIBCFF>(
            "AEABHIJBFCJ",
            |m: &EquipAetherDividePassiveSkillScRsp| { &m.AEABHIJBFCJ },
            |m: &mut EquipAetherDividePassiveSkillScRsp| { &mut m.AEABHIJBFCJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EquipAetherDividePassiveSkillScRsp>(
            "EquipAetherDividePassiveSkillScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EquipAetherDividePassiveSkillScRsp {
    const NAME: &'static str = "EquipAetherDividePassiveSkillScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HBOCMBOBJNG)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.AEABHIJBFCJ)?;
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
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.ADADHIHDHJC);
        }
        if let Some(v) = self.HBOCMBOBJNG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.AEABHIJBFCJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(15, self.ADADHIHDHJC)?;
        }
        if let Some(v) = self.HBOCMBOBJNG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.AEABHIJBFCJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> EquipAetherDividePassiveSkillScRsp {
        EquipAetherDividePassiveSkillScRsp::new()
    }

    fn clear(&mut self) {
        self.ADADHIHDHJC = 0;
        self.HBOCMBOBJNG.clear();
        self.AEABHIJBFCJ.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EquipAetherDividePassiveSkillScRsp {
        static instance: EquipAetherDividePassiveSkillScRsp = EquipAetherDividePassiveSkillScRsp {
            ADADHIHDHJC: 0,
            HBOCMBOBJNG: ::protobuf::MessageField::none(),
            AEABHIJBFCJ: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EquipAetherDividePassiveSkillScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EquipAetherDividePassiveSkillScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EquipAetherDividePassiveSkillScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EquipAetherDividePassiveSkillScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(EquipAetherDividePassiveSkillScRsp.proto\x1a\x11ADDCJFIBCFF.proto\x1a\
    \x11HHBFGOMBIMA.proto\"\xa6\x01\n\"EquipAetherDividePassiveSkillScRsp\
    \x12\x20\n\x0bADADHIHDHJC\x18\x0f\x20\x01(\rR\x0bADADHIHDHJC\x12.\n\x0bH\
    BOCMBOBJNG\x18\x02\x20\x01(\x0b2\x0c.HHBFGOMBIMAR\x0bHBOCMBOBJNG\x12.\n\
    \x0bAEABHIJBFCJ\x18\x01\x20\x01(\x0b2\x0c.ADDCJFIBCFFR\x0bAEABHIJBFCJb\
    \x06proto3\
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
            deps.push(super::ADDCJFIBCFF::file_descriptor().clone());
            deps.push(super::HHBFGOMBIMA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EquipAetherDividePassiveSkillScRsp::generated_message_descriptor_data());
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
