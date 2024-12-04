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

//! Generated file from `RogueMagicBattleUnitInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueMagicBattleUnitInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueMagicBattleUnitInfo {
    // message oneof groups
    pub item: ::std::option::Option<rogue_magic_battle_unit_info::Item>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueMagicBattleUnitInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueMagicBattleUnitInfo {
    fn default() -> &'a RogueMagicBattleUnitInfo {
        <RogueMagicBattleUnitInfo as ::protobuf::Message>::default_instance()
    }
}

impl RogueMagicBattleUnitInfo {
    pub fn new() -> RogueMagicBattleUnitInfo {
        ::std::default::Default::default()
    }

    // .BattleRogueMagicData battle_rogue_magic_data = 1;

    pub fn battle_rogue_magic_data(&self) -> &super::BattleRogueMagicData::BattleRogueMagicData {
        match self.item {
            ::std::option::Option::Some(rogue_magic_battle_unit_info::Item::BattleRogueMagicData(ref v)) => v,
            _ => <super::BattleRogueMagicData::BattleRogueMagicData as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_battle_rogue_magic_data(&mut self) {
        self.item = ::std::option::Option::None;
    }

    pub fn has_battle_rogue_magic_data(&self) -> bool {
        match self.item {
            ::std::option::Option::Some(rogue_magic_battle_unit_info::Item::BattleRogueMagicData(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_battle_rogue_magic_data(&mut self, v: super::BattleRogueMagicData::BattleRogueMagicData) {
        self.item = ::std::option::Option::Some(rogue_magic_battle_unit_info::Item::BattleRogueMagicData(v))
    }

    // Mutable pointer to the field.
    pub fn mut_battle_rogue_magic_data(&mut self) -> &mut super::BattleRogueMagicData::BattleRogueMagicData {
        if let ::std::option::Option::Some(rogue_magic_battle_unit_info::Item::BattleRogueMagicData(_)) = self.item {
        } else {
            self.item = ::std::option::Option::Some(rogue_magic_battle_unit_info::Item::BattleRogueMagicData(super::BattleRogueMagicData::BattleRogueMagicData::new()));
        }
        match self.item {
            ::std::option::Option::Some(rogue_magic_battle_unit_info::Item::BattleRogueMagicData(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_battle_rogue_magic_data(&mut self) -> super::BattleRogueMagicData::BattleRogueMagicData {
        if self.has_battle_rogue_magic_data() {
            match self.item.take() {
                ::std::option::Option::Some(rogue_magic_battle_unit_info::Item::BattleRogueMagicData(v)) => v,
                _ => panic!(),
            }
        } else {
            super::BattleRogueMagicData::BattleRogueMagicData::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::BattleRogueMagicData::BattleRogueMagicData>(
            "battle_rogue_magic_data",
            RogueMagicBattleUnitInfo::has_battle_rogue_magic_data,
            RogueMagicBattleUnitInfo::battle_rogue_magic_data,
            RogueMagicBattleUnitInfo::mut_battle_rogue_magic_data,
            RogueMagicBattleUnitInfo::set_battle_rogue_magic_data,
        ));
        oneofs.push(rogue_magic_battle_unit_info::Item::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueMagicBattleUnitInfo>(
            "RogueMagicBattleUnitInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueMagicBattleUnitInfo {
    const NAME: &'static str = "RogueMagicBattleUnitInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.item = ::std::option::Option::Some(rogue_magic_battle_unit_info::Item::BattleRogueMagicData(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.item {
            match v {
                &rogue_magic_battle_unit_info::Item::BattleRogueMagicData(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.item {
            match v {
                &rogue_magic_battle_unit_info::Item::BattleRogueMagicData(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
            };
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

    fn new() -> RogueMagicBattleUnitInfo {
        RogueMagicBattleUnitInfo::new()
    }

    fn clear(&mut self) {
        self.item = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueMagicBattleUnitInfo {
        static instance: RogueMagicBattleUnitInfo = RogueMagicBattleUnitInfo {
            item: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueMagicBattleUnitInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueMagicBattleUnitInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueMagicBattleUnitInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueMagicBattleUnitInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `RogueMagicBattleUnitInfo`
pub mod rogue_magic_battle_unit_info {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:RogueMagicBattleUnitInfo.item)
    pub enum Item {
        // @@protoc_insertion_point(oneof_field:RogueMagicBattleUnitInfo.battle_rogue_magic_data)
        BattleRogueMagicData(super::super::BattleRogueMagicData::BattleRogueMagicData),
    }

    impl ::protobuf::Oneof for Item {
    }

    impl ::protobuf::OneofFull for Item {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::RogueMagicBattleUnitInfo as ::protobuf::MessageFull>::descriptor().oneof_by_name("item").unwrap()).clone()
        }
    }

    impl Item {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Item>("item")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eRogueMagicBattleUnitInfo.proto\x1a\x1aBattleRogueMagicData.proto\"\
    r\n\x18RogueMagicBattleUnitInfo\x12N\n\x17battle_rogue_magic_data\x18\
    \x01\x20\x01(\x0b2\x15.BattleRogueMagicDataH\0R\x14battleRogueMagicDataB\
    \x06\n\x04itemB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::BattleRogueMagicData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueMagicBattleUnitInfo::generated_message_descriptor_data());
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