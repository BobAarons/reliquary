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

//! Generated file from `PunkLordMonsterInfoScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PunkLordMonsterInfoScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PunkLordMonsterInfoScNotify {
    // message fields
    // @@protoc_insertion_point(field:PunkLordMonsterInfoScNotify.KGGHLADEKGP)
    pub KGGHLADEKGP: ::protobuf::EnumOrUnknown<super::DOHLDJOEPIM::DOHLDJOEPIM>,
    // @@protoc_insertion_point(field:PunkLordMonsterInfoScNotify.AGBKKKNDOGO)
    pub AGBKKKNDOGO: ::protobuf::MessageField<super::PunkLordBattleRecord::PunkLordBattleRecord>,
    // @@protoc_insertion_point(field:PunkLordMonsterInfoScNotify.DAAOBJPPKND)
    pub DAAOBJPPKND: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PunkLordMonsterInfoScNotify.CBBFEFKKNPB)
    pub CBBFEFKKNPB: ::protobuf::MessageField<super::CKEMKLHHEFO::CKEMKLHHEFO>,
    // special fields
    // @@protoc_insertion_point(special_field:PunkLordMonsterInfoScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PunkLordMonsterInfoScNotify {
    fn default() -> &'a PunkLordMonsterInfoScNotify {
        <PunkLordMonsterInfoScNotify as ::protobuf::Message>::default_instance()
    }
}

impl PunkLordMonsterInfoScNotify {
    pub fn new() -> PunkLordMonsterInfoScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGGHLADEKGP",
            |m: &PunkLordMonsterInfoScNotify| { &m.KGGHLADEKGP },
            |m: &mut PunkLordMonsterInfoScNotify| { &mut m.KGGHLADEKGP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PunkLordBattleRecord::PunkLordBattleRecord>(
            "AGBKKKNDOGO",
            |m: &PunkLordMonsterInfoScNotify| { &m.AGBKKKNDOGO },
            |m: &mut PunkLordMonsterInfoScNotify| { &mut m.AGBKKKNDOGO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DAAOBJPPKND",
            |m: &PunkLordMonsterInfoScNotify| { &m.DAAOBJPPKND },
            |m: &mut PunkLordMonsterInfoScNotify| { &mut m.DAAOBJPPKND },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CKEMKLHHEFO::CKEMKLHHEFO>(
            "CBBFEFKKNPB",
            |m: &PunkLordMonsterInfoScNotify| { &m.CBBFEFKKNPB },
            |m: &mut PunkLordMonsterInfoScNotify| { &mut m.CBBFEFKKNPB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PunkLordMonsterInfoScNotify>(
            "PunkLordMonsterInfoScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PunkLordMonsterInfoScNotify {
    const NAME: &'static str = "PunkLordMonsterInfoScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.KGGHLADEKGP = is.read_enum_or_unknown()?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.AGBKKKNDOGO)?;
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.DAAOBJPPKND)?;
                },
                32 => {
                    self.DAAOBJPPKND.push(is.read_uint32()?);
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CBBFEFKKNPB)?;
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
        if self.KGGHLADEKGP != ::protobuf::EnumOrUnknown::new(super::DOHLDJOEPIM::DOHLDJOEPIM::PUNK_LORD_MONSTER_INFO_NOTIFY_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(13, self.KGGHLADEKGP.value());
        }
        if let Some(v) = self.AGBKKKNDOGO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.DAAOBJPPKND {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        if let Some(v) = self.CBBFEFKKNPB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KGGHLADEKGP != ::protobuf::EnumOrUnknown::new(super::DOHLDJOEPIM::DOHLDJOEPIM::PUNK_LORD_MONSTER_INFO_NOTIFY_REASON_NONE) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.KGGHLADEKGP))?;
        }
        if let Some(v) = self.AGBKKKNDOGO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        for v in &self.DAAOBJPPKND {
            os.write_uint32(4, *v)?;
        };
        if let Some(v) = self.CBBFEFKKNPB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> PunkLordMonsterInfoScNotify {
        PunkLordMonsterInfoScNotify::new()
    }

    fn clear(&mut self) {
        self.KGGHLADEKGP = ::protobuf::EnumOrUnknown::new(super::DOHLDJOEPIM::DOHLDJOEPIM::PUNK_LORD_MONSTER_INFO_NOTIFY_REASON_NONE);
        self.AGBKKKNDOGO.clear();
        self.DAAOBJPPKND.clear();
        self.CBBFEFKKNPB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PunkLordMonsterInfoScNotify {
        static instance: PunkLordMonsterInfoScNotify = PunkLordMonsterInfoScNotify {
            KGGHLADEKGP: ::protobuf::EnumOrUnknown::from_i32(0),
            AGBKKKNDOGO: ::protobuf::MessageField::none(),
            DAAOBJPPKND: ::std::vec::Vec::new(),
            CBBFEFKKNPB: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PunkLordMonsterInfoScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PunkLordMonsterInfoScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PunkLordMonsterInfoScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PunkLordMonsterInfoScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!PunkLordMonsterInfoScNotify.proto\x1a\x11CKEMKLHHEFO.proto\x1a\x11DOH\
    LDJOEPIM.proto\x1a\x1aPunkLordBattleRecord.proto\"\xd8\x01\n\x1bPunkLord\
    MonsterInfoScNotify\x12.\n\x0bKGGHLADEKGP\x18\r\x20\x01(\x0e2\x0c.DOHLDJ\
    OEPIMR\x0bKGGHLADEKGP\x127\n\x0bAGBKKKNDOGO\x18\x06\x20\x01(\x0b2\x15.Pu\
    nkLordBattleRecordR\x0bAGBKKKNDOGO\x12\x20\n\x0bDAAOBJPPKND\x18\x04\x20\
    \x03(\rR\x0bDAAOBJPPKND\x12.\n\x0bCBBFEFKKNPB\x18\x0b\x20\x01(\x0b2\x0c.\
    CKEMKLHHEFOR\x0bCBBFEFKKNPBb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::CKEMKLHHEFO::file_descriptor().clone());
            deps.push(super::DOHLDJOEPIM::file_descriptor().clone());
            deps.push(super::PunkLordBattleRecord::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PunkLordMonsterInfoScNotify::generated_message_descriptor_data());
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