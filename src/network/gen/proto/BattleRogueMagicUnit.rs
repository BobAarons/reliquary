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

//! Generated file from `BattleRogueMagicUnit.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BattleRogueMagicUnit)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BattleRogueMagicUnit {
    // message fields
    // @@protoc_insertion_point(field:BattleRogueMagicUnit.unit_id)
    pub unit_id: u32,
    // @@protoc_insertion_point(field:BattleRogueMagicUnit.level)
    pub level: u32,
    // @@protoc_insertion_point(field:BattleRogueMagicUnit.DCEDNGLAOJI)
    pub DCEDNGLAOJI: bool,
    // @@protoc_insertion_point(field:BattleRogueMagicUnit.NFANFIHOBLC)
    pub NFANFIHOBLC: u32,
    // @@protoc_insertion_point(field:BattleRogueMagicUnit.HGOCMKMFNDG)
    pub HGOCMKMFNDG: ::std::collections::HashMap<u32, u32>,
    // special fields
    // @@protoc_insertion_point(special_field:BattleRogueMagicUnit.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BattleRogueMagicUnit {
    fn default() -> &'a BattleRogueMagicUnit {
        <BattleRogueMagicUnit as ::protobuf::Message>::default_instance()
    }
}

impl BattleRogueMagicUnit {
    pub fn new() -> BattleRogueMagicUnit {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unit_id",
            |m: &BattleRogueMagicUnit| { &m.unit_id },
            |m: &mut BattleRogueMagicUnit| { &mut m.unit_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &BattleRogueMagicUnit| { &m.level },
            |m: &mut BattleRogueMagicUnit| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DCEDNGLAOJI",
            |m: &BattleRogueMagicUnit| { &m.DCEDNGLAOJI },
            |m: &mut BattleRogueMagicUnit| { &mut m.DCEDNGLAOJI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NFANFIHOBLC",
            |m: &BattleRogueMagicUnit| { &m.NFANFIHOBLC },
            |m: &mut BattleRogueMagicUnit| { &mut m.NFANFIHOBLC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "HGOCMKMFNDG",
            |m: &BattleRogueMagicUnit| { &m.HGOCMKMFNDG },
            |m: &mut BattleRogueMagicUnit| { &mut m.HGOCMKMFNDG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BattleRogueMagicUnit>(
            "BattleRogueMagicUnit",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BattleRogueMagicUnit {
    const NAME: &'static str = "BattleRogueMagicUnit";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.unit_id = is.read_uint32()?;
                },
                16 => {
                    self.level = is.read_uint32()?;
                },
                24 => {
                    self.DCEDNGLAOJI = is.read_bool()?;
                },
                32 => {
                    self.NFANFIHOBLC = is.read_uint32()?;
                },
                42 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.HGOCMKMFNDG.insert(key, value);
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
        if self.unit_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.unit_id);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.level);
        }
        if self.DCEDNGLAOJI != false {
            my_size += 1 + 1;
        }
        if self.NFANFIHOBLC != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.NFANFIHOBLC);
        }
        for (k, v) in &self.HGOCMKMFNDG {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.unit_id != 0 {
            os.write_uint32(1, self.unit_id)?;
        }
        if self.level != 0 {
            os.write_uint32(2, self.level)?;
        }
        if self.DCEDNGLAOJI != false {
            os.write_bool(3, self.DCEDNGLAOJI)?;
        }
        if self.NFANFIHOBLC != 0 {
            os.write_uint32(4, self.NFANFIHOBLC)?;
        }
        for (k, v) in &self.HGOCMKMFNDG {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(42)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> BattleRogueMagicUnit {
        BattleRogueMagicUnit::new()
    }

    fn clear(&mut self) {
        self.unit_id = 0;
        self.level = 0;
        self.DCEDNGLAOJI = false;
        self.NFANFIHOBLC = 0;
        self.HGOCMKMFNDG.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BattleRogueMagicUnit {
        static instance: ::protobuf::rt::Lazy<BattleRogueMagicUnit> = ::protobuf::rt::Lazy::new();
        instance.get(BattleRogueMagicUnit::new)
    }
}

impl ::protobuf::MessageFull for BattleRogueMagicUnit {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BattleRogueMagicUnit").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BattleRogueMagicUnit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BattleRogueMagicUnit {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aBattleRogueMagicUnit.proto\"\x93\x02\n\x14BattleRogueMagicUnit\x12\
    \x17\n\x07unit_id\x18\x01\x20\x01(\rR\x06unitId\x12\x14\n\x05level\x18\
    \x02\x20\x01(\rR\x05level\x12\x20\n\x0bDCEDNGLAOJI\x18\x03\x20\x01(\x08R\
    \x0bDCEDNGLAOJI\x12\x20\n\x0bNFANFIHOBLC\x18\x04\x20\x01(\rR\x0bNFANFIHO\
    BLC\x12H\n\x0bHGOCMKMFNDG\x18\x05\x20\x03(\x0b2&.BattleRogueMagicUnit.HG\
    OCMKMFNDGEntryR\x0bHGOCMKMFNDG\x1a>\n\x10HGOCMKMFNDGEntry\x12\x10\n\x03k\
    ey\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05\
    value:\x028\x01B\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            messages.push(BattleRogueMagicUnit::generated_message_descriptor_data());
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