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

//! Generated file from `ChessRogueLayerAccountInfoNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChessRogueLayerAccountInfoNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueLayerAccountInfoNotify {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueLayerAccountInfoNotify.IGODAFNLDCK)
    pub IGODAFNLDCK: u32,
    // @@protoc_insertion_point(field:ChessRogueLayerAccountInfoNotify.GEFPDFLDLHD)
    pub GEFPDFLDLHD: u32,
    // @@protoc_insertion_point(field:ChessRogueLayerAccountInfoNotify.LHPDBAMHBKB)
    pub LHPDBAMHBKB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ChessRogueLayerAccountInfoNotify.BCBJPFIKPHG)
    pub BCBJPFIKPHG: ::protobuf::MessageField<super::BNKNFJKMJIE::BNKNFJKMJIE>,
    // @@protoc_insertion_point(field:ChessRogueLayerAccountInfoNotify.APLCCKHHPKC)
    pub APLCCKHHPKC: ::protobuf::MessageField<super::LMPHODOEHCN::LMPHODOEHCN>,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueLayerAccountInfoNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueLayerAccountInfoNotify {
    fn default() -> &'a ChessRogueLayerAccountInfoNotify {
        <ChessRogueLayerAccountInfoNotify as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueLayerAccountInfoNotify {
    pub fn new() -> ChessRogueLayerAccountInfoNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IGODAFNLDCK",
            |m: &ChessRogueLayerAccountInfoNotify| { &m.IGODAFNLDCK },
            |m: &mut ChessRogueLayerAccountInfoNotify| { &mut m.IGODAFNLDCK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GEFPDFLDLHD",
            |m: &ChessRogueLayerAccountInfoNotify| { &m.GEFPDFLDLHD },
            |m: &mut ChessRogueLayerAccountInfoNotify| { &mut m.GEFPDFLDLHD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LHPDBAMHBKB",
            |m: &ChessRogueLayerAccountInfoNotify| { &m.LHPDBAMHBKB },
            |m: &mut ChessRogueLayerAccountInfoNotify| { &mut m.LHPDBAMHBKB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BNKNFJKMJIE::BNKNFJKMJIE>(
            "BCBJPFIKPHG",
            |m: &ChessRogueLayerAccountInfoNotify| { &m.BCBJPFIKPHG },
            |m: &mut ChessRogueLayerAccountInfoNotify| { &mut m.BCBJPFIKPHG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LMPHODOEHCN::LMPHODOEHCN>(
            "APLCCKHHPKC",
            |m: &ChessRogueLayerAccountInfoNotify| { &m.APLCCKHHPKC },
            |m: &mut ChessRogueLayerAccountInfoNotify| { &mut m.APLCCKHHPKC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueLayerAccountInfoNotify>(
            "ChessRogueLayerAccountInfoNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueLayerAccountInfoNotify {
    const NAME: &'static str = "ChessRogueLayerAccountInfoNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.IGODAFNLDCK = is.read_uint32()?;
                },
                64 => {
                    self.GEFPDFLDLHD = is.read_uint32()?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.LHPDBAMHBKB)?;
                },
                104 => {
                    self.LHPDBAMHBKB.push(is.read_uint32()?);
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BCBJPFIKPHG)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.APLCCKHHPKC)?;
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
        if self.IGODAFNLDCK != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.IGODAFNLDCK);
        }
        if self.GEFPDFLDLHD != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.GEFPDFLDLHD);
        }
        for value in &self.LHPDBAMHBKB {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        if let Some(v) = self.BCBJPFIKPHG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.APLCCKHHPKC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IGODAFNLDCK != 0 {
            os.write_uint32(2, self.IGODAFNLDCK)?;
        }
        if self.GEFPDFLDLHD != 0 {
            os.write_uint32(8, self.GEFPDFLDLHD)?;
        }
        for v in &self.LHPDBAMHBKB {
            os.write_uint32(13, *v)?;
        };
        if let Some(v) = self.BCBJPFIKPHG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.APLCCKHHPKC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
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

    fn new() -> ChessRogueLayerAccountInfoNotify {
        ChessRogueLayerAccountInfoNotify::new()
    }

    fn clear(&mut self) {
        self.IGODAFNLDCK = 0;
        self.GEFPDFLDLHD = 0;
        self.LHPDBAMHBKB.clear();
        self.BCBJPFIKPHG.clear();
        self.APLCCKHHPKC.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueLayerAccountInfoNotify {
        static instance: ChessRogueLayerAccountInfoNotify = ChessRogueLayerAccountInfoNotify {
            IGODAFNLDCK: 0,
            GEFPDFLDLHD: 0,
            LHPDBAMHBKB: ::std::vec::Vec::new(),
            BCBJPFIKPHG: ::protobuf::MessageField::none(),
            APLCCKHHPKC: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueLayerAccountInfoNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueLayerAccountInfoNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueLayerAccountInfoNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueLayerAccountInfoNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&ChessRogueLayerAccountInfoNotify.proto\x1a\x11BNKNFJKMJIE.proto\x1a\
    \x11LMPHODOEHCN.proto\"\xe8\x01\n\x20ChessRogueLayerAccountInfoNotify\
    \x12\x20\n\x0bIGODAFNLDCK\x18\x02\x20\x01(\rR\x0bIGODAFNLDCK\x12\x20\n\
    \x0bGEFPDFLDLHD\x18\x08\x20\x01(\rR\x0bGEFPDFLDLHD\x12\x20\n\x0bLHPDBAMH\
    BKB\x18\r\x20\x03(\rR\x0bLHPDBAMHBKB\x12.\n\x0bBCBJPFIKPHG\x18\x04\x20\
    \x01(\x0b2\x0c.BNKNFJKMJIER\x0bBCBJPFIKPHG\x12.\n\x0bAPLCCKHHPKC\x18\x06\
    \x20\x01(\x0b2\x0c.LMPHODOEHCNR\x0bAPLCCKHHPKCb\x06proto3\
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
            deps.push(super::BNKNFJKMJIE::file_descriptor().clone());
            deps.push(super::LMPHODOEHCN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueLayerAccountInfoNotify::generated_message_descriptor_data());
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