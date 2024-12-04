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

//! Generated file from `OPCEMHFFFCI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OPCEMHFFFCI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OPCEMHFFFCI {
    // message fields
    // @@protoc_insertion_point(field:OPCEMHFFFCI.LLBICIFCPPM)
    pub LLBICIFCPPM: u32,
    // @@protoc_insertion_point(field:OPCEMHFFFCI.CBBNGFNOIAP)
    pub CBBNGFNOIAP: u32,
    // @@protoc_insertion_point(field:OPCEMHFFFCI.NKPCHPKFELD)
    pub NKPCHPKFELD: u32,
    // @@protoc_insertion_point(field:OPCEMHFFFCI.FDHPGGFOCBM)
    pub FDHPGGFOCBM: u32,
    // special fields
    // @@protoc_insertion_point(special_field:OPCEMHFFFCI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OPCEMHFFFCI {
    fn default() -> &'a OPCEMHFFFCI {
        <OPCEMHFFFCI as ::protobuf::Message>::default_instance()
    }
}

impl OPCEMHFFFCI {
    pub fn new() -> OPCEMHFFFCI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LLBICIFCPPM",
            |m: &OPCEMHFFFCI| { &m.LLBICIFCPPM },
            |m: &mut OPCEMHFFFCI| { &mut m.LLBICIFCPPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CBBNGFNOIAP",
            |m: &OPCEMHFFFCI| { &m.CBBNGFNOIAP },
            |m: &mut OPCEMHFFFCI| { &mut m.CBBNGFNOIAP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NKPCHPKFELD",
            |m: &OPCEMHFFFCI| { &m.NKPCHPKFELD },
            |m: &mut OPCEMHFFFCI| { &mut m.NKPCHPKFELD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FDHPGGFOCBM",
            |m: &OPCEMHFFFCI| { &m.FDHPGGFOCBM },
            |m: &mut OPCEMHFFFCI| { &mut m.FDHPGGFOCBM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OPCEMHFFFCI>(
            "OPCEMHFFFCI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OPCEMHFFFCI {
    const NAME: &'static str = "OPCEMHFFFCI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.LLBICIFCPPM = is.read_uint32()?;
                },
                80 => {
                    self.CBBNGFNOIAP = is.read_uint32()?;
                },
                112 => {
                    self.NKPCHPKFELD = is.read_uint32()?;
                },
                120 => {
                    self.FDHPGGFOCBM = is.read_uint32()?;
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
        if self.LLBICIFCPPM != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.LLBICIFCPPM);
        }
        if self.CBBNGFNOIAP != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.CBBNGFNOIAP);
        }
        if self.NKPCHPKFELD != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.NKPCHPKFELD);
        }
        if self.FDHPGGFOCBM != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.FDHPGGFOCBM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LLBICIFCPPM != 0 {
            os.write_uint32(13, self.LLBICIFCPPM)?;
        }
        if self.CBBNGFNOIAP != 0 {
            os.write_uint32(10, self.CBBNGFNOIAP)?;
        }
        if self.NKPCHPKFELD != 0 {
            os.write_uint32(14, self.NKPCHPKFELD)?;
        }
        if self.FDHPGGFOCBM != 0 {
            os.write_uint32(15, self.FDHPGGFOCBM)?;
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

    fn new() -> OPCEMHFFFCI {
        OPCEMHFFFCI::new()
    }

    fn clear(&mut self) {
        self.LLBICIFCPPM = 0;
        self.CBBNGFNOIAP = 0;
        self.NKPCHPKFELD = 0;
        self.FDHPGGFOCBM = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OPCEMHFFFCI {
        static instance: OPCEMHFFFCI = OPCEMHFFFCI {
            LLBICIFCPPM: 0,
            CBBNGFNOIAP: 0,
            NKPCHPKFELD: 0,
            FDHPGGFOCBM: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OPCEMHFFFCI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OPCEMHFFFCI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OPCEMHFFFCI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OPCEMHFFFCI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OPCEMHFFFCI.proto\"\x95\x01\n\x0bOPCEMHFFFCI\x12\x20\n\x0bLLBICIFC\
    PPM\x18\r\x20\x01(\rR\x0bLLBICIFCPPM\x12\x20\n\x0bCBBNGFNOIAP\x18\n\x20\
    \x01(\rR\x0bCBBNGFNOIAP\x12\x20\n\x0bNKPCHPKFELD\x18\x0e\x20\x01(\rR\x0b\
    NKPCHPKFELD\x12\x20\n\x0bFDHPGGFOCBM\x18\x0f\x20\x01(\rR\x0bFDHPGGFOCBMb\
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
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OPCEMHFFFCI::generated_message_descriptor_data());
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