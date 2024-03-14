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

//! Generated file from `SyncReason.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:SyncReason)
pub enum SyncReason {
    // @@protoc_insertion_point(enum_value:SyncReason.SYNC_REASON_NONE)
    SYNC_REASON_NONE = 0,
    // @@protoc_insertion_point(enum_value:SyncReason.SYNC_REASON_MP_ADD)
    SYNC_REASON_MP_ADD = 1,
    // @@protoc_insertion_point(enum_value:SyncReason.SYNC_REASON_MP_ADD_PROP_HIT)
    SYNC_REASON_MP_ADD_PROP_HIT = 2,
    // @@protoc_insertion_point(enum_value:SyncReason.SYNC_REASON_HP_ADD)
    SYNC_REASON_HP_ADD = 3,
    // @@protoc_insertion_point(enum_value:SyncReason.SYNC_REASON_HP_ADD_PROP_HIT)
    SYNC_REASON_HP_ADD_PROP_HIT = 4,
}

impl ::protobuf::Enum for SyncReason {
    const NAME: &'static str = "SyncReason";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SyncReason> {
        match value {
            0 => ::std::option::Option::Some(SyncReason::SYNC_REASON_NONE),
            1 => ::std::option::Option::Some(SyncReason::SYNC_REASON_MP_ADD),
            2 => ::std::option::Option::Some(SyncReason::SYNC_REASON_MP_ADD_PROP_HIT),
            3 => ::std::option::Option::Some(SyncReason::SYNC_REASON_HP_ADD),
            4 => ::std::option::Option::Some(SyncReason::SYNC_REASON_HP_ADD_PROP_HIT),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<SyncReason> {
        match str {
            "SYNC_REASON_NONE" => ::std::option::Option::Some(SyncReason::SYNC_REASON_NONE),
            "SYNC_REASON_MP_ADD" => ::std::option::Option::Some(SyncReason::SYNC_REASON_MP_ADD),
            "SYNC_REASON_MP_ADD_PROP_HIT" => ::std::option::Option::Some(SyncReason::SYNC_REASON_MP_ADD_PROP_HIT),
            "SYNC_REASON_HP_ADD" => ::std::option::Option::Some(SyncReason::SYNC_REASON_HP_ADD),
            "SYNC_REASON_HP_ADD_PROP_HIT" => ::std::option::Option::Some(SyncReason::SYNC_REASON_HP_ADD_PROP_HIT),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [SyncReason] = &[
        SyncReason::SYNC_REASON_NONE,
        SyncReason::SYNC_REASON_MP_ADD,
        SyncReason::SYNC_REASON_MP_ADD_PROP_HIT,
        SyncReason::SYNC_REASON_HP_ADD,
        SyncReason::SYNC_REASON_HP_ADD_PROP_HIT,
    ];
}

impl ::protobuf::EnumFull for SyncReason {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("SyncReason").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for SyncReason {
    fn default() -> Self {
        SyncReason::SYNC_REASON_NONE
    }
}

impl SyncReason {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<SyncReason>("SyncReason")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10SyncReason.proto*\x94\x01\n\nSyncReason\x12\x14\n\x10SYNC_REASON_N\
    ONE\x10\0\x12\x16\n\x12SYNC_REASON_MP_ADD\x10\x01\x12\x1f\n\x1bSYNC_REAS\
    ON_MP_ADD_PROP_HIT\x10\x02\x12\x16\n\x12SYNC_REASON_HP_ADD\x10\x03\x12\
    \x1f\n\x1bSYNC_REASON_HP_ADD_PROP_HIT\x10\x04B\x15\n\x13emu.lunarcore.pr\
    otob\x06proto3\
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
            enums.push(SyncReason::generated_enum_descriptor_data());
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