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

//! Generated file from `GetMonopolyInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetMonopolyInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetMonopolyInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.MPBGHKODKKO)
    pub MPBGHKODKKO: ::protobuf::MessageField<super::IHDEIEEEIIE::IHDEIEEEIIE>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.KJJEFPLAFOA)
    pub KJJEFPLAFOA: ::protobuf::MessageField<super::MMGHBJCIBMN::MMGHBJCIBMN>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.PCEGPDNBFLN)
    pub PCEGPDNBFLN: ::protobuf::MessageField<super::LNLBMBDCLMC::LNLBMBDCLMC>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.LHMKHBFOLHE)
    pub LHMKHBFOLHE: ::protobuf::MessageField<super::NMOAGFKBEIE::NMOAGFKBEIE>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.ONJPOPFPEFH)
    pub ONJPOPFPEFH: ::protobuf::MessageField<super::KPNOKNOHBKC::KPNOKNOHBKC>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.PGDAMBECANP)
    pub PGDAMBECANP: ::protobuf::MessageField<super::JAFLGAJCHMG::JAFLGAJCHMG>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.LMMLDDKKKAI)
    pub LMMLDDKKKAI: ::protobuf::MessageField<super::KLPNIKBGGHO::KLPNIKBGGHO>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.KONHPGKDHLK)
    pub KONHPGKDHLK: ::protobuf::MessageField<super::NNOGMPFHOJJ::NNOGMPFHOJJ>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.GFOANGJFDBI)
    pub GFOANGJFDBI: ::protobuf::MessageField<super::PKICEHGICIG::PKICEHGICIG>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.NDEAEKHLMON)
    pub NDEAEKHLMON: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.FHEODGILEEP)
    pub FHEODGILEEP: ::protobuf::MessageField<super::BCDICBDJFHH::BCDICBDJFHH>,
    // @@protoc_insertion_point(field:GetMonopolyInfoScRsp.NBOOMOMMFBG)
    pub NBOOMOMMFBG: ::protobuf::MessageField<super::KHMBFLJMFEH::KHMBFLJMFEH>,
    // special fields
    // @@protoc_insertion_point(special_field:GetMonopolyInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetMonopolyInfoScRsp {
    fn default() -> &'a GetMonopolyInfoScRsp {
        <GetMonopolyInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetMonopolyInfoScRsp {
    pub fn new() -> GetMonopolyInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IHDEIEEEIIE::IHDEIEEEIIE>(
            "MPBGHKODKKO",
            |m: &GetMonopolyInfoScRsp| { &m.MPBGHKODKKO },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.MPBGHKODKKO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MMGHBJCIBMN::MMGHBJCIBMN>(
            "KJJEFPLAFOA",
            |m: &GetMonopolyInfoScRsp| { &m.KJJEFPLAFOA },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.KJJEFPLAFOA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LNLBMBDCLMC::LNLBMBDCLMC>(
            "PCEGPDNBFLN",
            |m: &GetMonopolyInfoScRsp| { &m.PCEGPDNBFLN },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.PCEGPDNBFLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NMOAGFKBEIE::NMOAGFKBEIE>(
            "LHMKHBFOLHE",
            |m: &GetMonopolyInfoScRsp| { &m.LHMKHBFOLHE },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.LHMKHBFOLHE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KPNOKNOHBKC::KPNOKNOHBKC>(
            "ONJPOPFPEFH",
            |m: &GetMonopolyInfoScRsp| { &m.ONJPOPFPEFH },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.ONJPOPFPEFH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JAFLGAJCHMG::JAFLGAJCHMG>(
            "PGDAMBECANP",
            |m: &GetMonopolyInfoScRsp| { &m.PGDAMBECANP },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.PGDAMBECANP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &GetMonopolyInfoScRsp| { &m.ADADHIHDHJC },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KLPNIKBGGHO::KLPNIKBGGHO>(
            "LMMLDDKKKAI",
            |m: &GetMonopolyInfoScRsp| { &m.LMMLDDKKKAI },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.LMMLDDKKKAI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NNOGMPFHOJJ::NNOGMPFHOJJ>(
            "KONHPGKDHLK",
            |m: &GetMonopolyInfoScRsp| { &m.KONHPGKDHLK },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.KONHPGKDHLK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PKICEHGICIG::PKICEHGICIG>(
            "GFOANGJFDBI",
            |m: &GetMonopolyInfoScRsp| { &m.GFOANGJFDBI },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.GFOANGJFDBI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NDEAEKHLMON",
            |m: &GetMonopolyInfoScRsp| { &m.NDEAEKHLMON },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.NDEAEKHLMON },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BCDICBDJFHH::BCDICBDJFHH>(
            "FHEODGILEEP",
            |m: &GetMonopolyInfoScRsp| { &m.FHEODGILEEP },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.FHEODGILEEP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KHMBFLJMFEH::KHMBFLJMFEH>(
            "NBOOMOMMFBG",
            |m: &GetMonopolyInfoScRsp| { &m.NBOOMOMMFBG },
            |m: &mut GetMonopolyInfoScRsp| { &mut m.NBOOMOMMFBG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetMonopolyInfoScRsp>(
            "GetMonopolyInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetMonopolyInfoScRsp {
    const NAME: &'static str = "GetMonopolyInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MPBGHKODKKO)?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KJJEFPLAFOA)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PCEGPDNBFLN)?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LHMKHBFOLHE)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ONJPOPFPEFH)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PGDAMBECANP)?;
                },
                16 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LMMLDDKKKAI)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KONHPGKDHLK)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GFOANGJFDBI)?;
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.NDEAEKHLMON)?;
                },
                40 => {
                    self.NDEAEKHLMON.push(is.read_uint32()?);
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FHEODGILEEP)?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NBOOMOMMFBG)?;
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
        if let Some(v) = self.MPBGHKODKKO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.KJJEFPLAFOA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.PCEGPDNBFLN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.LHMKHBFOLHE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.ONJPOPFPEFH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.PGDAMBECANP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.ADADHIHDHJC);
        }
        if let Some(v) = self.LMMLDDKKKAI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.KONHPGKDHLK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.GFOANGJFDBI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.NDEAEKHLMON {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        if let Some(v) = self.FHEODGILEEP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.NBOOMOMMFBG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.MPBGHKODKKO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if let Some(v) = self.KJJEFPLAFOA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.PCEGPDNBFLN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.LHMKHBFOLHE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.ONJPOPFPEFH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.PGDAMBECANP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(2, self.ADADHIHDHJC)?;
        }
        if let Some(v) = self.LMMLDDKKKAI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.KONHPGKDHLK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.GFOANGJFDBI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        for v in &self.NDEAEKHLMON {
            os.write_uint32(5, *v)?;
        };
        if let Some(v) = self.FHEODGILEEP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if let Some(v) = self.NBOOMOMMFBG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> GetMonopolyInfoScRsp {
        GetMonopolyInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.MPBGHKODKKO.clear();
        self.KJJEFPLAFOA.clear();
        self.PCEGPDNBFLN.clear();
        self.LHMKHBFOLHE.clear();
        self.ONJPOPFPEFH.clear();
        self.PGDAMBECANP.clear();
        self.ADADHIHDHJC = 0;
        self.LMMLDDKKKAI.clear();
        self.KONHPGKDHLK.clear();
        self.GFOANGJFDBI.clear();
        self.NDEAEKHLMON.clear();
        self.FHEODGILEEP.clear();
        self.NBOOMOMMFBG.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetMonopolyInfoScRsp {
        static instance: GetMonopolyInfoScRsp = GetMonopolyInfoScRsp {
            MPBGHKODKKO: ::protobuf::MessageField::none(),
            KJJEFPLAFOA: ::protobuf::MessageField::none(),
            PCEGPDNBFLN: ::protobuf::MessageField::none(),
            LHMKHBFOLHE: ::protobuf::MessageField::none(),
            ONJPOPFPEFH: ::protobuf::MessageField::none(),
            PGDAMBECANP: ::protobuf::MessageField::none(),
            ADADHIHDHJC: 0,
            LMMLDDKKKAI: ::protobuf::MessageField::none(),
            KONHPGKDHLK: ::protobuf::MessageField::none(),
            GFOANGJFDBI: ::protobuf::MessageField::none(),
            NDEAEKHLMON: ::std::vec::Vec::new(),
            FHEODGILEEP: ::protobuf::MessageField::none(),
            NBOOMOMMFBG: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetMonopolyInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetMonopolyInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetMonopolyInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetMonopolyInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aGetMonopolyInfoScRsp.proto\x1a\x11BCDICBDJFHH.proto\x1a\x11IHDEIEE\
    EIIE.proto\x1a\x11JAFLGAJCHMG.proto\x1a\x11KHMBFLJMFEH.proto\x1a\x11KLPN\
    IKBGGHO.proto\x1a\x11KPNOKNOHBKC.proto\x1a\x11LNLBMBDCLMC.proto\x1a\x11M\
    MGHBJCIBMN.proto\x1a\x11NMOAGFKBEIE.proto\x1a\x11NNOGMPFHOJJ.proto\x1a\
    \x11PKICEHGICIG.proto\"\xea\x04\n\x14GetMonopolyInfoScRsp\x12.\n\x0bMPBG\
    HKODKKO\x18\x0c\x20\x01(\x0b2\x0c.IHDEIEEEIIER\x0bMPBGHKODKKO\x12.\n\x0b\
    KJJEFPLAFOA\x18\r\x20\x01(\x0b2\x0c.MMGHBJCIBMNR\x0bKJJEFPLAFOA\x12.\n\
    \x0bPCEGPDNBFLN\x18\x0e\x20\x01(\x0b2\x0c.LNLBMBDCLMCR\x0bPCEGPDNBFLN\
    \x12.\n\x0bLHMKHBFOLHE\x18\t\x20\x01(\x0b2\x0c.NMOAGFKBEIER\x0bLHMKHBFOL\
    HE\x12.\n\x0bONJPOPFPEFH\x18\n\x20\x01(\x0b2\x0c.KPNOKNOHBKCR\x0bONJPOPF\
    PEFH\x12.\n\x0bPGDAMBECANP\x18\x03\x20\x01(\x0b2\x0c.JAFLGAJCHMGR\x0bPGD\
    AMBECANP\x12\x20\n\x0bADADHIHDHJC\x18\x02\x20\x01(\rR\x0bADADHIHDHJC\x12\
    .\n\x0bLMMLDDKKKAI\x18\x04\x20\x01(\x0b2\x0c.KLPNIKBGGHOR\x0bLMMLDDKKKAI\
    \x12.\n\x0bKONHPGKDHLK\x18\x01\x20\x01(\x0b2\x0c.NNOGMPFHOJJR\x0bKONHPGK\
    DHLK\x12.\n\x0bGFOANGJFDBI\x18\x0b\x20\x01(\x0b2\x0c.PKICEHGICIGR\x0bGFO\
    ANGJFDBI\x12\x20\n\x0bNDEAEKHLMON\x18\x05\x20\x03(\rR\x0bNDEAEKHLMON\x12\
    .\n\x0bFHEODGILEEP\x18\x07\x20\x01(\x0b2\x0c.BCDICBDJFHHR\x0bFHEODGILEEP\
    \x12.\n\x0bNBOOMOMMFBG\x18\x0f\x20\x01(\x0b2\x0c.KHMBFLJMFEHR\x0bNBOOMOM\
    MFBGb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(11);
            deps.push(super::BCDICBDJFHH::file_descriptor().clone());
            deps.push(super::IHDEIEEEIIE::file_descriptor().clone());
            deps.push(super::JAFLGAJCHMG::file_descriptor().clone());
            deps.push(super::KHMBFLJMFEH::file_descriptor().clone());
            deps.push(super::KLPNIKBGGHO::file_descriptor().clone());
            deps.push(super::KPNOKNOHBKC::file_descriptor().clone());
            deps.push(super::LNLBMBDCLMC::file_descriptor().clone());
            deps.push(super::MMGHBJCIBMN::file_descriptor().clone());
            deps.push(super::NMOAGFKBEIE::file_descriptor().clone());
            deps.push(super::NNOGMPFHOJJ::file_descriptor().clone());
            deps.push(super::PKICEHGICIG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetMonopolyInfoScRsp::generated_message_descriptor_data());
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