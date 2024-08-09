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

//! Generated file from `SceneSummonUnitInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SceneSummonUnitInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneSummonUnitInfo {
    // message fields
    // @@protoc_insertion_point(field:SceneSummonUnitInfo.trigger_name_list)
    pub trigger_name_list: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:SceneSummonUnitInfo.life_time_ms)
    pub life_time_ms: i32,
    // @@protoc_insertion_point(field:SceneSummonUnitInfo.caster_entity_id)
    pub caster_entity_id: u32,
    // @@protoc_insertion_point(field:SceneSummonUnitInfo.attach_entity_id)
    pub attach_entity_id: u32,
    // @@protoc_insertion_point(field:SceneSummonUnitInfo.summon_unit_id)
    pub summon_unit_id: u32,
    // @@protoc_insertion_point(field:SceneSummonUnitInfo.create_time_ms)
    pub create_time_ms: u64,
    // special fields
    // @@protoc_insertion_point(special_field:SceneSummonUnitInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneSummonUnitInfo {
    fn default() -> &'a SceneSummonUnitInfo {
        <SceneSummonUnitInfo as ::protobuf::Message>::default_instance()
    }
}

impl SceneSummonUnitInfo {
    pub fn new() -> SceneSummonUnitInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "trigger_name_list",
            |m: &SceneSummonUnitInfo| { &m.trigger_name_list },
            |m: &mut SceneSummonUnitInfo| { &mut m.trigger_name_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "life_time_ms",
            |m: &SceneSummonUnitInfo| { &m.life_time_ms },
            |m: &mut SceneSummonUnitInfo| { &mut m.life_time_ms },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "caster_entity_id",
            |m: &SceneSummonUnitInfo| { &m.caster_entity_id },
            |m: &mut SceneSummonUnitInfo| { &mut m.caster_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "attach_entity_id",
            |m: &SceneSummonUnitInfo| { &m.attach_entity_id },
            |m: &mut SceneSummonUnitInfo| { &mut m.attach_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "summon_unit_id",
            |m: &SceneSummonUnitInfo| { &m.summon_unit_id },
            |m: &mut SceneSummonUnitInfo| { &mut m.summon_unit_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "create_time_ms",
            |m: &SceneSummonUnitInfo| { &m.create_time_ms },
            |m: &mut SceneSummonUnitInfo| { &mut m.create_time_ms },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneSummonUnitInfo>(
            "SceneSummonUnitInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneSummonUnitInfo {
    const NAME: &'static str = "SceneSummonUnitInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    self.trigger_name_list.push(is.read_string()?);
                },
                120 => {
                    self.life_time_ms = is.read_sint32()?;
                },
                32 => {
                    self.caster_entity_id = is.read_uint32()?;
                },
                80 => {
                    self.attach_entity_id = is.read_uint32()?;
                },
                112 => {
                    self.summon_unit_id = is.read_uint32()?;
                },
                96 => {
                    self.create_time_ms = is.read_uint64()?;
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
        for value in &self.trigger_name_list {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        if self.life_time_ms != 0 {
            my_size += ::protobuf::rt::sint32_size(15, self.life_time_ms);
        }
        if self.caster_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.caster_entity_id);
        }
        if self.attach_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.attach_entity_id);
        }
        if self.summon_unit_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.summon_unit_id);
        }
        if self.create_time_ms != 0 {
            my_size += ::protobuf::rt::uint64_size(12, self.create_time_ms);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.trigger_name_list {
            os.write_string(6, &v)?;
        };
        if self.life_time_ms != 0 {
            os.write_sint32(15, self.life_time_ms)?;
        }
        if self.caster_entity_id != 0 {
            os.write_uint32(4, self.caster_entity_id)?;
        }
        if self.attach_entity_id != 0 {
            os.write_uint32(10, self.attach_entity_id)?;
        }
        if self.summon_unit_id != 0 {
            os.write_uint32(14, self.summon_unit_id)?;
        }
        if self.create_time_ms != 0 {
            os.write_uint64(12, self.create_time_ms)?;
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

    fn new() -> SceneSummonUnitInfo {
        SceneSummonUnitInfo::new()
    }

    fn clear(&mut self) {
        self.trigger_name_list.clear();
        self.life_time_ms = 0;
        self.caster_entity_id = 0;
        self.attach_entity_id = 0;
        self.summon_unit_id = 0;
        self.create_time_ms = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneSummonUnitInfo {
        static instance: SceneSummonUnitInfo = SceneSummonUnitInfo {
            trigger_name_list: ::std::vec::Vec::new(),
            life_time_ms: 0,
            caster_entity_id: 0,
            attach_entity_id: 0,
            summon_unit_id: 0,
            create_time_ms: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneSummonUnitInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneSummonUnitInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneSummonUnitInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneSummonUnitInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19SceneSummonUnitInfo.proto\"\x83\x02\n\x13SceneSummonUnitInfo\x12*\
    \n\x11trigger_name_list\x18\x06\x20\x03(\tR\x0ftriggerNameList\x12\x20\n\
    \x0clife_time_ms\x18\x0f\x20\x01(\x11R\nlifeTimeMs\x12(\n\x10caster_enti\
    ty_id\x18\x04\x20\x01(\rR\x0ecasterEntityId\x12(\n\x10attach_entity_id\
    \x18\n\x20\x01(\rR\x0eattachEntityId\x12$\n\x0esummon_unit_id\x18\x0e\
    \x20\x01(\rR\x0csummonUnitId\x12$\n\x0ecreate_time_ms\x18\x0c\x20\x01(\
    \x04R\x0ccreateTimeMsB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            messages.push(SceneSummonUnitInfo::generated_message_descriptor_data());
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
