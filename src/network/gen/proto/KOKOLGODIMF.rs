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

//! Generated file from `KOKOLGODIMF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KOKOLGODIMF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KOKOLGODIMF {
    // message fields
    // @@protoc_insertion_point(field:KOKOLGODIMF.ICGOAMADMPH)
    pub ICGOAMADMPH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:KOKOLGODIMF.GPKMFEMIDEM)
    pub GPKMFEMIDEM: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:KOKOLGODIMF.NAJPHNPMAIN)
    pub NAJPHNPMAIN: ::std::vec::Vec<super::PBPDBCCFBGH::PBPDBCCFBGH>,
    // @@protoc_insertion_point(field:KOKOLGODIMF.HJLFMIGNEMA)
    pub HJLFMIGNEMA: ::std::vec::Vec<super::FDMIHCJCAEM::FDMIHCJCAEM>,
    // special fields
    // @@protoc_insertion_point(special_field:KOKOLGODIMF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KOKOLGODIMF {
    fn default() -> &'a KOKOLGODIMF {
        <KOKOLGODIMF as ::protobuf::Message>::default_instance()
    }
}

impl KOKOLGODIMF {
    pub fn new() -> KOKOLGODIMF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ICGOAMADMPH",
            |m: &KOKOLGODIMF| { &m.ICGOAMADMPH },
            |m: &mut KOKOLGODIMF| { &mut m.ICGOAMADMPH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GPKMFEMIDEM",
            |m: &KOKOLGODIMF| { &m.GPKMFEMIDEM },
            |m: &mut KOKOLGODIMF| { &mut m.GPKMFEMIDEM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NAJPHNPMAIN",
            |m: &KOKOLGODIMF| { &m.NAJPHNPMAIN },
            |m: &mut KOKOLGODIMF| { &mut m.NAJPHNPMAIN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HJLFMIGNEMA",
            |m: &KOKOLGODIMF| { &m.HJLFMIGNEMA },
            |m: &mut KOKOLGODIMF| { &mut m.HJLFMIGNEMA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KOKOLGODIMF>(
            "KOKOLGODIMF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KOKOLGODIMF {
    const NAME: &'static str = "KOKOLGODIMF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    is.read_repeated_packed_uint32_into(&mut self.ICGOAMADMPH)?;
                },
                48 => {
                    self.ICGOAMADMPH.push(is.read_uint32()?);
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.GPKMFEMIDEM)?;
                },
                56 => {
                    self.GPKMFEMIDEM.push(is.read_uint32()?);
                },
                106 => {
                    self.NAJPHNPMAIN.push(is.read_message()?);
                },
                122 => {
                    self.HJLFMIGNEMA.push(is.read_message()?);
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
        for value in &self.ICGOAMADMPH {
            my_size += ::protobuf::rt::uint32_size(6, *value);
        };
        for value in &self.GPKMFEMIDEM {
            my_size += ::protobuf::rt::uint32_size(7, *value);
        };
        for value in &self.NAJPHNPMAIN {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.HJLFMIGNEMA {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.ICGOAMADMPH {
            os.write_uint32(6, *v)?;
        };
        for v in &self.GPKMFEMIDEM {
            os.write_uint32(7, *v)?;
        };
        for v in &self.NAJPHNPMAIN {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        for v in &self.HJLFMIGNEMA {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> KOKOLGODIMF {
        KOKOLGODIMF::new()
    }

    fn clear(&mut self) {
        self.ICGOAMADMPH.clear();
        self.GPKMFEMIDEM.clear();
        self.NAJPHNPMAIN.clear();
        self.HJLFMIGNEMA.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KOKOLGODIMF {
        static instance: KOKOLGODIMF = KOKOLGODIMF {
            ICGOAMADMPH: ::std::vec::Vec::new(),
            GPKMFEMIDEM: ::std::vec::Vec::new(),
            NAJPHNPMAIN: ::std::vec::Vec::new(),
            HJLFMIGNEMA: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KOKOLGODIMF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KOKOLGODIMF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KOKOLGODIMF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KOKOLGODIMF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KOKOLGODIMF.proto\x1a\x11FDMIHCJCAEM.proto\x1a\x11PBPDBCCFBGH.prot\
    o\"\xb1\x01\n\x0bKOKOLGODIMF\x12\x20\n\x0bICGOAMADMPH\x18\x06\x20\x03(\r\
    R\x0bICGOAMADMPH\x12\x20\n\x0bGPKMFEMIDEM\x18\x07\x20\x03(\rR\x0bGPKMFEM\
    IDEM\x12.\n\x0bNAJPHNPMAIN\x18\r\x20\x03(\x0b2\x0c.PBPDBCCFBGHR\x0bNAJPH\
    NPMAIN\x12.\n\x0bHJLFMIGNEMA\x18\x0f\x20\x03(\x0b2\x0c.FDMIHCJCAEMR\x0bH\
    JLFMIGNEMAb\x06proto3\
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
            deps.push(super::FDMIHCJCAEM::file_descriptor().clone());
            deps.push(super::PBPDBCCFBGH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(KOKOLGODIMF::generated_message_descriptor_data());
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
