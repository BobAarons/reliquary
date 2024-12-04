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

//! Generated file from `CmdBoxingClubType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdBoxingClubType)
pub enum CmdBoxingClubType {
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdBoxingClubTypeNone)
    CmdBoxingClubTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdBoxingClubRewardScNotify)
    CmdBoxingClubRewardScNotify = 4280,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdMatchBoxingClubOpponentScRsp)
    CmdMatchBoxingClubOpponentScRsp = 4246,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdSetBoxingClubResonanceLineupScRsp)
    CmdSetBoxingClubResonanceLineupScRsp = 4275,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdStartBoxingClubBattleScRsp)
    CmdStartBoxingClubBattleScRsp = 4253,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdGetBoxingClubInfoScRsp)
    CmdGetBoxingClubInfoScRsp = 4220,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdChooseBoxingClubStageOptionalBuffCsReq)
    CmdChooseBoxingClubStageOptionalBuffCsReq = 4248,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdChooseBoxingClubResonanceScRsp)
    CmdChooseBoxingClubResonanceScRsp = 4274,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdGetBoxingClubInfoCsReq)
    CmdGetBoxingClubInfoCsReq = 4259,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdSetBoxingClubResonanceLineupCsReq)
    CmdSetBoxingClubResonanceLineupCsReq = 4230,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdStartBoxingClubBattleCsReq)
    CmdStartBoxingClubBattleCsReq = 4239,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdBoxingClubChallengeUpdateScNotify)
    CmdBoxingClubChallengeUpdateScNotify = 4216,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdMatchBoxingClubOpponentCsReq)
    CmdMatchBoxingClubOpponentCsReq = 4203,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdGiveUpBoxingClubChallengeScRsp)
    CmdGiveUpBoxingClubChallengeScRsp = 4237,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdChooseBoxingClubStageOptionalBuffScRsp)
    CmdChooseBoxingClubStageOptionalBuffScRsp = 4290,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdGiveUpBoxingClubChallengeCsReq)
    CmdGiveUpBoxingClubChallengeCsReq = 4234,
    // @@protoc_insertion_point(enum_value:CmdBoxingClubType.CmdChooseBoxingClubResonanceCsReq)
    CmdChooseBoxingClubResonanceCsReq = 4247,
}

impl ::protobuf::Enum for CmdBoxingClubType {
    const NAME: &'static str = "CmdBoxingClubType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdBoxingClubType> {
        match value {
            0 => ::std::option::Option::Some(CmdBoxingClubType::CmdBoxingClubTypeNone),
            4280 => ::std::option::Option::Some(CmdBoxingClubType::CmdBoxingClubRewardScNotify),
            4246 => ::std::option::Option::Some(CmdBoxingClubType::CmdMatchBoxingClubOpponentScRsp),
            4275 => ::std::option::Option::Some(CmdBoxingClubType::CmdSetBoxingClubResonanceLineupScRsp),
            4253 => ::std::option::Option::Some(CmdBoxingClubType::CmdStartBoxingClubBattleScRsp),
            4220 => ::std::option::Option::Some(CmdBoxingClubType::CmdGetBoxingClubInfoScRsp),
            4248 => ::std::option::Option::Some(CmdBoxingClubType::CmdChooseBoxingClubStageOptionalBuffCsReq),
            4274 => ::std::option::Option::Some(CmdBoxingClubType::CmdChooseBoxingClubResonanceScRsp),
            4259 => ::std::option::Option::Some(CmdBoxingClubType::CmdGetBoxingClubInfoCsReq),
            4230 => ::std::option::Option::Some(CmdBoxingClubType::CmdSetBoxingClubResonanceLineupCsReq),
            4239 => ::std::option::Option::Some(CmdBoxingClubType::CmdStartBoxingClubBattleCsReq),
            4216 => ::std::option::Option::Some(CmdBoxingClubType::CmdBoxingClubChallengeUpdateScNotify),
            4203 => ::std::option::Option::Some(CmdBoxingClubType::CmdMatchBoxingClubOpponentCsReq),
            4237 => ::std::option::Option::Some(CmdBoxingClubType::CmdGiveUpBoxingClubChallengeScRsp),
            4290 => ::std::option::Option::Some(CmdBoxingClubType::CmdChooseBoxingClubStageOptionalBuffScRsp),
            4234 => ::std::option::Option::Some(CmdBoxingClubType::CmdGiveUpBoxingClubChallengeCsReq),
            4247 => ::std::option::Option::Some(CmdBoxingClubType::CmdChooseBoxingClubResonanceCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdBoxingClubType> {
        match str {
            "CmdBoxingClubTypeNone" => ::std::option::Option::Some(CmdBoxingClubType::CmdBoxingClubTypeNone),
            "CmdBoxingClubRewardScNotify" => ::std::option::Option::Some(CmdBoxingClubType::CmdBoxingClubRewardScNotify),
            "CmdMatchBoxingClubOpponentScRsp" => ::std::option::Option::Some(CmdBoxingClubType::CmdMatchBoxingClubOpponentScRsp),
            "CmdSetBoxingClubResonanceLineupScRsp" => ::std::option::Option::Some(CmdBoxingClubType::CmdSetBoxingClubResonanceLineupScRsp),
            "CmdStartBoxingClubBattleScRsp" => ::std::option::Option::Some(CmdBoxingClubType::CmdStartBoxingClubBattleScRsp),
            "CmdGetBoxingClubInfoScRsp" => ::std::option::Option::Some(CmdBoxingClubType::CmdGetBoxingClubInfoScRsp),
            "CmdChooseBoxingClubStageOptionalBuffCsReq" => ::std::option::Option::Some(CmdBoxingClubType::CmdChooseBoxingClubStageOptionalBuffCsReq),
            "CmdChooseBoxingClubResonanceScRsp" => ::std::option::Option::Some(CmdBoxingClubType::CmdChooseBoxingClubResonanceScRsp),
            "CmdGetBoxingClubInfoCsReq" => ::std::option::Option::Some(CmdBoxingClubType::CmdGetBoxingClubInfoCsReq),
            "CmdSetBoxingClubResonanceLineupCsReq" => ::std::option::Option::Some(CmdBoxingClubType::CmdSetBoxingClubResonanceLineupCsReq),
            "CmdStartBoxingClubBattleCsReq" => ::std::option::Option::Some(CmdBoxingClubType::CmdStartBoxingClubBattleCsReq),
            "CmdBoxingClubChallengeUpdateScNotify" => ::std::option::Option::Some(CmdBoxingClubType::CmdBoxingClubChallengeUpdateScNotify),
            "CmdMatchBoxingClubOpponentCsReq" => ::std::option::Option::Some(CmdBoxingClubType::CmdMatchBoxingClubOpponentCsReq),
            "CmdGiveUpBoxingClubChallengeScRsp" => ::std::option::Option::Some(CmdBoxingClubType::CmdGiveUpBoxingClubChallengeScRsp),
            "CmdChooseBoxingClubStageOptionalBuffScRsp" => ::std::option::Option::Some(CmdBoxingClubType::CmdChooseBoxingClubStageOptionalBuffScRsp),
            "CmdGiveUpBoxingClubChallengeCsReq" => ::std::option::Option::Some(CmdBoxingClubType::CmdGiveUpBoxingClubChallengeCsReq),
            "CmdChooseBoxingClubResonanceCsReq" => ::std::option::Option::Some(CmdBoxingClubType::CmdChooseBoxingClubResonanceCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdBoxingClubType] = &[
        CmdBoxingClubType::CmdBoxingClubTypeNone,
        CmdBoxingClubType::CmdBoxingClubRewardScNotify,
        CmdBoxingClubType::CmdMatchBoxingClubOpponentScRsp,
        CmdBoxingClubType::CmdSetBoxingClubResonanceLineupScRsp,
        CmdBoxingClubType::CmdStartBoxingClubBattleScRsp,
        CmdBoxingClubType::CmdGetBoxingClubInfoScRsp,
        CmdBoxingClubType::CmdChooseBoxingClubStageOptionalBuffCsReq,
        CmdBoxingClubType::CmdChooseBoxingClubResonanceScRsp,
        CmdBoxingClubType::CmdGetBoxingClubInfoCsReq,
        CmdBoxingClubType::CmdSetBoxingClubResonanceLineupCsReq,
        CmdBoxingClubType::CmdStartBoxingClubBattleCsReq,
        CmdBoxingClubType::CmdBoxingClubChallengeUpdateScNotify,
        CmdBoxingClubType::CmdMatchBoxingClubOpponentCsReq,
        CmdBoxingClubType::CmdGiveUpBoxingClubChallengeScRsp,
        CmdBoxingClubType::CmdChooseBoxingClubStageOptionalBuffScRsp,
        CmdBoxingClubType::CmdGiveUpBoxingClubChallengeCsReq,
        CmdBoxingClubType::CmdChooseBoxingClubResonanceCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdBoxingClubType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdBoxingClubType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdBoxingClubType::CmdBoxingClubTypeNone => 0,
            CmdBoxingClubType::CmdBoxingClubRewardScNotify => 1,
            CmdBoxingClubType::CmdMatchBoxingClubOpponentScRsp => 2,
            CmdBoxingClubType::CmdSetBoxingClubResonanceLineupScRsp => 3,
            CmdBoxingClubType::CmdStartBoxingClubBattleScRsp => 4,
            CmdBoxingClubType::CmdGetBoxingClubInfoScRsp => 5,
            CmdBoxingClubType::CmdChooseBoxingClubStageOptionalBuffCsReq => 6,
            CmdBoxingClubType::CmdChooseBoxingClubResonanceScRsp => 7,
            CmdBoxingClubType::CmdGetBoxingClubInfoCsReq => 8,
            CmdBoxingClubType::CmdSetBoxingClubResonanceLineupCsReq => 9,
            CmdBoxingClubType::CmdStartBoxingClubBattleCsReq => 10,
            CmdBoxingClubType::CmdBoxingClubChallengeUpdateScNotify => 11,
            CmdBoxingClubType::CmdMatchBoxingClubOpponentCsReq => 12,
            CmdBoxingClubType::CmdGiveUpBoxingClubChallengeScRsp => 13,
            CmdBoxingClubType::CmdChooseBoxingClubStageOptionalBuffScRsp => 14,
            CmdBoxingClubType::CmdGiveUpBoxingClubChallengeCsReq => 15,
            CmdBoxingClubType::CmdChooseBoxingClubResonanceCsReq => 16,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdBoxingClubType {
    fn default() -> Self {
        CmdBoxingClubType::CmdBoxingClubTypeNone
    }
}

impl CmdBoxingClubType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdBoxingClubType>("CmdBoxingClubType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17CmdBoxingClubType.proto*\xa5\x05\n\x11CmdBoxingClubType\x12\x19\n\
    \x15CmdBoxingClubTypeNone\x10\0\x12\x20\n\x1bCmdBoxingClubRewardScNotify\
    \x10\xb8!\x12$\n\x1fCmdMatchBoxingClubOpponentScRsp\x10\x96!\x12)\n$CmdS\
    etBoxingClubResonanceLineupScRsp\x10\xb3!\x12\"\n\x1dCmdStartBoxingClubB\
    attleScRsp\x10\x9d!\x12\x1e\n\x19CmdGetBoxingClubInfoScRsp\x10\xfc\x20\
    \x12.\n)CmdChooseBoxingClubStageOptionalBuffCsReq\x10\x98!\x12&\n!CmdCho\
    oseBoxingClubResonanceScRsp\x10\xb2!\x12\x1e\n\x19CmdGetBoxingClubInfoCs\
    Req\x10\xa3!\x12)\n$CmdSetBoxingClubResonanceLineupCsReq\x10\x86!\x12\"\
    \n\x1dCmdStartBoxingClubBattleCsReq\x10\x8f!\x12)\n$CmdBoxingClubChallen\
    geUpdateScNotify\x10\xf8\x20\x12$\n\x1fCmdMatchBoxingClubOpponentCsReq\
    \x10\xeb\x20\x12&\n!CmdGiveUpBoxingClubChallengeScRsp\x10\x8d!\x12.\n)Cm\
    dChooseBoxingClubStageOptionalBuffScRsp\x10\xc2!\x12&\n!CmdGiveUpBoxingC\
    lubChallengeCsReq\x10\x8a!\x12&\n!CmdChooseBoxingClubResonanceCsReq\x10\
    \x97!b\x06proto3\
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
            enums.push(CmdBoxingClubType::generated_enum_descriptor_data());
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