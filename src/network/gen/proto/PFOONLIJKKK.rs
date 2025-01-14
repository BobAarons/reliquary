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

//! Generated file from `PFOONLIJKKK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PFOONLIJKKK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PFOONLIJKKK {
    // message fields
    // @@protoc_insertion_point(field:PFOONLIJKKK.DFCCGODMLLD)
    pub DFCCGODMLLD: bool,
    // @@protoc_insertion_point(field:PFOONLIJKKK.PAGMFIDOLPD)
    pub PAGMFIDOLPD: bool,
    // @@protoc_insertion_point(field:PFOONLIJKKK.GCIMMFIDPJP)
    pub GCIMMFIDPJP: u32,
    // @@protoc_insertion_point(field:PFOONLIJKKK.AKLHHELBPPI)
    pub AKLHHELBPPI: u32,
    // @@protoc_insertion_point(field:PFOONLIJKKK.MNCENLIMJEB)
    pub MNCENLIMJEB: bool,
    // @@protoc_insertion_point(field:PFOONLIJKKK.FDBGIABHNGC)
    pub FDBGIABHNGC: bool,
    // @@protoc_insertion_point(field:PFOONLIJKKK.EBDDNGHLIGH)
    pub EBDDNGHLIGH: ::std::vec::Vec<super::ANOEBHANHDM::ANOEBHANHDM>,
    // @@protoc_insertion_point(field:PFOONLIJKKK.ONNOFIINCAH)
    pub ONNOFIINCAH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PFOONLIJKKK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PFOONLIJKKK {
    fn default() -> &'a PFOONLIJKKK {
        <PFOONLIJKKK as ::protobuf::Message>::default_instance()
    }
}

impl PFOONLIJKKK {
    pub fn new() -> PFOONLIJKKK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DFCCGODMLLD",
            |m: &PFOONLIJKKK| { &m.DFCCGODMLLD },
            |m: &mut PFOONLIJKKK| { &mut m.DFCCGODMLLD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PAGMFIDOLPD",
            |m: &PFOONLIJKKK| { &m.PAGMFIDOLPD },
            |m: &mut PFOONLIJKKK| { &mut m.PAGMFIDOLPD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GCIMMFIDPJP",
            |m: &PFOONLIJKKK| { &m.GCIMMFIDPJP },
            |m: &mut PFOONLIJKKK| { &mut m.GCIMMFIDPJP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AKLHHELBPPI",
            |m: &PFOONLIJKKK| { &m.AKLHHELBPPI },
            |m: &mut PFOONLIJKKK| { &mut m.AKLHHELBPPI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MNCENLIMJEB",
            |m: &PFOONLIJKKK| { &m.MNCENLIMJEB },
            |m: &mut PFOONLIJKKK| { &mut m.MNCENLIMJEB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FDBGIABHNGC",
            |m: &PFOONLIJKKK| { &m.FDBGIABHNGC },
            |m: &mut PFOONLIJKKK| { &mut m.FDBGIABHNGC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EBDDNGHLIGH",
            |m: &PFOONLIJKKK| { &m.EBDDNGHLIGH },
            |m: &mut PFOONLIJKKK| { &mut m.EBDDNGHLIGH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ONNOFIINCAH",
            |m: &PFOONLIJKKK| { &m.ONNOFIINCAH },
            |m: &mut PFOONLIJKKK| { &mut m.ONNOFIINCAH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PFOONLIJKKK>(
            "PFOONLIJKKK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PFOONLIJKKK {
    const NAME: &'static str = "PFOONLIJKKK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.DFCCGODMLLD = is.read_bool()?;
                },
                48 => {
                    self.PAGMFIDOLPD = is.read_bool()?;
                },
                88 => {
                    self.GCIMMFIDPJP = is.read_uint32()?;
                },
                56 => {
                    self.AKLHHELBPPI = is.read_uint32()?;
                },
                104 => {
                    self.MNCENLIMJEB = is.read_bool()?;
                },
                64 => {
                    self.FDBGIABHNGC = is.read_bool()?;
                },
                13290 => {
                    self.EBDDNGHLIGH.push(is.read_message()?);
                },
                16 => {
                    self.ONNOFIINCAH = is.read_uint32()?;
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
        if self.DFCCGODMLLD != false {
            my_size += 1 + 1;
        }
        if self.PAGMFIDOLPD != false {
            my_size += 1 + 1;
        }
        if self.GCIMMFIDPJP != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.GCIMMFIDPJP);
        }
        if self.AKLHHELBPPI != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.AKLHHELBPPI);
        }
        if self.MNCENLIMJEB != false {
            my_size += 1 + 1;
        }
        if self.FDBGIABHNGC != false {
            my_size += 1 + 1;
        }
        for value in &self.EBDDNGHLIGH {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.ONNOFIINCAH != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.ONNOFIINCAH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DFCCGODMLLD != false {
            os.write_bool(3, self.DFCCGODMLLD)?;
        }
        if self.PAGMFIDOLPD != false {
            os.write_bool(6, self.PAGMFIDOLPD)?;
        }
        if self.GCIMMFIDPJP != 0 {
            os.write_uint32(11, self.GCIMMFIDPJP)?;
        }
        if self.AKLHHELBPPI != 0 {
            os.write_uint32(7, self.AKLHHELBPPI)?;
        }
        if self.MNCENLIMJEB != false {
            os.write_bool(13, self.MNCENLIMJEB)?;
        }
        if self.FDBGIABHNGC != false {
            os.write_bool(8, self.FDBGIABHNGC)?;
        }
        for v in &self.EBDDNGHLIGH {
            ::protobuf::rt::write_message_field_with_cached_size(1661, v, os)?;
        };
        if self.ONNOFIINCAH != 0 {
            os.write_uint32(2, self.ONNOFIINCAH)?;
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

    fn new() -> PFOONLIJKKK {
        PFOONLIJKKK::new()
    }

    fn clear(&mut self) {
        self.DFCCGODMLLD = false;
        self.PAGMFIDOLPD = false;
        self.GCIMMFIDPJP = 0;
        self.AKLHHELBPPI = 0;
        self.MNCENLIMJEB = false;
        self.FDBGIABHNGC = false;
        self.EBDDNGHLIGH.clear();
        self.ONNOFIINCAH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PFOONLIJKKK {
        static instance: PFOONLIJKKK = PFOONLIJKKK {
            DFCCGODMLLD: false,
            PAGMFIDOLPD: false,
            GCIMMFIDPJP: 0,
            AKLHHELBPPI: 0,
            MNCENLIMJEB: false,
            FDBGIABHNGC: false,
            EBDDNGHLIGH: ::std::vec::Vec::new(),
            ONNOFIINCAH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PFOONLIJKKK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PFOONLIJKKK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PFOONLIJKKK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PFOONLIJKKK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PFOONLIJKKK.proto\x1a\x11ANOEBHANHDM.proto\"\xac\x02\n\x0bPFOONLIJ\
    KKK\x12\x20\n\x0bDFCCGODMLLD\x18\x03\x20\x01(\x08R\x0bDFCCGODMLLD\x12\
    \x20\n\x0bPAGMFIDOLPD\x18\x06\x20\x01(\x08R\x0bPAGMFIDOLPD\x12\x20\n\x0b\
    GCIMMFIDPJP\x18\x0b\x20\x01(\rR\x0bGCIMMFIDPJP\x12\x20\n\x0bAKLHHELBPPI\
    \x18\x07\x20\x01(\rR\x0bAKLHHELBPPI\x12\x20\n\x0bMNCENLIMJEB\x18\r\x20\
    \x01(\x08R\x0bMNCENLIMJEB\x12\x20\n\x0bFDBGIABHNGC\x18\x08\x20\x01(\x08R\
    \x0bFDBGIABHNGC\x12/\n\x0bEBDDNGHLIGH\x18\xfd\x0c\x20\x03(\x0b2\x0c.ANOE\
    BHANHDMR\x0bEBDDNGHLIGH\x12\x20\n\x0bONNOFIINCAH\x18\x02\x20\x01(\rR\x0b\
    ONNOFIINCAHb\x06proto3\
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
            deps.push(super::ANOEBHANHDM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PFOONLIJKKK::generated_message_descriptor_data());
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
