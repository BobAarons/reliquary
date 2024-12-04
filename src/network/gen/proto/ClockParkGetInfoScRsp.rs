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

//! Generated file from `ClockParkGetInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ClockParkGetInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ClockParkGetInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:ClockParkGetInfoScRsp.DMDPHPDNDPO)
    pub DMDPHPDNDPO: ::std::vec::Vec<super::LMHJGGEILGO::LMHJGGEILGO>,
    // @@protoc_insertion_point(field:ClockParkGetInfoScRsp.CMAECALDMAN)
    pub CMAECALDMAN: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ClockParkGetInfoScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:ClockParkGetInfoScRsp.FINLPBFNLHP)
    pub FINLPBFNLHP: u32,
    // @@protoc_insertion_point(field:ClockParkGetInfoScRsp.GJLJIOICNBE)
    pub GJLJIOICNBE: u32,
    // @@protoc_insertion_point(field:ClockParkGetInfoScRsp.KGHFABADCHE)
    pub KGHFABADCHE: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ClockParkGetInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ClockParkGetInfoScRsp {
    fn default() -> &'a ClockParkGetInfoScRsp {
        <ClockParkGetInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ClockParkGetInfoScRsp {
    pub fn new() -> ClockParkGetInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DMDPHPDNDPO",
            |m: &ClockParkGetInfoScRsp| { &m.DMDPHPDNDPO },
            |m: &mut ClockParkGetInfoScRsp| { &mut m.DMDPHPDNDPO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CMAECALDMAN",
            |m: &ClockParkGetInfoScRsp| { &m.CMAECALDMAN },
            |m: &mut ClockParkGetInfoScRsp| { &mut m.CMAECALDMAN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &ClockParkGetInfoScRsp| { &m.ADADHIHDHJC },
            |m: &mut ClockParkGetInfoScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FINLPBFNLHP",
            |m: &ClockParkGetInfoScRsp| { &m.FINLPBFNLHP },
            |m: &mut ClockParkGetInfoScRsp| { &mut m.FINLPBFNLHP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GJLJIOICNBE",
            |m: &ClockParkGetInfoScRsp| { &m.GJLJIOICNBE },
            |m: &mut ClockParkGetInfoScRsp| { &mut m.GJLJIOICNBE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGHFABADCHE",
            |m: &ClockParkGetInfoScRsp| { &m.KGHFABADCHE },
            |m: &mut ClockParkGetInfoScRsp| { &mut m.KGHFABADCHE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ClockParkGetInfoScRsp>(
            "ClockParkGetInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ClockParkGetInfoScRsp {
    const NAME: &'static str = "ClockParkGetInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    self.DMDPHPDNDPO.push(is.read_message()?);
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.CMAECALDMAN)?;
                },
                96 => {
                    self.CMAECALDMAN.push(is.read_uint32()?);
                },
                56 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                8 => {
                    self.FINLPBFNLHP = is.read_uint32()?;
                },
                112 => {
                    self.GJLJIOICNBE = is.read_uint32()?;
                },
                80 => {
                    self.KGHFABADCHE = is.read_uint32()?;
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
        for value in &self.DMDPHPDNDPO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.CMAECALDMAN {
            my_size += ::protobuf::rt::uint32_size(12, *value);
        };
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.ADADHIHDHJC);
        }
        if self.FINLPBFNLHP != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.FINLPBFNLHP);
        }
        if self.GJLJIOICNBE != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.GJLJIOICNBE);
        }
        if self.KGHFABADCHE != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.KGHFABADCHE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.DMDPHPDNDPO {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        for v in &self.CMAECALDMAN {
            os.write_uint32(12, *v)?;
        };
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(7, self.ADADHIHDHJC)?;
        }
        if self.FINLPBFNLHP != 0 {
            os.write_uint32(1, self.FINLPBFNLHP)?;
        }
        if self.GJLJIOICNBE != 0 {
            os.write_uint32(14, self.GJLJIOICNBE)?;
        }
        if self.KGHFABADCHE != 0 {
            os.write_uint32(10, self.KGHFABADCHE)?;
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

    fn new() -> ClockParkGetInfoScRsp {
        ClockParkGetInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.DMDPHPDNDPO.clear();
        self.CMAECALDMAN.clear();
        self.ADADHIHDHJC = 0;
        self.FINLPBFNLHP = 0;
        self.GJLJIOICNBE = 0;
        self.KGHFABADCHE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ClockParkGetInfoScRsp {
        static instance: ClockParkGetInfoScRsp = ClockParkGetInfoScRsp {
            DMDPHPDNDPO: ::std::vec::Vec::new(),
            CMAECALDMAN: ::std::vec::Vec::new(),
            ADADHIHDHJC: 0,
            FINLPBFNLHP: 0,
            GJLJIOICNBE: 0,
            KGHFABADCHE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ClockParkGetInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ClockParkGetInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ClockParkGetInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClockParkGetInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bClockParkGetInfoScRsp.proto\x1a\x11LMHJGGEILGO.proto\"\xf1\x01\n\
    \x15ClockParkGetInfoScRsp\x12.\n\x0bDMDPHPDNDPO\x18\x0b\x20\x03(\x0b2\
    \x0c.LMHJGGEILGOR\x0bDMDPHPDNDPO\x12\x20\n\x0bCMAECALDMAN\x18\x0c\x20\
    \x03(\rR\x0bCMAECALDMAN\x12\x20\n\x0bADADHIHDHJC\x18\x07\x20\x01(\rR\x0b\
    ADADHIHDHJC\x12\x20\n\x0bFINLPBFNLHP\x18\x01\x20\x01(\rR\x0bFINLPBFNLHP\
    \x12\x20\n\x0bGJLJIOICNBE\x18\x0e\x20\x01(\rR\x0bGJLJIOICNBE\x12\x20\n\
    \x0bKGHFABADCHE\x18\n\x20\x01(\rR\x0bKGHFABADCHEb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::LMHJGGEILGO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ClockParkGetInfoScRsp::generated_message_descriptor_data());
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