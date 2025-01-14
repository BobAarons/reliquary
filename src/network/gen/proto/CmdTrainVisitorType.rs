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

//! Generated file from `CmdTrainVisitorType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdTrainVisitorType)
pub enum CmdTrainVisitorType {
    // @@protoc_insertion_point(enum_value:CmdTrainVisitorType.CmdTrainVisitorTypeNone)
    CmdTrainVisitorTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdTrainVisitorType.CmdShowNewSupplementVisitorScRsp)
    CmdShowNewSupplementVisitorScRsp = 3774,
    // @@protoc_insertion_point(enum_value:CmdTrainVisitorType.CmdTrainVisitorBehaviorFinishScRsp)
    CmdTrainVisitorBehaviorFinishScRsp = 3720,
    // @@protoc_insertion_point(enum_value:CmdTrainVisitorType.CmdGetTrainVisitorRegisterCsReq)
    CmdGetTrainVisitorRegisterCsReq = 3734,
    // @@protoc_insertion_point(enum_value:CmdTrainVisitorType.CmdTrainVisitorBehaviorFinishCsReq)
    CmdTrainVisitorBehaviorFinishCsReq = 3759,
    // @@protoc_insertion_point(enum_value:CmdTrainVisitorType.CmdTakeTrainVisitorUntakenBehaviorRewardCsReq)
    CmdTakeTrainVisitorUntakenBehaviorRewardCsReq = 3780,
    // @@protoc_insertion_point(enum_value:CmdTrainVisitorType.CmdGetTrainVisitorRegisterScRsp)
    CmdGetTrainVisitorRegisterScRsp = 3737,
    // @@protoc_insertion_point(enum_value:CmdTrainVisitorType.CmdTrainVisitorRewardSendNotify)
    CmdTrainVisitorRewardSendNotify = 3753,
    // @@protoc_insertion_point(enum_value:CmdTrainVisitorType.CmdTrainRefreshTimeNotify)
    CmdTrainRefreshTimeNotify = 3739,
    // @@protoc_insertion_point(enum_value:CmdTrainVisitorType.CmdGetTrainVisitorBehaviorScRsp)
    CmdGetTrainVisitorBehaviorScRsp = 3746,
    // @@protoc_insertion_point(enum_value:CmdTrainVisitorType.CmdGetTrainVisitorBehaviorCsReq)
    CmdGetTrainVisitorBehaviorCsReq = 3703,
    // @@protoc_insertion_point(enum_value:CmdTrainVisitorType.CmdShowNewSupplementVisitorCsReq)
    CmdShowNewSupplementVisitorCsReq = 3747,
    // @@protoc_insertion_point(enum_value:CmdTrainVisitorType.CmdTakeTrainVisitorUntakenBehaviorRewardScRsp)
    CmdTakeTrainVisitorUntakenBehaviorRewardScRsp = 3716,
}

impl ::protobuf::Enum for CmdTrainVisitorType {
    const NAME: &'static str = "CmdTrainVisitorType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdTrainVisitorType> {
        match value {
            0 => ::std::option::Option::Some(CmdTrainVisitorType::CmdTrainVisitorTypeNone),
            3774 => ::std::option::Option::Some(CmdTrainVisitorType::CmdShowNewSupplementVisitorScRsp),
            3720 => ::std::option::Option::Some(CmdTrainVisitorType::CmdTrainVisitorBehaviorFinishScRsp),
            3734 => ::std::option::Option::Some(CmdTrainVisitorType::CmdGetTrainVisitorRegisterCsReq),
            3759 => ::std::option::Option::Some(CmdTrainVisitorType::CmdTrainVisitorBehaviorFinishCsReq),
            3780 => ::std::option::Option::Some(CmdTrainVisitorType::CmdTakeTrainVisitorUntakenBehaviorRewardCsReq),
            3737 => ::std::option::Option::Some(CmdTrainVisitorType::CmdGetTrainVisitorRegisterScRsp),
            3753 => ::std::option::Option::Some(CmdTrainVisitorType::CmdTrainVisitorRewardSendNotify),
            3739 => ::std::option::Option::Some(CmdTrainVisitorType::CmdTrainRefreshTimeNotify),
            3746 => ::std::option::Option::Some(CmdTrainVisitorType::CmdGetTrainVisitorBehaviorScRsp),
            3703 => ::std::option::Option::Some(CmdTrainVisitorType::CmdGetTrainVisitorBehaviorCsReq),
            3747 => ::std::option::Option::Some(CmdTrainVisitorType::CmdShowNewSupplementVisitorCsReq),
            3716 => ::std::option::Option::Some(CmdTrainVisitorType::CmdTakeTrainVisitorUntakenBehaviorRewardScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdTrainVisitorType> {
        match str {
            "CmdTrainVisitorTypeNone" => ::std::option::Option::Some(CmdTrainVisitorType::CmdTrainVisitorTypeNone),
            "CmdShowNewSupplementVisitorScRsp" => ::std::option::Option::Some(CmdTrainVisitorType::CmdShowNewSupplementVisitorScRsp),
            "CmdTrainVisitorBehaviorFinishScRsp" => ::std::option::Option::Some(CmdTrainVisitorType::CmdTrainVisitorBehaviorFinishScRsp),
            "CmdGetTrainVisitorRegisterCsReq" => ::std::option::Option::Some(CmdTrainVisitorType::CmdGetTrainVisitorRegisterCsReq),
            "CmdTrainVisitorBehaviorFinishCsReq" => ::std::option::Option::Some(CmdTrainVisitorType::CmdTrainVisitorBehaviorFinishCsReq),
            "CmdTakeTrainVisitorUntakenBehaviorRewardCsReq" => ::std::option::Option::Some(CmdTrainVisitorType::CmdTakeTrainVisitorUntakenBehaviorRewardCsReq),
            "CmdGetTrainVisitorRegisterScRsp" => ::std::option::Option::Some(CmdTrainVisitorType::CmdGetTrainVisitorRegisterScRsp),
            "CmdTrainVisitorRewardSendNotify" => ::std::option::Option::Some(CmdTrainVisitorType::CmdTrainVisitorRewardSendNotify),
            "CmdTrainRefreshTimeNotify" => ::std::option::Option::Some(CmdTrainVisitorType::CmdTrainRefreshTimeNotify),
            "CmdGetTrainVisitorBehaviorScRsp" => ::std::option::Option::Some(CmdTrainVisitorType::CmdGetTrainVisitorBehaviorScRsp),
            "CmdGetTrainVisitorBehaviorCsReq" => ::std::option::Option::Some(CmdTrainVisitorType::CmdGetTrainVisitorBehaviorCsReq),
            "CmdShowNewSupplementVisitorCsReq" => ::std::option::Option::Some(CmdTrainVisitorType::CmdShowNewSupplementVisitorCsReq),
            "CmdTakeTrainVisitorUntakenBehaviorRewardScRsp" => ::std::option::Option::Some(CmdTrainVisitorType::CmdTakeTrainVisitorUntakenBehaviorRewardScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdTrainVisitorType] = &[
        CmdTrainVisitorType::CmdTrainVisitorTypeNone,
        CmdTrainVisitorType::CmdShowNewSupplementVisitorScRsp,
        CmdTrainVisitorType::CmdTrainVisitorBehaviorFinishScRsp,
        CmdTrainVisitorType::CmdGetTrainVisitorRegisterCsReq,
        CmdTrainVisitorType::CmdTrainVisitorBehaviorFinishCsReq,
        CmdTrainVisitorType::CmdTakeTrainVisitorUntakenBehaviorRewardCsReq,
        CmdTrainVisitorType::CmdGetTrainVisitorRegisterScRsp,
        CmdTrainVisitorType::CmdTrainVisitorRewardSendNotify,
        CmdTrainVisitorType::CmdTrainRefreshTimeNotify,
        CmdTrainVisitorType::CmdGetTrainVisitorBehaviorScRsp,
        CmdTrainVisitorType::CmdGetTrainVisitorBehaviorCsReq,
        CmdTrainVisitorType::CmdShowNewSupplementVisitorCsReq,
        CmdTrainVisitorType::CmdTakeTrainVisitorUntakenBehaviorRewardScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdTrainVisitorType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdTrainVisitorType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdTrainVisitorType::CmdTrainVisitorTypeNone => 0,
            CmdTrainVisitorType::CmdShowNewSupplementVisitorScRsp => 1,
            CmdTrainVisitorType::CmdTrainVisitorBehaviorFinishScRsp => 2,
            CmdTrainVisitorType::CmdGetTrainVisitorRegisterCsReq => 3,
            CmdTrainVisitorType::CmdTrainVisitorBehaviorFinishCsReq => 4,
            CmdTrainVisitorType::CmdTakeTrainVisitorUntakenBehaviorRewardCsReq => 5,
            CmdTrainVisitorType::CmdGetTrainVisitorRegisterScRsp => 6,
            CmdTrainVisitorType::CmdTrainVisitorRewardSendNotify => 7,
            CmdTrainVisitorType::CmdTrainRefreshTimeNotify => 8,
            CmdTrainVisitorType::CmdGetTrainVisitorBehaviorScRsp => 9,
            CmdTrainVisitorType::CmdGetTrainVisitorBehaviorCsReq => 10,
            CmdTrainVisitorType::CmdShowNewSupplementVisitorCsReq => 11,
            CmdTrainVisitorType::CmdTakeTrainVisitorUntakenBehaviorRewardScRsp => 12,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdTrainVisitorType {
    fn default() -> Self {
        CmdTrainVisitorType::CmdTrainVisitorTypeNone
    }
}

impl CmdTrainVisitorType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdTrainVisitorType>("CmdTrainVisitorType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19CmdTrainVisitorType.proto*\x98\x04\n\x13CmdTrainVisitorType\x12\
    \x1b\n\x17CmdTrainVisitorTypeNone\x10\0\x12%\n\x20CmdShowNewSupplementVi\
    sitorScRsp\x10\xbe\x1d\x12'\n\"CmdTrainVisitorBehaviorFinishScRsp\x10\
    \x88\x1d\x12$\n\x1fCmdGetTrainVisitorRegisterCsReq\x10\x96\x1d\x12'\n\"C\
    mdTrainVisitorBehaviorFinishCsReq\x10\xaf\x1d\x122\n-CmdTakeTrainVisitor\
    UntakenBehaviorRewardCsReq\x10\xc4\x1d\x12$\n\x1fCmdGetTrainVisitorRegis\
    terScRsp\x10\x99\x1d\x12$\n\x1fCmdTrainVisitorRewardSendNotify\x10\xa9\
    \x1d\x12\x1e\n\x19CmdTrainRefreshTimeNotify\x10\x9b\x1d\x12$\n\x1fCmdGet\
    TrainVisitorBehaviorScRsp\x10\xa2\x1d\x12$\n\x1fCmdGetTrainVisitorBehavi\
    orCsReq\x10\xf7\x1c\x12%\n\x20CmdShowNewSupplementVisitorCsReq\x10\xa3\
    \x1d\x122\n-CmdTakeTrainVisitorUntakenBehaviorRewardScRsp\x10\x84\x1db\
    \x06proto3\
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
            enums.push(CmdTrainVisitorType::generated_enum_descriptor_data());
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
