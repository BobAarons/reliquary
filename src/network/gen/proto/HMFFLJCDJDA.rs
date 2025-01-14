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

//! Generated file from `HMFFLJCDJDA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:HMFFLJCDJDA)
pub enum HMFFLJCDJDA {
    // @@protoc_insertion_point(enum_value:HMFFLJCDJDA.LEFT)
    LEFT = 0,
    // @@protoc_insertion_point(enum_value:HMFFLJCDJDA.RIGHT)
    RIGHT = 1,
    // @@protoc_insertion_point(enum_value:HMFFLJCDJDA.UP)
    UP = 2,
    // @@protoc_insertion_point(enum_value:HMFFLJCDJDA.DOWN)
    DOWN = 3,
    // @@protoc_insertion_point(enum_value:HMFFLJCDJDA.LEFT_UP)
    LEFT_UP = 4,
    // @@protoc_insertion_point(enum_value:HMFFLJCDJDA.LEFT_DOWN)
    LEFT_DOWN = 5,
    // @@protoc_insertion_point(enum_value:HMFFLJCDJDA.RIGHT_UP)
    RIGHT_UP = 6,
    // @@protoc_insertion_point(enum_value:HMFFLJCDJDA.RIGHT_DOWN)
    RIGHT_DOWN = 7,
}

impl ::protobuf::Enum for HMFFLJCDJDA {
    const NAME: &'static str = "HMFFLJCDJDA";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HMFFLJCDJDA> {
        match value {
            0 => ::std::option::Option::Some(HMFFLJCDJDA::LEFT),
            1 => ::std::option::Option::Some(HMFFLJCDJDA::RIGHT),
            2 => ::std::option::Option::Some(HMFFLJCDJDA::UP),
            3 => ::std::option::Option::Some(HMFFLJCDJDA::DOWN),
            4 => ::std::option::Option::Some(HMFFLJCDJDA::LEFT_UP),
            5 => ::std::option::Option::Some(HMFFLJCDJDA::LEFT_DOWN),
            6 => ::std::option::Option::Some(HMFFLJCDJDA::RIGHT_UP),
            7 => ::std::option::Option::Some(HMFFLJCDJDA::RIGHT_DOWN),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<HMFFLJCDJDA> {
        match str {
            "LEFT" => ::std::option::Option::Some(HMFFLJCDJDA::LEFT),
            "RIGHT" => ::std::option::Option::Some(HMFFLJCDJDA::RIGHT),
            "UP" => ::std::option::Option::Some(HMFFLJCDJDA::UP),
            "DOWN" => ::std::option::Option::Some(HMFFLJCDJDA::DOWN),
            "LEFT_UP" => ::std::option::Option::Some(HMFFLJCDJDA::LEFT_UP),
            "LEFT_DOWN" => ::std::option::Option::Some(HMFFLJCDJDA::LEFT_DOWN),
            "RIGHT_UP" => ::std::option::Option::Some(HMFFLJCDJDA::RIGHT_UP),
            "RIGHT_DOWN" => ::std::option::Option::Some(HMFFLJCDJDA::RIGHT_DOWN),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [HMFFLJCDJDA] = &[
        HMFFLJCDJDA::LEFT,
        HMFFLJCDJDA::RIGHT,
        HMFFLJCDJDA::UP,
        HMFFLJCDJDA::DOWN,
        HMFFLJCDJDA::LEFT_UP,
        HMFFLJCDJDA::LEFT_DOWN,
        HMFFLJCDJDA::RIGHT_UP,
        HMFFLJCDJDA::RIGHT_DOWN,
    ];
}

impl ::protobuf::EnumFull for HMFFLJCDJDA {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("HMFFLJCDJDA").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for HMFFLJCDJDA {
    fn default() -> Self {
        HMFFLJCDJDA::LEFT
    }
}

impl HMFFLJCDJDA {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<HMFFLJCDJDA>("HMFFLJCDJDA")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HMFFLJCDJDA.proto*n\n\x0bHMFFLJCDJDA\x12\x08\n\x04LEFT\x10\0\x12\t\
    \n\x05RIGHT\x10\x01\x12\x06\n\x02UP\x10\x02\x12\x08\n\x04DOWN\x10\x03\
    \x12\x0b\n\x07LEFT_UP\x10\x04\x12\r\n\tLEFT_DOWN\x10\x05\x12\x0c\n\x08RI\
    GHT_UP\x10\x06\x12\x0e\n\nRIGHT_DOWN\x10\x07b\x06proto3\
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
            enums.push(HMFFLJCDJDA::generated_enum_descriptor_data());
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
