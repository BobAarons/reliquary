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

//! Generated file from `POGLCMCGMIK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:POGLCMCGMIK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct POGLCMCGMIK {
    // message fields
    // @@protoc_insertion_point(field:POGLCMCGMIK.OMBLHJDKEJA)
    pub OMBLHJDKEJA: u32,
    // @@protoc_insertion_point(field:POGLCMCGMIK.KFIMLHHPMLI)
    pub KFIMLHHPMLI: ::protobuf::MessageField<super::DEGGBBPMLMI::DEGGBBPMLMI>,
    // @@protoc_insertion_point(field:POGLCMCGMIK.HAGGJLJEFKA)
    pub HAGGJLJEFKA: ::protobuf::MessageField<super::LNFPOIOEBKD::LNFPOIOEBKD>,
    // @@protoc_insertion_point(field:POGLCMCGMIK.DDMDPBEGKHC)
    pub DDMDPBEGKHC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:POGLCMCGMIK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a POGLCMCGMIK {
    fn default() -> &'a POGLCMCGMIK {
        <POGLCMCGMIK as ::protobuf::Message>::default_instance()
    }
}

impl POGLCMCGMIK {
    pub fn new() -> POGLCMCGMIK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OMBLHJDKEJA",
            |m: &POGLCMCGMIK| { &m.OMBLHJDKEJA },
            |m: &mut POGLCMCGMIK| { &mut m.OMBLHJDKEJA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DEGGBBPMLMI::DEGGBBPMLMI>(
            "KFIMLHHPMLI",
            |m: &POGLCMCGMIK| { &m.KFIMLHHPMLI },
            |m: &mut POGLCMCGMIK| { &mut m.KFIMLHHPMLI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LNFPOIOEBKD::LNFPOIOEBKD>(
            "HAGGJLJEFKA",
            |m: &POGLCMCGMIK| { &m.HAGGJLJEFKA },
            |m: &mut POGLCMCGMIK| { &mut m.HAGGJLJEFKA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DDMDPBEGKHC",
            |m: &POGLCMCGMIK| { &m.DDMDPBEGKHC },
            |m: &mut POGLCMCGMIK| { &mut m.DDMDPBEGKHC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<POGLCMCGMIK>(
            "POGLCMCGMIK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for POGLCMCGMIK {
    const NAME: &'static str = "POGLCMCGMIK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.OMBLHJDKEJA = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KFIMLHHPMLI)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HAGGJLJEFKA)?;
                },
                40 => {
                    self.DDMDPBEGKHC = is.read_uint32()?;
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
        if self.OMBLHJDKEJA != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.OMBLHJDKEJA);
        }
        if let Some(v) = self.KFIMLHHPMLI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.HAGGJLJEFKA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.DDMDPBEGKHC != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.DDMDPBEGKHC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OMBLHJDKEJA != 0 {
            os.write_uint32(15, self.OMBLHJDKEJA)?;
        }
        if let Some(v) = self.KFIMLHHPMLI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if let Some(v) = self.HAGGJLJEFKA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.DDMDPBEGKHC != 0 {
            os.write_uint32(5, self.DDMDPBEGKHC)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> POGLCMCGMIK {
        POGLCMCGMIK::new()
    }

    fn clear(&mut self) {
        self.OMBLHJDKEJA = 0;
        self.KFIMLHHPMLI.clear();
        self.HAGGJLJEFKA.clear();
        self.DDMDPBEGKHC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static POGLCMCGMIK {
        static instance: POGLCMCGMIK = POGLCMCGMIK {
            OMBLHJDKEJA: 0,
            KFIMLHHPMLI: ::protobuf::MessageField::none(),
            HAGGJLJEFKA: ::protobuf::MessageField::none(),
            DDMDPBEGKHC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for POGLCMCGMIK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("POGLCMCGMIK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for POGLCMCGMIK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for POGLCMCGMIK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11POGLCMCGMIK.proto\x1a\x11DEGGBBPMLMI.proto\x1a\x11LNFPOIOEBKD.prot\
    o\"\xb1\x01\n\x0bPOGLCMCGMIK\x12\x20\n\x0bOMBLHJDKEJA\x18\x0f\x20\x01(\r\
    R\x0bOMBLHJDKEJA\x12.\n\x0bKFIMLHHPMLI\x18\x07\x20\x01(\x0b2\x0c.DEGGBBP\
    MLMIR\x0bKFIMLHHPMLI\x12.\n\x0bHAGGJLJEFKA\x18\x04\x20\x01(\x0b2\x0c.LNF\
    POIOEBKDR\x0bHAGGJLJEFKA\x12\x20\n\x0bDDMDPBEGKHC\x18\x05\x20\x01(\rR\
    \x0bDDMDPBEGKHCb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::DEGGBBPMLMI::file_descriptor().clone());
            deps.push(super::LNFPOIOEBKD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(POGLCMCGMIK::generated_message_descriptor_data());
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