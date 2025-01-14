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

//! Generated file from `CmdMapRotationType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdMapRotationType)
pub enum CmdMapRotationType {
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdMapRotationTypeNone)
    CmdMapRotationTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdLeaveMapRotationRegionScNotify)
    CmdLeaveMapRotationRegionScNotify = 6848,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdEnterMapRotationRegionCsReq)
    CmdEnterMapRotationRegionCsReq = 6859,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdResetMapRotationRegionCsReq)
    CmdResetMapRotationRegionCsReq = 6830,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdRotateMapCsReq)
    CmdRotateMapCsReq = 6834,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdLeaveMapRotationRegionCsReq)
    CmdLeaveMapRotationRegionCsReq = 6880,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdRemoveRotaterScRsp)
    CmdRemoveRotaterScRsp = 6861,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdUpdateMapRotationDataScNotify)
    CmdUpdateMapRotationDataScNotify = 6879,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdDeployRotaterCsReq)
    CmdDeployRotaterCsReq = 6839,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdLeaveMapRotationRegionScRsp)
    CmdLeaveMapRotationRegionScRsp = 6816,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdInteractChargerScRsp)
    CmdInteractChargerScRsp = 6846,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdEnterMapRotationRegionScRsp)
    CmdEnterMapRotationRegionScRsp = 6820,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdDeployRotaterScRsp)
    CmdDeployRotaterScRsp = 6853,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdUpdateEnergyScNotify)
    CmdUpdateEnergyScNotify = 6890,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdResetMapRotationRegionScRsp)
    CmdResetMapRotationRegionScRsp = 6875,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdInteractChargerCsReq)
    CmdInteractChargerCsReq = 6803,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdGetMapRotationDataCsReq)
    CmdGetMapRotationDataCsReq = 6847,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdGetMapRotationDataScRsp)
    CmdGetMapRotationDataScRsp = 6874,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdRemoveRotaterCsReq)
    CmdRemoveRotaterCsReq = 6819,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdRotateMapScRsp)
    CmdRotateMapScRsp = 6837,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdUpdateRotaterScNotify)
    CmdUpdateRotaterScNotify = 6825,
}

impl ::protobuf::Enum for CmdMapRotationType {
    const NAME: &'static str = "CmdMapRotationType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdMapRotationType> {
        match value {
            0 => ::std::option::Option::Some(CmdMapRotationType::CmdMapRotationTypeNone),
            6848 => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionScNotify),
            6859 => ::std::option::Option::Some(CmdMapRotationType::CmdEnterMapRotationRegionCsReq),
            6830 => ::std::option::Option::Some(CmdMapRotationType::CmdResetMapRotationRegionCsReq),
            6834 => ::std::option::Option::Some(CmdMapRotationType::CmdRotateMapCsReq),
            6880 => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionCsReq),
            6861 => ::std::option::Option::Some(CmdMapRotationType::CmdRemoveRotaterScRsp),
            6879 => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateMapRotationDataScNotify),
            6839 => ::std::option::Option::Some(CmdMapRotationType::CmdDeployRotaterCsReq),
            6816 => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionScRsp),
            6846 => ::std::option::Option::Some(CmdMapRotationType::CmdInteractChargerScRsp),
            6820 => ::std::option::Option::Some(CmdMapRotationType::CmdEnterMapRotationRegionScRsp),
            6853 => ::std::option::Option::Some(CmdMapRotationType::CmdDeployRotaterScRsp),
            6890 => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateEnergyScNotify),
            6875 => ::std::option::Option::Some(CmdMapRotationType::CmdResetMapRotationRegionScRsp),
            6803 => ::std::option::Option::Some(CmdMapRotationType::CmdInteractChargerCsReq),
            6847 => ::std::option::Option::Some(CmdMapRotationType::CmdGetMapRotationDataCsReq),
            6874 => ::std::option::Option::Some(CmdMapRotationType::CmdGetMapRotationDataScRsp),
            6819 => ::std::option::Option::Some(CmdMapRotationType::CmdRemoveRotaterCsReq),
            6837 => ::std::option::Option::Some(CmdMapRotationType::CmdRotateMapScRsp),
            6825 => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateRotaterScNotify),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdMapRotationType> {
        match str {
            "CmdMapRotationTypeNone" => ::std::option::Option::Some(CmdMapRotationType::CmdMapRotationTypeNone),
            "CmdLeaveMapRotationRegionScNotify" => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionScNotify),
            "CmdEnterMapRotationRegionCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdEnterMapRotationRegionCsReq),
            "CmdResetMapRotationRegionCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdResetMapRotationRegionCsReq),
            "CmdRotateMapCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdRotateMapCsReq),
            "CmdLeaveMapRotationRegionCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionCsReq),
            "CmdRemoveRotaterScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdRemoveRotaterScRsp),
            "CmdUpdateMapRotationDataScNotify" => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateMapRotationDataScNotify),
            "CmdDeployRotaterCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdDeployRotaterCsReq),
            "CmdLeaveMapRotationRegionScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionScRsp),
            "CmdInteractChargerScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdInteractChargerScRsp),
            "CmdEnterMapRotationRegionScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdEnterMapRotationRegionScRsp),
            "CmdDeployRotaterScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdDeployRotaterScRsp),
            "CmdUpdateEnergyScNotify" => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateEnergyScNotify),
            "CmdResetMapRotationRegionScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdResetMapRotationRegionScRsp),
            "CmdInteractChargerCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdInteractChargerCsReq),
            "CmdGetMapRotationDataCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdGetMapRotationDataCsReq),
            "CmdGetMapRotationDataScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdGetMapRotationDataScRsp),
            "CmdRemoveRotaterCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdRemoveRotaterCsReq),
            "CmdRotateMapScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdRotateMapScRsp),
            "CmdUpdateRotaterScNotify" => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateRotaterScNotify),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdMapRotationType] = &[
        CmdMapRotationType::CmdMapRotationTypeNone,
        CmdMapRotationType::CmdLeaveMapRotationRegionScNotify,
        CmdMapRotationType::CmdEnterMapRotationRegionCsReq,
        CmdMapRotationType::CmdResetMapRotationRegionCsReq,
        CmdMapRotationType::CmdRotateMapCsReq,
        CmdMapRotationType::CmdLeaveMapRotationRegionCsReq,
        CmdMapRotationType::CmdRemoveRotaterScRsp,
        CmdMapRotationType::CmdUpdateMapRotationDataScNotify,
        CmdMapRotationType::CmdDeployRotaterCsReq,
        CmdMapRotationType::CmdLeaveMapRotationRegionScRsp,
        CmdMapRotationType::CmdInteractChargerScRsp,
        CmdMapRotationType::CmdEnterMapRotationRegionScRsp,
        CmdMapRotationType::CmdDeployRotaterScRsp,
        CmdMapRotationType::CmdUpdateEnergyScNotify,
        CmdMapRotationType::CmdResetMapRotationRegionScRsp,
        CmdMapRotationType::CmdInteractChargerCsReq,
        CmdMapRotationType::CmdGetMapRotationDataCsReq,
        CmdMapRotationType::CmdGetMapRotationDataScRsp,
        CmdMapRotationType::CmdRemoveRotaterCsReq,
        CmdMapRotationType::CmdRotateMapScRsp,
        CmdMapRotationType::CmdUpdateRotaterScNotify,
    ];
}

impl ::protobuf::EnumFull for CmdMapRotationType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdMapRotationType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdMapRotationType::CmdMapRotationTypeNone => 0,
            CmdMapRotationType::CmdLeaveMapRotationRegionScNotify => 1,
            CmdMapRotationType::CmdEnterMapRotationRegionCsReq => 2,
            CmdMapRotationType::CmdResetMapRotationRegionCsReq => 3,
            CmdMapRotationType::CmdRotateMapCsReq => 4,
            CmdMapRotationType::CmdLeaveMapRotationRegionCsReq => 5,
            CmdMapRotationType::CmdRemoveRotaterScRsp => 6,
            CmdMapRotationType::CmdUpdateMapRotationDataScNotify => 7,
            CmdMapRotationType::CmdDeployRotaterCsReq => 8,
            CmdMapRotationType::CmdLeaveMapRotationRegionScRsp => 9,
            CmdMapRotationType::CmdInteractChargerScRsp => 10,
            CmdMapRotationType::CmdEnterMapRotationRegionScRsp => 11,
            CmdMapRotationType::CmdDeployRotaterScRsp => 12,
            CmdMapRotationType::CmdUpdateEnergyScNotify => 13,
            CmdMapRotationType::CmdResetMapRotationRegionScRsp => 14,
            CmdMapRotationType::CmdInteractChargerCsReq => 15,
            CmdMapRotationType::CmdGetMapRotationDataCsReq => 16,
            CmdMapRotationType::CmdGetMapRotationDataScRsp => 17,
            CmdMapRotationType::CmdRemoveRotaterCsReq => 18,
            CmdMapRotationType::CmdRotateMapScRsp => 19,
            CmdMapRotationType::CmdUpdateRotaterScNotify => 20,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdMapRotationType {
    fn default() -> Self {
        CmdMapRotationType::CmdMapRotationTypeNone
    }
}

impl CmdMapRotationType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdMapRotationType>("CmdMapRotationType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18CmdMapRotationType.proto*\xb8\x05\n\x12CmdMapRotationType\x12\x1a\
    \n\x16CmdMapRotationTypeNone\x10\0\x12&\n!CmdLeaveMapRotationRegionScNot\
    ify\x10\xc05\x12#\n\x1eCmdEnterMapRotationRegionCsReq\x10\xcb5\x12#\n\
    \x1eCmdResetMapRotationRegionCsReq\x10\xae5\x12\x16\n\x11CmdRotateMapCsR\
    eq\x10\xb25\x12#\n\x1eCmdLeaveMapRotationRegionCsReq\x10\xe05\x12\x1a\n\
    \x15CmdRemoveRotaterScRsp\x10\xcd5\x12%\n\x20CmdUpdateMapRotationDataScN\
    otify\x10\xdf5\x12\x1a\n\x15CmdDeployRotaterCsReq\x10\xb75\x12#\n\x1eCmd\
    LeaveMapRotationRegionScRsp\x10\xa05\x12\x1c\n\x17CmdInteractChargerScRs\
    p\x10\xbe5\x12#\n\x1eCmdEnterMapRotationRegionScRsp\x10\xa45\x12\x1a\n\
    \x15CmdDeployRotaterScRsp\x10\xc55\x12\x1c\n\x17CmdUpdateEnergyScNotify\
    \x10\xea5\x12#\n\x1eCmdResetMapRotationRegionScRsp\x10\xdb5\x12\x1c\n\
    \x17CmdInteractChargerCsReq\x10\x935\x12\x1f\n\x1aCmdGetMapRotationDataC\
    sReq\x10\xbf5\x12\x1f\n\x1aCmdGetMapRotationDataScRsp\x10\xda5\x12\x1a\n\
    \x15CmdRemoveRotaterCsReq\x10\xa35\x12\x16\n\x11CmdRotateMapScRsp\x10\
    \xb55\x12\x1d\n\x18CmdUpdateRotaterScNotify\x10\xa95b\x06proto3\
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
            enums.push(CmdMapRotationType::generated_enum_descriptor_data());
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
