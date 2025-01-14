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

//! Generated file from `LEFBICDEJDE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:LEFBICDEJDE)
pub enum LEFBICDEJDE {
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.EDITOR)
    EDITOR = 0,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.IOS)
    IOS = 1,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.ANDROID)
    ANDROID = 2,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.PC)
    PC = 3,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.WEB)
    WEB = 4,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.WAP)
    WAP = 5,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.PS4)
    PS4 = 6,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.NINTENDO)
    NINTENDO = 7,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.CLOUD_ANDROID)
    CLOUD_ANDROID = 8,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.CLOUD_PC)
    CLOUD_PC = 9,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.CLOUD_IOS)
    CLOUD_IOS = 10,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.PS5)
    PS5 = 11,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.MAC)
    MAC = 12,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.CLOUD_MAC)
    CLOUD_MAC = 13,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.CLOUD_WEB_ANDROID)
    CLOUD_WEB_ANDROID = 20,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.CLOUD_WEB_IOS)
    CLOUD_WEB_IOS = 21,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.CLOUD_WEB_PC)
    CLOUD_WEB_PC = 22,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.CLOUD_WEB_MAC)
    CLOUD_WEB_MAC = 23,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.CLOUD_WEB_TOUCH)
    CLOUD_WEB_TOUCH = 24,
    // @@protoc_insertion_point(enum_value:LEFBICDEJDE.CLOUD_WEB_KEYBOARD)
    CLOUD_WEB_KEYBOARD = 25,
}

impl ::protobuf::Enum for LEFBICDEJDE {
    const NAME: &'static str = "LEFBICDEJDE";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LEFBICDEJDE> {
        match value {
            0 => ::std::option::Option::Some(LEFBICDEJDE::EDITOR),
            1 => ::std::option::Option::Some(LEFBICDEJDE::IOS),
            2 => ::std::option::Option::Some(LEFBICDEJDE::ANDROID),
            3 => ::std::option::Option::Some(LEFBICDEJDE::PC),
            4 => ::std::option::Option::Some(LEFBICDEJDE::WEB),
            5 => ::std::option::Option::Some(LEFBICDEJDE::WAP),
            6 => ::std::option::Option::Some(LEFBICDEJDE::PS4),
            7 => ::std::option::Option::Some(LEFBICDEJDE::NINTENDO),
            8 => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_ANDROID),
            9 => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_PC),
            10 => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_IOS),
            11 => ::std::option::Option::Some(LEFBICDEJDE::PS5),
            12 => ::std::option::Option::Some(LEFBICDEJDE::MAC),
            13 => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_MAC),
            20 => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_WEB_ANDROID),
            21 => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_WEB_IOS),
            22 => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_WEB_PC),
            23 => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_WEB_MAC),
            24 => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_WEB_TOUCH),
            25 => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_WEB_KEYBOARD),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<LEFBICDEJDE> {
        match str {
            "EDITOR" => ::std::option::Option::Some(LEFBICDEJDE::EDITOR),
            "IOS" => ::std::option::Option::Some(LEFBICDEJDE::IOS),
            "ANDROID" => ::std::option::Option::Some(LEFBICDEJDE::ANDROID),
            "PC" => ::std::option::Option::Some(LEFBICDEJDE::PC),
            "WEB" => ::std::option::Option::Some(LEFBICDEJDE::WEB),
            "WAP" => ::std::option::Option::Some(LEFBICDEJDE::WAP),
            "PS4" => ::std::option::Option::Some(LEFBICDEJDE::PS4),
            "NINTENDO" => ::std::option::Option::Some(LEFBICDEJDE::NINTENDO),
            "CLOUD_ANDROID" => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_ANDROID),
            "CLOUD_PC" => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_PC),
            "CLOUD_IOS" => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_IOS),
            "PS5" => ::std::option::Option::Some(LEFBICDEJDE::PS5),
            "MAC" => ::std::option::Option::Some(LEFBICDEJDE::MAC),
            "CLOUD_MAC" => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_MAC),
            "CLOUD_WEB_ANDROID" => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_WEB_ANDROID),
            "CLOUD_WEB_IOS" => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_WEB_IOS),
            "CLOUD_WEB_PC" => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_WEB_PC),
            "CLOUD_WEB_MAC" => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_WEB_MAC),
            "CLOUD_WEB_TOUCH" => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_WEB_TOUCH),
            "CLOUD_WEB_KEYBOARD" => ::std::option::Option::Some(LEFBICDEJDE::CLOUD_WEB_KEYBOARD),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [LEFBICDEJDE] = &[
        LEFBICDEJDE::EDITOR,
        LEFBICDEJDE::IOS,
        LEFBICDEJDE::ANDROID,
        LEFBICDEJDE::PC,
        LEFBICDEJDE::WEB,
        LEFBICDEJDE::WAP,
        LEFBICDEJDE::PS4,
        LEFBICDEJDE::NINTENDO,
        LEFBICDEJDE::CLOUD_ANDROID,
        LEFBICDEJDE::CLOUD_PC,
        LEFBICDEJDE::CLOUD_IOS,
        LEFBICDEJDE::PS5,
        LEFBICDEJDE::MAC,
        LEFBICDEJDE::CLOUD_MAC,
        LEFBICDEJDE::CLOUD_WEB_ANDROID,
        LEFBICDEJDE::CLOUD_WEB_IOS,
        LEFBICDEJDE::CLOUD_WEB_PC,
        LEFBICDEJDE::CLOUD_WEB_MAC,
        LEFBICDEJDE::CLOUD_WEB_TOUCH,
        LEFBICDEJDE::CLOUD_WEB_KEYBOARD,
    ];
}

impl ::protobuf::EnumFull for LEFBICDEJDE {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("LEFBICDEJDE").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            LEFBICDEJDE::EDITOR => 0,
            LEFBICDEJDE::IOS => 1,
            LEFBICDEJDE::ANDROID => 2,
            LEFBICDEJDE::PC => 3,
            LEFBICDEJDE::WEB => 4,
            LEFBICDEJDE::WAP => 5,
            LEFBICDEJDE::PS4 => 6,
            LEFBICDEJDE::NINTENDO => 7,
            LEFBICDEJDE::CLOUD_ANDROID => 8,
            LEFBICDEJDE::CLOUD_PC => 9,
            LEFBICDEJDE::CLOUD_IOS => 10,
            LEFBICDEJDE::PS5 => 11,
            LEFBICDEJDE::MAC => 12,
            LEFBICDEJDE::CLOUD_MAC => 13,
            LEFBICDEJDE::CLOUD_WEB_ANDROID => 14,
            LEFBICDEJDE::CLOUD_WEB_IOS => 15,
            LEFBICDEJDE::CLOUD_WEB_PC => 16,
            LEFBICDEJDE::CLOUD_WEB_MAC => 17,
            LEFBICDEJDE::CLOUD_WEB_TOUCH => 18,
            LEFBICDEJDE::CLOUD_WEB_KEYBOARD => 19,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for LEFBICDEJDE {
    fn default() -> Self {
        LEFBICDEJDE::EDITOR
    }
}

impl LEFBICDEJDE {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<LEFBICDEJDE>("LEFBICDEJDE")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LEFBICDEJDE.proto*\xad\x02\n\x0bLEFBICDEJDE\x12\n\n\x06EDITOR\x10\
    \0\x12\x07\n\x03IOS\x10\x01\x12\x0b\n\x07ANDROID\x10\x02\x12\x06\n\x02PC\
    \x10\x03\x12\x07\n\x03WEB\x10\x04\x12\x07\n\x03WAP\x10\x05\x12\x07\n\x03\
    PS4\x10\x06\x12\x0c\n\x08NINTENDO\x10\x07\x12\x11\n\rCLOUD_ANDROID\x10\
    \x08\x12\x0c\n\x08CLOUD_PC\x10\t\x12\r\n\tCLOUD_IOS\x10\n\x12\x07\n\x03P\
    S5\x10\x0b\x12\x07\n\x03MAC\x10\x0c\x12\r\n\tCLOUD_MAC\x10\r\x12\x15\n\
    \x11CLOUD_WEB_ANDROID\x10\x14\x12\x11\n\rCLOUD_WEB_IOS\x10\x15\x12\x10\n\
    \x0cCLOUD_WEB_PC\x10\x16\x12\x11\n\rCLOUD_WEB_MAC\x10\x17\x12\x13\n\x0fC\
    LOUD_WEB_TOUCH\x10\x18\x12\x16\n\x12CLOUD_WEB_KEYBOARD\x10\x19b\x06proto\
    3\
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
            enums.push(LEFBICDEJDE::generated_enum_descriptor_data());
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
