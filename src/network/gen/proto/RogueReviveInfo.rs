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

//! Generated file from `RogueReviveInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueReviveInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueReviveInfo {
    // message fields
    // @@protoc_insertion_point(field:RogueReviveInfo.HONNEKKCDAB)
    pub HONNEKKCDAB: u32,
    // @@protoc_insertion_point(field:RogueReviveInfo.DDFABECFCLI)
    pub DDFABECFCLI: u32,
    // @@protoc_insertion_point(field:RogueReviveInfo.rogue_revive_cost)
    pub rogue_revive_cost: ::protobuf::MessageField<super::ItemCostList::ItemCostList>,
    // @@protoc_insertion_point(field:RogueReviveInfo.OBLFFICKCJL)
    pub OBLFFICKCJL: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueReviveInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueReviveInfo {
    fn default() -> &'a RogueReviveInfo {
        <RogueReviveInfo as ::protobuf::Message>::default_instance()
    }
}

impl RogueReviveInfo {
    pub fn new() -> RogueReviveInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HONNEKKCDAB",
            |m: &RogueReviveInfo| { &m.HONNEKKCDAB },
            |m: &mut RogueReviveInfo| { &mut m.HONNEKKCDAB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DDFABECFCLI",
            |m: &RogueReviveInfo| { &m.DDFABECFCLI },
            |m: &mut RogueReviveInfo| { &mut m.DDFABECFCLI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemCostList::ItemCostList>(
            "rogue_revive_cost",
            |m: &RogueReviveInfo| { &m.rogue_revive_cost },
            |m: &mut RogueReviveInfo| { &mut m.rogue_revive_cost },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OBLFFICKCJL",
            |m: &RogueReviveInfo| { &m.OBLFFICKCJL },
            |m: &mut RogueReviveInfo| { &mut m.OBLFFICKCJL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueReviveInfo>(
            "RogueReviveInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueReviveInfo {
    const NAME: &'static str = "RogueReviveInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.HONNEKKCDAB = is.read_uint32()?;
                },
                40 => {
                    self.DDFABECFCLI = is.read_uint32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_revive_cost)?;
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.OBLFFICKCJL)?;
                },
                24 => {
                    self.OBLFFICKCJL.push(is.read_uint32()?);
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
        if self.HONNEKKCDAB != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.HONNEKKCDAB);
        }
        if self.DDFABECFCLI != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.DDFABECFCLI);
        }
        if let Some(v) = self.rogue_revive_cost.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.OBLFFICKCJL {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HONNEKKCDAB != 0 {
            os.write_uint32(10, self.HONNEKKCDAB)?;
        }
        if self.DDFABECFCLI != 0 {
            os.write_uint32(5, self.DDFABECFCLI)?;
        }
        if let Some(v) = self.rogue_revive_cost.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        for v in &self.OBLFFICKCJL {
            os.write_uint32(3, *v)?;
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

    fn new() -> RogueReviveInfo {
        RogueReviveInfo::new()
    }

    fn clear(&mut self) {
        self.HONNEKKCDAB = 0;
        self.DDFABECFCLI = 0;
        self.rogue_revive_cost.clear();
        self.OBLFFICKCJL.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueReviveInfo {
        static instance: RogueReviveInfo = RogueReviveInfo {
            HONNEKKCDAB: 0,
            DDFABECFCLI: 0,
            rogue_revive_cost: ::protobuf::MessageField::none(),
            OBLFFICKCJL: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueReviveInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueReviveInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueReviveInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueReviveInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15RogueReviveInfo.proto\x1a\x12ItemCostList.proto\"\xb2\x01\n\x0fRog\
    ueReviveInfo\x12\x20\n\x0bHONNEKKCDAB\x18\n\x20\x01(\rR\x0bHONNEKKCDAB\
    \x12\x20\n\x0bDDFABECFCLI\x18\x05\x20\x01(\rR\x0bDDFABECFCLI\x129\n\x11r\
    ogue_revive_cost\x18\x0f\x20\x01(\x0b2\r.ItemCostListR\x0frogueReviveCos\
    t\x12\x20\n\x0bOBLFFICKCJL\x18\x03\x20\x03(\rR\x0bOBLFFICKCJLB\x15\n\x13\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::ItemCostList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueReviveInfo::generated_message_descriptor_data());
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