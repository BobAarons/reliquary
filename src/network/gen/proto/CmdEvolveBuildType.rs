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

//! Generated file from `CmdEvolveBuildType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdEvolveBuildType)
pub enum CmdEvolveBuildType {
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildNone)
    CmdEvolveBuildNone = 0,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityResetScRsp)
    CmdEvolveBuildShopAbilityResetScRsp = 7132,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildLeaveCsReq)
    CmdEvolveBuildLeaveCsReq = 7108,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildCoinNotify)
    CmdEvolveBuildCoinNotify = 7119,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildQueryInfoScRsp)
    CmdEvolveBuildQueryInfoScRsp = 7116,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildLeaveScRsp)
    CmdEvolveBuildLeaveScRsp = 7130,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityResetCsReq)
    CmdEvolveBuildShopAbilityResetCsReq = 7118,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildStartStageScRsp)
    CmdEvolveBuildStartStageScRsp = 7137,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildTakeExpRewardCsReq)
    CmdEvolveBuildTakeExpRewardCsReq = 7105,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityDownCsReq)
    CmdEvolveBuildShopAbilityDownCsReq = 7144,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildStartStageCsReq)
    CmdEvolveBuildStartStageCsReq = 7142,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityUpScRsp)
    CmdEvolveBuildShopAbilityUpScRsp = 7133,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildUnlockInfoNotify)
    CmdEvolveBuildUnlockInfoNotify = 7126,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildFinishScNotify)
    CmdEvolveBuildFinishScNotify = 7135,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityUpCsReq)
    CmdEvolveBuildShopAbilityUpCsReq = 7139,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildQueryInfoCsReq)
    CmdEvolveBuildQueryInfoCsReq = 7115,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildReRandomStageCsReq)
    CmdEvolveBuildReRandomStageCsReq = 7128,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildReRandomStageScRsp)
    CmdEvolveBuildReRandomStageScRsp = 7107,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildStartLevelCsReq)
    CmdEvolveBuildStartLevelCsReq = 7112,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildStartLevelScRsp)
    CmdEvolveBuildStartLevelScRsp = 7124,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildGiveupCsReq)
    CmdEvolveBuildGiveupCsReq = 7140,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildTakeExpRewardScRsp)
    CmdEvolveBuildTakeExpRewardScRsp = 7131,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityDownScRsp)
    CmdEvolveBuildShopAbilityDownScRsp = 7120,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildGiveupScRsp)
    CmdEvolveBuildGiveupScRsp = 7123,
}

impl ::protobuf::Enum for CmdEvolveBuildType {
    const NAME: &'static str = "CmdEvolveBuildType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdEvolveBuildType> {
        match value {
            0 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildNone),
            7132 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetScRsp),
            7108 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildLeaveCsReq),
            7119 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildCoinNotify),
            7116 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildQueryInfoScRsp),
            7130 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildLeaveScRsp),
            7118 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetCsReq),
            7137 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartStageScRsp),
            7105 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardCsReq),
            7144 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownCsReq),
            7142 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartStageCsReq),
            7133 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpScRsp),
            7126 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildUnlockInfoNotify),
            7135 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildFinishScNotify),
            7139 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpCsReq),
            7115 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildQueryInfoCsReq),
            7128 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildReRandomStageCsReq),
            7107 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildReRandomStageScRsp),
            7112 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartLevelCsReq),
            7124 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartLevelScRsp),
            7140 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildGiveupCsReq),
            7131 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardScRsp),
            7120 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownScRsp),
            7123 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildGiveupScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdEvolveBuildType> {
        match str {
            "CmdEvolveBuildNone" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildNone),
            "CmdEvolveBuildShopAbilityResetScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetScRsp),
            "CmdEvolveBuildLeaveCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildLeaveCsReq),
            "CmdEvolveBuildCoinNotify" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildCoinNotify),
            "CmdEvolveBuildQueryInfoScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildQueryInfoScRsp),
            "CmdEvolveBuildLeaveScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildLeaveScRsp),
            "CmdEvolveBuildShopAbilityResetCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetCsReq),
            "CmdEvolveBuildStartStageScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartStageScRsp),
            "CmdEvolveBuildTakeExpRewardCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardCsReq),
            "CmdEvolveBuildShopAbilityDownCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownCsReq),
            "CmdEvolveBuildStartStageCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartStageCsReq),
            "CmdEvolveBuildShopAbilityUpScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpScRsp),
            "CmdEvolveBuildUnlockInfoNotify" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildUnlockInfoNotify),
            "CmdEvolveBuildFinishScNotify" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildFinishScNotify),
            "CmdEvolveBuildShopAbilityUpCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpCsReq),
            "CmdEvolveBuildQueryInfoCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildQueryInfoCsReq),
            "CmdEvolveBuildReRandomStageCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildReRandomStageCsReq),
            "CmdEvolveBuildReRandomStageScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildReRandomStageScRsp),
            "CmdEvolveBuildStartLevelCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartLevelCsReq),
            "CmdEvolveBuildStartLevelScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartLevelScRsp),
            "CmdEvolveBuildGiveupCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildGiveupCsReq),
            "CmdEvolveBuildTakeExpRewardScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardScRsp),
            "CmdEvolveBuildShopAbilityDownScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownScRsp),
            "CmdEvolveBuildGiveupScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildGiveupScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdEvolveBuildType] = &[
        CmdEvolveBuildType::CmdEvolveBuildNone,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetScRsp,
        CmdEvolveBuildType::CmdEvolveBuildLeaveCsReq,
        CmdEvolveBuildType::CmdEvolveBuildCoinNotify,
        CmdEvolveBuildType::CmdEvolveBuildQueryInfoScRsp,
        CmdEvolveBuildType::CmdEvolveBuildLeaveScRsp,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetCsReq,
        CmdEvolveBuildType::CmdEvolveBuildStartStageScRsp,
        CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardCsReq,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownCsReq,
        CmdEvolveBuildType::CmdEvolveBuildStartStageCsReq,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpScRsp,
        CmdEvolveBuildType::CmdEvolveBuildUnlockInfoNotify,
        CmdEvolveBuildType::CmdEvolveBuildFinishScNotify,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpCsReq,
        CmdEvolveBuildType::CmdEvolveBuildQueryInfoCsReq,
        CmdEvolveBuildType::CmdEvolveBuildReRandomStageCsReq,
        CmdEvolveBuildType::CmdEvolveBuildReRandomStageScRsp,
        CmdEvolveBuildType::CmdEvolveBuildStartLevelCsReq,
        CmdEvolveBuildType::CmdEvolveBuildStartLevelScRsp,
        CmdEvolveBuildType::CmdEvolveBuildGiveupCsReq,
        CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardScRsp,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownScRsp,
        CmdEvolveBuildType::CmdEvolveBuildGiveupScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdEvolveBuildType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdEvolveBuildType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdEvolveBuildType::CmdEvolveBuildNone => 0,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetScRsp => 1,
            CmdEvolveBuildType::CmdEvolveBuildLeaveCsReq => 2,
            CmdEvolveBuildType::CmdEvolveBuildCoinNotify => 3,
            CmdEvolveBuildType::CmdEvolveBuildQueryInfoScRsp => 4,
            CmdEvolveBuildType::CmdEvolveBuildLeaveScRsp => 5,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetCsReq => 6,
            CmdEvolveBuildType::CmdEvolveBuildStartStageScRsp => 7,
            CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardCsReq => 8,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownCsReq => 9,
            CmdEvolveBuildType::CmdEvolveBuildStartStageCsReq => 10,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpScRsp => 11,
            CmdEvolveBuildType::CmdEvolveBuildUnlockInfoNotify => 12,
            CmdEvolveBuildType::CmdEvolveBuildFinishScNotify => 13,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpCsReq => 14,
            CmdEvolveBuildType::CmdEvolveBuildQueryInfoCsReq => 15,
            CmdEvolveBuildType::CmdEvolveBuildReRandomStageCsReq => 16,
            CmdEvolveBuildType::CmdEvolveBuildReRandomStageScRsp => 17,
            CmdEvolveBuildType::CmdEvolveBuildStartLevelCsReq => 18,
            CmdEvolveBuildType::CmdEvolveBuildStartLevelScRsp => 19,
            CmdEvolveBuildType::CmdEvolveBuildGiveupCsReq => 20,
            CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardScRsp => 21,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownScRsp => 22,
            CmdEvolveBuildType::CmdEvolveBuildGiveupScRsp => 23,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdEvolveBuildType {
    fn default() -> Self {
        CmdEvolveBuildType::CmdEvolveBuildNone
    }
}

impl CmdEvolveBuildType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdEvolveBuildType>("CmdEvolveBuildType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18CmdEvolveBuildType.proto*\xf7\x06\n\x12CmdEvolveBuildType\x12\x16\
    \n\x12CmdEvolveBuildNone\x10\0\x12(\n#CmdEvolveBuildShopAbilityResetScRs\
    p\x10\xdc7\x12\x1d\n\x18CmdEvolveBuildLeaveCsReq\x10\xc47\x12\x1d\n\x18C\
    mdEvolveBuildCoinNotify\x10\xcf7\x12!\n\x1cCmdEvolveBuildQueryInfoScRsp\
    \x10\xcc7\x12\x1d\n\x18CmdEvolveBuildLeaveScRsp\x10\xda7\x12(\n#CmdEvolv\
    eBuildShopAbilityResetCsReq\x10\xce7\x12\"\n\x1dCmdEvolveBuildStartStage\
    ScRsp\x10\xe17\x12%\n\x20CmdEvolveBuildTakeExpRewardCsReq\x10\xc17\x12'\
    \n\"CmdEvolveBuildShopAbilityDownCsReq\x10\xe87\x12\"\n\x1dCmdEvolveBuil\
    dStartStageCsReq\x10\xe67\x12%\n\x20CmdEvolveBuildShopAbilityUpScRsp\x10\
    \xdd7\x12#\n\x1eCmdEvolveBuildUnlockInfoNotify\x10\xd67\x12!\n\x1cCmdEvo\
    lveBuildFinishScNotify\x10\xdf7\x12%\n\x20CmdEvolveBuildShopAbilityUpCsR\
    eq\x10\xe37\x12!\n\x1cCmdEvolveBuildQueryInfoCsReq\x10\xcb7\x12%\n\x20Cm\
    dEvolveBuildReRandomStageCsReq\x10\xd87\x12%\n\x20CmdEvolveBuildReRandom\
    StageScRsp\x10\xc37\x12\"\n\x1dCmdEvolveBuildStartLevelCsReq\x10\xc87\
    \x12\"\n\x1dCmdEvolveBuildStartLevelScRsp\x10\xd47\x12\x1e\n\x19CmdEvolv\
    eBuildGiveupCsReq\x10\xe47\x12%\n\x20CmdEvolveBuildTakeExpRewardScRsp\
    \x10\xdb7\x12'\n\"CmdEvolveBuildShopAbilityDownScRsp\x10\xd07\x12\x1e\n\
    \x19CmdEvolveBuildGiveupScRsp\x10\xd37b\x06proto3\
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
            enums.push(CmdEvolveBuildType::generated_enum_descriptor_data());
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