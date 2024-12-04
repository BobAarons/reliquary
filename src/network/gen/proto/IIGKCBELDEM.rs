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

//! Generated file from `IIGKCBELDEM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:IIGKCBELDEM)
pub enum IIGKCBELDEM {
    // @@protoc_insertion_point(enum_value:IIGKCBELDEM.ALLEY_EVENT_TYPE_NONE)
    ALLEY_EVENT_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:IIGKCBELDEM.ALLEY_MAIN_EVENT)
    ALLEY_MAIN_EVENT = 1,
    // @@protoc_insertion_point(enum_value:IIGKCBELDEM.ALLEY_CRITICAL_EVENT)
    ALLEY_CRITICAL_EVENT = 2,
    // @@protoc_insertion_point(enum_value:IIGKCBELDEM.ALLEY_DAILY_EVENT)
    ALLEY_DAILY_EVENT = 3,
}

impl ::protobuf::Enum for IIGKCBELDEM {
    const NAME: &'static str = "IIGKCBELDEM";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<IIGKCBELDEM> {
        match value {
            0 => ::std::option::Option::Some(IIGKCBELDEM::ALLEY_EVENT_TYPE_NONE),
            1 => ::std::option::Option::Some(IIGKCBELDEM::ALLEY_MAIN_EVENT),
            2 => ::std::option::Option::Some(IIGKCBELDEM::ALLEY_CRITICAL_EVENT),
            3 => ::std::option::Option::Some(IIGKCBELDEM::ALLEY_DAILY_EVENT),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<IIGKCBELDEM> {
        match str {
            "ALLEY_EVENT_TYPE_NONE" => ::std::option::Option::Some(IIGKCBELDEM::ALLEY_EVENT_TYPE_NONE),
            "ALLEY_MAIN_EVENT" => ::std::option::Option::Some(IIGKCBELDEM::ALLEY_MAIN_EVENT),
            "ALLEY_CRITICAL_EVENT" => ::std::option::Option::Some(IIGKCBELDEM::ALLEY_CRITICAL_EVENT),
            "ALLEY_DAILY_EVENT" => ::std::option::Option::Some(IIGKCBELDEM::ALLEY_DAILY_EVENT),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [IIGKCBELDEM] = &[
        IIGKCBELDEM::ALLEY_EVENT_TYPE_NONE,
        IIGKCBELDEM::ALLEY_MAIN_EVENT,
        IIGKCBELDEM::ALLEY_CRITICAL_EVENT,
        IIGKCBELDEM::ALLEY_DAILY_EVENT,
    ];
}

impl ::protobuf::EnumFull for IIGKCBELDEM {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("IIGKCBELDEM").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for IIGKCBELDEM {
    fn default() -> Self {
        IIGKCBELDEM::ALLEY_EVENT_TYPE_NONE
    }
}

impl IIGKCBELDEM {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<IIGKCBELDEM>("IIGKCBELDEM")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IIGKCBELDEM.proto*o\n\x0bIIGKCBELDEM\x12\x19\n\x15ALLEY_EVENT_TYPE\
    _NONE\x10\0\x12\x14\n\x10ALLEY_MAIN_EVENT\x10\x01\x12\x18\n\x14ALLEY_CRI\
    TICAL_EVENT\x10\x02\x12\x15\n\x11ALLEY_DAILY_EVENT\x10\x03b\x06proto3\
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
            enums.push(IIGKCBELDEM::generated_enum_descriptor_data());
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