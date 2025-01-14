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

//! Generated file from `CmdMessageType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdMessageType)
pub enum CmdMessageType {
    // @@protoc_insertion_point(enum_value:CmdMessageType.CmdMessageTypeNone)
    CmdMessageTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdMessageType.CmdGetNpcMessageGroupScRsp)
    CmdGetNpcMessageGroupScRsp = 2720,
    // @@protoc_insertion_point(enum_value:CmdMessageType.CmdFinishSectionIdScRsp)
    CmdFinishSectionIdScRsp = 2737,
    // @@protoc_insertion_point(enum_value:CmdMessageType.CmdFinishItemIdScRsp)
    CmdFinishItemIdScRsp = 2753,
    // @@protoc_insertion_point(enum_value:CmdMessageType.CmdFinishPerformSectionIdCsReq)
    CmdFinishPerformSectionIdCsReq = 2780,
    // @@protoc_insertion_point(enum_value:CmdMessageType.CmdGetNpcStatusCsReq)
    CmdGetNpcStatusCsReq = 2703,
    // @@protoc_insertion_point(enum_value:CmdMessageType.CmdFinishItemIdCsReq)
    CmdFinishItemIdCsReq = 2739,
    // @@protoc_insertion_point(enum_value:CmdMessageType.CmdFinishSectionIdCsReq)
    CmdFinishSectionIdCsReq = 2734,
    // @@protoc_insertion_point(enum_value:CmdMessageType.CmdGetNpcMessageGroupCsReq)
    CmdGetNpcMessageGroupCsReq = 2759,
    // @@protoc_insertion_point(enum_value:CmdMessageType.CmdFinishPerformSectionIdScRsp)
    CmdFinishPerformSectionIdScRsp = 2716,
    // @@protoc_insertion_point(enum_value:CmdMessageType.CmdGetNpcStatusScRsp)
    CmdGetNpcStatusScRsp = 2746,
}

impl ::protobuf::Enum for CmdMessageType {
    const NAME: &'static str = "CmdMessageType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdMessageType> {
        match value {
            0 => ::std::option::Option::Some(CmdMessageType::CmdMessageTypeNone),
            2720 => ::std::option::Option::Some(CmdMessageType::CmdGetNpcMessageGroupScRsp),
            2737 => ::std::option::Option::Some(CmdMessageType::CmdFinishSectionIdScRsp),
            2753 => ::std::option::Option::Some(CmdMessageType::CmdFinishItemIdScRsp),
            2780 => ::std::option::Option::Some(CmdMessageType::CmdFinishPerformSectionIdCsReq),
            2703 => ::std::option::Option::Some(CmdMessageType::CmdGetNpcStatusCsReq),
            2739 => ::std::option::Option::Some(CmdMessageType::CmdFinishItemIdCsReq),
            2734 => ::std::option::Option::Some(CmdMessageType::CmdFinishSectionIdCsReq),
            2759 => ::std::option::Option::Some(CmdMessageType::CmdGetNpcMessageGroupCsReq),
            2716 => ::std::option::Option::Some(CmdMessageType::CmdFinishPerformSectionIdScRsp),
            2746 => ::std::option::Option::Some(CmdMessageType::CmdGetNpcStatusScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdMessageType> {
        match str {
            "CmdMessageTypeNone" => ::std::option::Option::Some(CmdMessageType::CmdMessageTypeNone),
            "CmdGetNpcMessageGroupScRsp" => ::std::option::Option::Some(CmdMessageType::CmdGetNpcMessageGroupScRsp),
            "CmdFinishSectionIdScRsp" => ::std::option::Option::Some(CmdMessageType::CmdFinishSectionIdScRsp),
            "CmdFinishItemIdScRsp" => ::std::option::Option::Some(CmdMessageType::CmdFinishItemIdScRsp),
            "CmdFinishPerformSectionIdCsReq" => ::std::option::Option::Some(CmdMessageType::CmdFinishPerformSectionIdCsReq),
            "CmdGetNpcStatusCsReq" => ::std::option::Option::Some(CmdMessageType::CmdGetNpcStatusCsReq),
            "CmdFinishItemIdCsReq" => ::std::option::Option::Some(CmdMessageType::CmdFinishItemIdCsReq),
            "CmdFinishSectionIdCsReq" => ::std::option::Option::Some(CmdMessageType::CmdFinishSectionIdCsReq),
            "CmdGetNpcMessageGroupCsReq" => ::std::option::Option::Some(CmdMessageType::CmdGetNpcMessageGroupCsReq),
            "CmdFinishPerformSectionIdScRsp" => ::std::option::Option::Some(CmdMessageType::CmdFinishPerformSectionIdScRsp),
            "CmdGetNpcStatusScRsp" => ::std::option::Option::Some(CmdMessageType::CmdGetNpcStatusScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdMessageType] = &[
        CmdMessageType::CmdMessageTypeNone,
        CmdMessageType::CmdGetNpcMessageGroupScRsp,
        CmdMessageType::CmdFinishSectionIdScRsp,
        CmdMessageType::CmdFinishItemIdScRsp,
        CmdMessageType::CmdFinishPerformSectionIdCsReq,
        CmdMessageType::CmdGetNpcStatusCsReq,
        CmdMessageType::CmdFinishItemIdCsReq,
        CmdMessageType::CmdFinishSectionIdCsReq,
        CmdMessageType::CmdGetNpcMessageGroupCsReq,
        CmdMessageType::CmdFinishPerformSectionIdScRsp,
        CmdMessageType::CmdGetNpcStatusScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdMessageType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdMessageType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdMessageType::CmdMessageTypeNone => 0,
            CmdMessageType::CmdGetNpcMessageGroupScRsp => 1,
            CmdMessageType::CmdFinishSectionIdScRsp => 2,
            CmdMessageType::CmdFinishItemIdScRsp => 3,
            CmdMessageType::CmdFinishPerformSectionIdCsReq => 4,
            CmdMessageType::CmdGetNpcStatusCsReq => 5,
            CmdMessageType::CmdFinishItemIdCsReq => 6,
            CmdMessageType::CmdFinishSectionIdCsReq => 7,
            CmdMessageType::CmdGetNpcMessageGroupCsReq => 8,
            CmdMessageType::CmdFinishPerformSectionIdScRsp => 9,
            CmdMessageType::CmdGetNpcStatusScRsp => 10,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdMessageType {
    fn default() -> Self {
        CmdMessageType::CmdMessageTypeNone
    }
}

impl CmdMessageType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdMessageType>("CmdMessageType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14CmdMessageType.proto*\xdc\x02\n\x0eCmdMessageType\x12\x16\n\x12Cmd\
    MessageTypeNone\x10\0\x12\x1f\n\x1aCmdGetNpcMessageGroupScRsp\x10\xa0\
    \x15\x12\x1c\n\x17CmdFinishSectionIdScRsp\x10\xb1\x15\x12\x19\n\x14CmdFi\
    nishItemIdScRsp\x10\xc1\x15\x12#\n\x1eCmdFinishPerformSectionIdCsReq\x10\
    \xdc\x15\x12\x19\n\x14CmdGetNpcStatusCsReq\x10\x8f\x15\x12\x19\n\x14CmdF\
    inishItemIdCsReq\x10\xb3\x15\x12\x1c\n\x17CmdFinishSectionIdCsReq\x10\
    \xae\x15\x12\x1f\n\x1aCmdGetNpcMessageGroupCsReq\x10\xc7\x15\x12#\n\x1eC\
    mdFinishPerformSectionIdScRsp\x10\x9c\x15\x12\x19\n\x14CmdGetNpcStatusSc\
    Rsp\x10\xba\x15b\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(CmdMessageType::generated_enum_descriptor_data());
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
