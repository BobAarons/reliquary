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

//! Generated file from `DDNHBFCDPJN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DDNHBFCDPJN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DDNHBFCDPJN {
    // message fields
    // @@protoc_insertion_point(field:DDNHBFCDPJN.CEAAONOMINB)
    pub CEAAONOMINB: ::protobuf::MessageField<super::GCPAMIEDFIP::GCPAMIEDFIP>,
    // @@protoc_insertion_point(field:DDNHBFCDPJN.BKAEEIGENOA)
    pub BKAEEIGENOA: bool,
    // @@protoc_insertion_point(field:DDNHBFCDPJN.HCFGMKCCAGA)
    pub HCFGMKCCAGA: u32,
    // @@protoc_insertion_point(field:DDNHBFCDPJN.IDIMBKMJOMA)
    pub IDIMBKMJOMA: ::protobuf::MessageField<super::GCPAMIEDFIP::GCPAMIEDFIP>,
    // @@protoc_insertion_point(field:DDNHBFCDPJN.HNNCGIBOHCO)
    pub HNNCGIBOHCO: bool,
    // @@protoc_insertion_point(field:DDNHBFCDPJN.PCJNNCJLPDA)
    pub PCJNNCJLPDA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DDNHBFCDPJN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DDNHBFCDPJN {
    fn default() -> &'a DDNHBFCDPJN {
        <DDNHBFCDPJN as ::protobuf::Message>::default_instance()
    }
}

impl DDNHBFCDPJN {
    pub fn new() -> DDNHBFCDPJN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GCPAMIEDFIP::GCPAMIEDFIP>(
            "CEAAONOMINB",
            |m: &DDNHBFCDPJN| { &m.CEAAONOMINB },
            |m: &mut DDNHBFCDPJN| { &mut m.CEAAONOMINB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BKAEEIGENOA",
            |m: &DDNHBFCDPJN| { &m.BKAEEIGENOA },
            |m: &mut DDNHBFCDPJN| { &mut m.BKAEEIGENOA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HCFGMKCCAGA",
            |m: &DDNHBFCDPJN| { &m.HCFGMKCCAGA },
            |m: &mut DDNHBFCDPJN| { &mut m.HCFGMKCCAGA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GCPAMIEDFIP::GCPAMIEDFIP>(
            "IDIMBKMJOMA",
            |m: &DDNHBFCDPJN| { &m.IDIMBKMJOMA },
            |m: &mut DDNHBFCDPJN| { &mut m.IDIMBKMJOMA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HNNCGIBOHCO",
            |m: &DDNHBFCDPJN| { &m.HNNCGIBOHCO },
            |m: &mut DDNHBFCDPJN| { &mut m.HNNCGIBOHCO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PCJNNCJLPDA",
            |m: &DDNHBFCDPJN| { &m.PCJNNCJLPDA },
            |m: &mut DDNHBFCDPJN| { &mut m.PCJNNCJLPDA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DDNHBFCDPJN>(
            "DDNHBFCDPJN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DDNHBFCDPJN {
    const NAME: &'static str = "DDNHBFCDPJN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CEAAONOMINB)?;
                },
                48 => {
                    self.BKAEEIGENOA = is.read_bool()?;
                },
                24 => {
                    self.HCFGMKCCAGA = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IDIMBKMJOMA)?;
                },
                16 => {
                    self.HNNCGIBOHCO = is.read_bool()?;
                },
                112 => {
                    self.PCJNNCJLPDA = is.read_uint32()?;
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
        if let Some(v) = self.CEAAONOMINB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.BKAEEIGENOA != false {
            my_size += 1 + 1;
        }
        if self.HCFGMKCCAGA != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.HCFGMKCCAGA);
        }
        if let Some(v) = self.IDIMBKMJOMA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.HNNCGIBOHCO != false {
            my_size += 1 + 1;
        }
        if self.PCJNNCJLPDA != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.PCJNNCJLPDA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.CEAAONOMINB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.BKAEEIGENOA != false {
            os.write_bool(6, self.BKAEEIGENOA)?;
        }
        if self.HCFGMKCCAGA != 0 {
            os.write_uint32(3, self.HCFGMKCCAGA)?;
        }
        if let Some(v) = self.IDIMBKMJOMA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.HNNCGIBOHCO != false {
            os.write_bool(2, self.HNNCGIBOHCO)?;
        }
        if self.PCJNNCJLPDA != 0 {
            os.write_uint32(14, self.PCJNNCJLPDA)?;
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

    fn new() -> DDNHBFCDPJN {
        DDNHBFCDPJN::new()
    }

    fn clear(&mut self) {
        self.CEAAONOMINB.clear();
        self.BKAEEIGENOA = false;
        self.HCFGMKCCAGA = 0;
        self.IDIMBKMJOMA.clear();
        self.HNNCGIBOHCO = false;
        self.PCJNNCJLPDA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DDNHBFCDPJN {
        static instance: DDNHBFCDPJN = DDNHBFCDPJN {
            CEAAONOMINB: ::protobuf::MessageField::none(),
            BKAEEIGENOA: false,
            HCFGMKCCAGA: 0,
            IDIMBKMJOMA: ::protobuf::MessageField::none(),
            HNNCGIBOHCO: false,
            PCJNNCJLPDA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DDNHBFCDPJN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DDNHBFCDPJN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DDNHBFCDPJN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DDNHBFCDPJN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DDNHBFCDPJN.proto\x1a\x11GCPAMIEDFIP.proto\"\xf5\x01\n\x0bDDNHBFCD\
    PJN\x12.\n\x0bCEAAONOMINB\x18\x08\x20\x01(\x0b2\x0c.GCPAMIEDFIPR\x0bCEAA\
    ONOMINB\x12\x20\n\x0bBKAEEIGENOA\x18\x06\x20\x01(\x08R\x0bBKAEEIGENOA\
    \x12\x20\n\x0bHCFGMKCCAGA\x18\x03\x20\x01(\rR\x0bHCFGMKCCAGA\x12.\n\x0bI\
    DIMBKMJOMA\x18\x07\x20\x01(\x0b2\x0c.GCPAMIEDFIPR\x0bIDIMBKMJOMA\x12\x20\
    \n\x0bHNNCGIBOHCO\x18\x02\x20\x01(\x08R\x0bHNNCGIBOHCO\x12\x20\n\x0bPCJN\
    NCJLPDA\x18\x0e\x20\x01(\rR\x0bPCJNNCJLPDAb\x06proto3\
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
            deps.push(super::GCPAMIEDFIP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DDNHBFCDPJN::generated_message_descriptor_data());
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