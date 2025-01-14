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

//! Generated file from `HBCAPGAGKHK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HBCAPGAGKHK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HBCAPGAGKHK {
    // message fields
    // @@protoc_insertion_point(field:HBCAPGAGKHK.OJBAILGKLBM)
    pub OJBAILGKLBM: ::protobuf::EnumOrUnknown<super::QuestStatus::QuestStatus>,
    // @@protoc_insertion_point(field:HBCAPGAGKHK.OKFKBJPEDNP)
    pub OKFKBJPEDNP: i64,
    // @@protoc_insertion_point(field:HBCAPGAGKHK.FFFEHLJFOFN)
    pub FFFEHLJFOFN: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:HBCAPGAGKHK.FINLPBFNLHP)
    pub FINLPBFNLHP: u32,
    // @@protoc_insertion_point(field:HBCAPGAGKHK.IPNHCCODNDI)
    pub IPNHCCODNDI: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HBCAPGAGKHK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HBCAPGAGKHK {
    fn default() -> &'a HBCAPGAGKHK {
        <HBCAPGAGKHK as ::protobuf::Message>::default_instance()
    }
}

impl HBCAPGAGKHK {
    pub fn new() -> HBCAPGAGKHK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OJBAILGKLBM",
            |m: &HBCAPGAGKHK| { &m.OJBAILGKLBM },
            |m: &mut HBCAPGAGKHK| { &mut m.OJBAILGKLBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OKFKBJPEDNP",
            |m: &HBCAPGAGKHK| { &m.OKFKBJPEDNP },
            |m: &mut HBCAPGAGKHK| { &mut m.OKFKBJPEDNP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FFFEHLJFOFN",
            |m: &HBCAPGAGKHK| { &m.FFFEHLJFOFN },
            |m: &mut HBCAPGAGKHK| { &mut m.FFFEHLJFOFN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FINLPBFNLHP",
            |m: &HBCAPGAGKHK| { &m.FINLPBFNLHP },
            |m: &mut HBCAPGAGKHK| { &mut m.FINLPBFNLHP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IPNHCCODNDI",
            |m: &HBCAPGAGKHK| { &m.IPNHCCODNDI },
            |m: &mut HBCAPGAGKHK| { &mut m.IPNHCCODNDI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HBCAPGAGKHK>(
            "HBCAPGAGKHK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HBCAPGAGKHK {
    const NAME: &'static str = "HBCAPGAGKHK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.OJBAILGKLBM = is.read_enum_or_unknown()?;
                },
                88 => {
                    self.OKFKBJPEDNP = is.read_int64()?;
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.FFFEHLJFOFN)?;
                },
                40 => {
                    self.FFFEHLJFOFN.push(is.read_uint32()?);
                },
                48 => {
                    self.FINLPBFNLHP = is.read_uint32()?;
                },
                16 => {
                    self.IPNHCCODNDI = is.read_uint32()?;
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
        if self.OJBAILGKLBM != ::protobuf::EnumOrUnknown::new(super::QuestStatus::QuestStatus::QUEST_NONE) {
            my_size += ::protobuf::rt::int32_size(4, self.OJBAILGKLBM.value());
        }
        if self.OKFKBJPEDNP != 0 {
            my_size += ::protobuf::rt::int64_size(11, self.OKFKBJPEDNP);
        }
        for value in &self.FFFEHLJFOFN {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        if self.FINLPBFNLHP != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.FINLPBFNLHP);
        }
        if self.IPNHCCODNDI != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.IPNHCCODNDI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OJBAILGKLBM != ::protobuf::EnumOrUnknown::new(super::QuestStatus::QuestStatus::QUEST_NONE) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.OJBAILGKLBM))?;
        }
        if self.OKFKBJPEDNP != 0 {
            os.write_int64(11, self.OKFKBJPEDNP)?;
        }
        for v in &self.FFFEHLJFOFN {
            os.write_uint32(5, *v)?;
        };
        if self.FINLPBFNLHP != 0 {
            os.write_uint32(6, self.FINLPBFNLHP)?;
        }
        if self.IPNHCCODNDI != 0 {
            os.write_uint32(2, self.IPNHCCODNDI)?;
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

    fn new() -> HBCAPGAGKHK {
        HBCAPGAGKHK::new()
    }

    fn clear(&mut self) {
        self.OJBAILGKLBM = ::protobuf::EnumOrUnknown::new(super::QuestStatus::QuestStatus::QUEST_NONE);
        self.OKFKBJPEDNP = 0;
        self.FFFEHLJFOFN.clear();
        self.FINLPBFNLHP = 0;
        self.IPNHCCODNDI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HBCAPGAGKHK {
        static instance: HBCAPGAGKHK = HBCAPGAGKHK {
            OJBAILGKLBM: ::protobuf::EnumOrUnknown::from_i32(0),
            OKFKBJPEDNP: 0,
            FFFEHLJFOFN: ::std::vec::Vec::new(),
            FINLPBFNLHP: 0,
            IPNHCCODNDI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HBCAPGAGKHK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HBCAPGAGKHK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HBCAPGAGKHK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HBCAPGAGKHK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HBCAPGAGKHK.proto\x1a\x11QuestStatus.proto\"\xc5\x01\n\x0bHBCAPGAG\
    KHK\x12.\n\x0bOJBAILGKLBM\x18\x04\x20\x01(\x0e2\x0c.QuestStatusR\x0bOJBA\
    ILGKLBM\x12\x20\n\x0bOKFKBJPEDNP\x18\x0b\x20\x01(\x03R\x0bOKFKBJPEDNP\
    \x12\x20\n\x0bFFFEHLJFOFN\x18\x05\x20\x03(\rR\x0bFFFEHLJFOFN\x12\x20\n\
    \x0bFINLPBFNLHP\x18\x06\x20\x01(\rR\x0bFINLPBFNLHP\x12\x20\n\x0bIPNHCCOD\
    NDI\x18\x02\x20\x01(\rR\x0bIPNHCCODNDIb\x06proto3\
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
            deps.push(super::QuestStatus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HBCAPGAGKHK::generated_message_descriptor_data());
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
