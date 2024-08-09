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

//! Generated file from `Quest.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:Quest)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Quest {
    // message fields
    // @@protoc_insertion_point(field:Quest.finish_time)
    pub finish_time: i64,
    // @@protoc_insertion_point(field:Quest.id)
    pub id: u32,
    // @@protoc_insertion_point(field:Quest.progress)
    pub progress: u32,
    // @@protoc_insertion_point(field:Quest.status)
    pub status: ::protobuf::EnumOrUnknown<super::QuestStatus::QuestStatus>,
    // special fields
    // @@protoc_insertion_point(special_field:Quest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Quest {
    fn default() -> &'a Quest {
        <Quest as ::protobuf::Message>::default_instance()
    }
}

impl Quest {
    pub fn new() -> Quest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "finish_time",
            |m: &Quest| { &m.finish_time },
            |m: &mut Quest| { &mut m.finish_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &Quest| { &m.id },
            |m: &mut Quest| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "progress",
            |m: &Quest| { &m.progress },
            |m: &mut Quest| { &mut m.progress },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &Quest| { &m.status },
            |m: &mut Quest| { &mut m.status },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Quest>(
            "Quest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Quest {
    const NAME: &'static str = "Quest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.finish_time = is.read_int64()?;
                },
                120 => {
                    self.id = is.read_uint32()?;
                },
                48 => {
                    self.progress = is.read_uint32()?;
                },
                64 => {
                    self.status = is.read_enum_or_unknown()?;
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
        if self.finish_time != 0 {
            my_size += ::protobuf::rt::int64_size(4, self.finish_time);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.id);
        }
        if self.progress != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.progress);
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::QuestStatus::QuestStatus::QUEST_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.status.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.finish_time != 0 {
            os.write_int64(4, self.finish_time)?;
        }
        if self.id != 0 {
            os.write_uint32(15, self.id)?;
        }
        if self.progress != 0 {
            os.write_uint32(6, self.progress)?;
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::QuestStatus::QuestStatus::QUEST_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.status))?;
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

    fn new() -> Quest {
        Quest::new()
    }

    fn clear(&mut self) {
        self.finish_time = 0;
        self.id = 0;
        self.progress = 0;
        self.status = ::protobuf::EnumOrUnknown::new(super::QuestStatus::QuestStatus::QUEST_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Quest {
        static instance: Quest = Quest {
            finish_time: 0,
            id: 0,
            progress: 0,
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Quest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Quest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Quest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Quest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bQuest.proto\x1a\x11QuestStatus.proto\"z\n\x05Quest\x12\x1f\n\x0bfi\
    nish_time\x18\x04\x20\x01(\x03R\nfinishTime\x12\x0e\n\x02id\x18\x0f\x20\
    \x01(\rR\x02id\x12\x1a\n\x08progress\x18\x06\x20\x01(\rR\x08progress\x12\
    $\n\x06status\x18\x08\x20\x01(\x0e2\x0c.QuestStatusR\x06statusB\x15\n\
    \x13emu.lunarcore.protob\x06proto3\
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
            messages.push(Quest::generated_message_descriptor_data());
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
