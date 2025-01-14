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

//! Generated file from `ChallengeBossInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChallengeBossInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChallengeBossInfo {
    // message fields
    // @@protoc_insertion_point(field:ChallengeBossInfo.second_node)
    pub second_node: ::protobuf::MessageField<super::ChallengeBossStageInfo::ChallengeBossStageInfo>,
    // @@protoc_insertion_point(field:ChallengeBossInfo.first_lineup_ids)
    pub first_lineup_ids: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ChallengeBossInfo.used_equipment_map)
    pub used_equipment_map: ::std::collections::HashMap<u32, super::ChallengeBossEquipment::ChallengeBossEquipment>,
    // @@protoc_insertion_point(field:ChallengeBossInfo.used_relic_map)
    pub used_relic_map: ::std::collections::HashMap<u32, super::ChallengeBossRelicList::ChallengeBossRelicList>,
    // @@protoc_insertion_point(field:ChallengeBossInfo.BCMDAAKCNBA)
    pub BCMDAAKCNBA: bool,
    // @@protoc_insertion_point(field:ChallengeBossInfo.second_lineup_ids)
    pub second_lineup_ids: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ChallengeBossInfo.first_node)
    pub first_node: ::protobuf::MessageField<super::ChallengeBossStageInfo::ChallengeBossStageInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:ChallengeBossInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChallengeBossInfo {
    fn default() -> &'a ChallengeBossInfo {
        <ChallengeBossInfo as ::protobuf::Message>::default_instance()
    }
}

impl ChallengeBossInfo {
    pub fn new() -> ChallengeBossInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChallengeBossStageInfo::ChallengeBossStageInfo>(
            "second_node",
            |m: &ChallengeBossInfo| { &m.second_node },
            |m: &mut ChallengeBossInfo| { &mut m.second_node },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "first_lineup_ids",
            |m: &ChallengeBossInfo| { &m.first_lineup_ids },
            |m: &mut ChallengeBossInfo| { &mut m.first_lineup_ids },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "used_equipment_map",
            |m: &ChallengeBossInfo| { &m.used_equipment_map },
            |m: &mut ChallengeBossInfo| { &mut m.used_equipment_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "used_relic_map",
            |m: &ChallengeBossInfo| { &m.used_relic_map },
            |m: &mut ChallengeBossInfo| { &mut m.used_relic_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BCMDAAKCNBA",
            |m: &ChallengeBossInfo| { &m.BCMDAAKCNBA },
            |m: &mut ChallengeBossInfo| { &mut m.BCMDAAKCNBA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "second_lineup_ids",
            |m: &ChallengeBossInfo| { &m.second_lineup_ids },
            |m: &mut ChallengeBossInfo| { &mut m.second_lineup_ids },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChallengeBossStageInfo::ChallengeBossStageInfo>(
            "first_node",
            |m: &ChallengeBossInfo| { &m.first_node },
            |m: &mut ChallengeBossInfo| { &mut m.first_node },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChallengeBossInfo>(
            "ChallengeBossInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChallengeBossInfo {
    const NAME: &'static str = "ChallengeBossInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.second_node)?;
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.first_lineup_ids)?;
                },
                40 => {
                    self.first_lineup_ids.push(is.read_uint32()?);
                },
                82 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.used_equipment_map.insert(key, value);
                },
                58 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.used_relic_map.insert(key, value);
                },
                112 => {
                    self.BCMDAAKCNBA = is.read_bool()?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.second_lineup_ids)?;
                },
                104 => {
                    self.second_lineup_ids.push(is.read_uint32()?);
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.first_node)?;
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
        if let Some(v) = self.second_node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.first_lineup_ids {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        for (k, v) in &self.used_equipment_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for (k, v) in &self.used_relic_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.BCMDAAKCNBA != false {
            my_size += 1 + 1;
        }
        for value in &self.second_lineup_ids {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        if let Some(v) = self.first_node.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.second_node.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        for v in &self.first_lineup_ids {
            os.write_uint32(5, *v)?;
        };
        for (k, v) in &self.used_equipment_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(82)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        for (k, v) in &self.used_relic_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(58)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.BCMDAAKCNBA != false {
            os.write_bool(14, self.BCMDAAKCNBA)?;
        }
        for v in &self.second_lineup_ids {
            os.write_uint32(13, *v)?;
        };
        if let Some(v) = self.first_node.as_ref() {
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

    fn new() -> ChallengeBossInfo {
        ChallengeBossInfo::new()
    }

    fn clear(&mut self) {
        self.second_node.clear();
        self.first_lineup_ids.clear();
        self.used_equipment_map.clear();
        self.used_relic_map.clear();
        self.BCMDAAKCNBA = false;
        self.second_lineup_ids.clear();
        self.first_node.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChallengeBossInfo {
        static instance: ::protobuf::rt::Lazy<ChallengeBossInfo> = ::protobuf::rt::Lazy::new();
        instance.get(ChallengeBossInfo::new)
    }
}

impl ::protobuf::MessageFull for ChallengeBossInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChallengeBossInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChallengeBossInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChallengeBossInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17ChallengeBossInfo.proto\x1a\x1cChallengeBossStageInfo.proto\x1a\
    \x1cChallengeBossRelicList.proto\x1a\x1cChallengeBossEquipment.proto\"\
    \xd9\x04\n\x11ChallengeBossInfo\x128\n\x0bsecond_node\x18\x06\x20\x01(\
    \x0b2\x17.ChallengeBossStageInfoR\nsecondNode\x12(\n\x10first_lineup_ids\
    \x18\x05\x20\x03(\rR\x0efirstLineupIds\x12V\n\x12used_equipment_map\x18\
    \n\x20\x03(\x0b2(.ChallengeBossInfo.UsedEquipmentMapEntryR\x10usedEquipm\
    entMap\x12J\n\x0eused_relic_map\x18\x07\x20\x03(\x0b2$.ChallengeBossInfo\
    .UsedRelicMapEntryR\x0cusedRelicMap\x12\x20\n\x0bBCMDAAKCNBA\x18\x0e\x20\
    \x01(\x08R\x0bBCMDAAKCNBA\x12*\n\x11second_lineup_ids\x18\r\x20\x03(\rR\
    \x0fsecondLineupIds\x126\n\nfirst_node\x18\x01\x20\x01(\x0b2\x17.Challen\
    geBossStageInfoR\tfirstNode\x1a\\\n\x15UsedEquipmentMapEntry\x12\x10\n\
    \x03key\x18\x01\x20\x01(\rR\x03key\x12-\n\x05value\x18\x02\x20\x01(\x0b2\
    \x17.ChallengeBossEquipmentR\x05value:\x028\x01\x1aX\n\x11UsedRelicMapEn\
    try\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12-\n\x05value\x18\x02\
    \x20\x01(\x0b2\x17.ChallengeBossRelicListR\x05value:\x028\x01B\x15\n\x13\
    emu.lunarcore.protob\x06proto3\
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
            deps.push(super::ChallengeBossStageInfo::file_descriptor().clone());
            deps.push(super::ChallengeBossRelicList::file_descriptor().clone());
            deps.push(super::ChallengeBossEquipment::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChallengeBossInfo::generated_message_descriptor_data());
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
