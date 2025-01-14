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

//! Generated file from `APKPMOHIBKJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:APKPMOHIBKJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct APKPMOHIBKJ {
    // message fields
    // @@protoc_insertion_point(field:APKPMOHIBKJ.GIHPHOHBGFG)
    pub GIHPHOHBGFG: ::protobuf::MessageField<super::EMEGHPNHPFG::EMEGHPNHPFG>,
    // @@protoc_insertion_point(field:APKPMOHIBKJ.PJGBJGHPDAC)
    pub PJGBJGHPDAC: ::protobuf::MessageField<super::GIIPEENECMB::GIIPEENECMB>,
    // @@protoc_insertion_point(field:APKPMOHIBKJ.NNDPGGJCPOG)
    pub NNDPGGJCPOG: ::protobuf::MessageField<super::CFHNFGHEJOB::CFHNFGHEJOB>,
    // @@protoc_insertion_point(field:APKPMOHIBKJ.DAELBDIDMMH)
    pub DAELBDIDMMH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:APKPMOHIBKJ.NEDIBLKMFAO)
    pub NEDIBLKMFAO: ::protobuf::MessageField<super::FKGKOEMFGEO::FKGKOEMFGEO>,
    // @@protoc_insertion_point(field:APKPMOHIBKJ.EAMJPKMKKHK)
    pub EAMJPKMKKHK: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:APKPMOHIBKJ.OFHKCHGPKHE)
    pub OFHKCHGPKHE: ::protobuf::MessageField<super::ACDOIEAHOKK::ACDOIEAHOKK>,
    // special fields
    // @@protoc_insertion_point(special_field:APKPMOHIBKJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a APKPMOHIBKJ {
    fn default() -> &'a APKPMOHIBKJ {
        <APKPMOHIBKJ as ::protobuf::Message>::default_instance()
    }
}

impl APKPMOHIBKJ {
    pub fn new() -> APKPMOHIBKJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EMEGHPNHPFG::EMEGHPNHPFG>(
            "GIHPHOHBGFG",
            |m: &APKPMOHIBKJ| { &m.GIHPHOHBGFG },
            |m: &mut APKPMOHIBKJ| { &mut m.GIHPHOHBGFG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GIIPEENECMB::GIIPEENECMB>(
            "PJGBJGHPDAC",
            |m: &APKPMOHIBKJ| { &m.PJGBJGHPDAC },
            |m: &mut APKPMOHIBKJ| { &mut m.PJGBJGHPDAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CFHNFGHEJOB::CFHNFGHEJOB>(
            "NNDPGGJCPOG",
            |m: &APKPMOHIBKJ| { &m.NNDPGGJCPOG },
            |m: &mut APKPMOHIBKJ| { &mut m.NNDPGGJCPOG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DAELBDIDMMH",
            |m: &APKPMOHIBKJ| { &m.DAELBDIDMMH },
            |m: &mut APKPMOHIBKJ| { &mut m.DAELBDIDMMH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FKGKOEMFGEO::FKGKOEMFGEO>(
            "NEDIBLKMFAO",
            |m: &APKPMOHIBKJ| { &m.NEDIBLKMFAO },
            |m: &mut APKPMOHIBKJ| { &mut m.NEDIBLKMFAO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EAMJPKMKKHK",
            |m: &APKPMOHIBKJ| { &m.EAMJPKMKKHK },
            |m: &mut APKPMOHIBKJ| { &mut m.EAMJPKMKKHK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ACDOIEAHOKK::ACDOIEAHOKK>(
            "OFHKCHGPKHE",
            |m: &APKPMOHIBKJ| { &m.OFHKCHGPKHE },
            |m: &mut APKPMOHIBKJ| { &mut m.OFHKCHGPKHE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<APKPMOHIBKJ>(
            "APKPMOHIBKJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for APKPMOHIBKJ {
    const NAME: &'static str = "APKPMOHIBKJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GIHPHOHBGFG)?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PJGBJGHPDAC)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NNDPGGJCPOG)?;
                },
                50 => {
                    is.read_repeated_packed_uint32_into(&mut self.DAELBDIDMMH)?;
                },
                48 => {
                    self.DAELBDIDMMH.push(is.read_uint32()?);
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NEDIBLKMFAO)?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.EAMJPKMKKHK)?;
                },
                16 => {
                    self.EAMJPKMKKHK.push(is.read_uint32()?);
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OFHKCHGPKHE)?;
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
        if let Some(v) = self.GIHPHOHBGFG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.PJGBJGHPDAC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.NNDPGGJCPOG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.DAELBDIDMMH {
            my_size += ::protobuf::rt::uint32_size(6, *value);
        };
        if let Some(v) = self.NEDIBLKMFAO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.EAMJPKMKKHK {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        if let Some(v) = self.OFHKCHGPKHE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.GIHPHOHBGFG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.PJGBJGHPDAC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if let Some(v) = self.NNDPGGJCPOG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        for v in &self.DAELBDIDMMH {
            os.write_uint32(6, *v)?;
        };
        if let Some(v) = self.NEDIBLKMFAO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        for v in &self.EAMJPKMKKHK {
            os.write_uint32(2, *v)?;
        };
        if let Some(v) = self.OFHKCHGPKHE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
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

    fn new() -> APKPMOHIBKJ {
        APKPMOHIBKJ::new()
    }

    fn clear(&mut self) {
        self.GIHPHOHBGFG.clear();
        self.PJGBJGHPDAC.clear();
        self.NNDPGGJCPOG.clear();
        self.DAELBDIDMMH.clear();
        self.NEDIBLKMFAO.clear();
        self.EAMJPKMKKHK.clear();
        self.OFHKCHGPKHE.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static APKPMOHIBKJ {
        static instance: APKPMOHIBKJ = APKPMOHIBKJ {
            GIHPHOHBGFG: ::protobuf::MessageField::none(),
            PJGBJGHPDAC: ::protobuf::MessageField::none(),
            NNDPGGJCPOG: ::protobuf::MessageField::none(),
            DAELBDIDMMH: ::std::vec::Vec::new(),
            NEDIBLKMFAO: ::protobuf::MessageField::none(),
            EAMJPKMKKHK: ::std::vec::Vec::new(),
            OFHKCHGPKHE: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for APKPMOHIBKJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("APKPMOHIBKJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for APKPMOHIBKJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for APKPMOHIBKJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11APKPMOHIBKJ.proto\x1a\x11ACDOIEAHOKK.proto\x1a\x11CFHNFGHEJOB.prot\
    o\x1a\x11EMEGHPNHPFG.proto\x1a\x11FKGKOEMFGEO.proto\x1a\x11GIIPEENECMB.p\
    roto\"\xc1\x02\n\x0bAPKPMOHIBKJ\x12.\n\x0bGIHPHOHBGFG\x18\x0f\x20\x01(\
    \x0b2\x0c.EMEGHPNHPFGR\x0bGIHPHOHBGFG\x12.\n\x0bPJGBJGHPDAC\x18\x08\x20\
    \x01(\x0b2\x0c.GIIPEENECMBR\x0bPJGBJGHPDAC\x12.\n\x0bNNDPGGJCPOG\x18\x07\
    \x20\x01(\x0b2\x0c.CFHNFGHEJOBR\x0bNNDPGGJCPOG\x12\x20\n\x0bDAELBDIDMMH\
    \x18\x06\x20\x03(\rR\x0bDAELBDIDMMH\x12.\n\x0bNEDIBLKMFAO\x18\x04\x20\
    \x01(\x0b2\x0c.FKGKOEMFGEOR\x0bNEDIBLKMFAO\x12\x20\n\x0bEAMJPKMKKHK\x18\
    \x02\x20\x03(\rR\x0bEAMJPKMKKHK\x12.\n\x0bOFHKCHGPKHE\x18\x0c\x20\x01(\
    \x0b2\x0c.ACDOIEAHOKKR\x0bOFHKCHGPKHEb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::ACDOIEAHOKK::file_descriptor().clone());
            deps.push(super::CFHNFGHEJOB::file_descriptor().clone());
            deps.push(super::EMEGHPNHPFG::file_descriptor().clone());
            deps.push(super::FKGKOEMFGEO::file_descriptor().clone());
            deps.push(super::GIIPEENECMB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(APKPMOHIBKJ::generated_message_descriptor_data());
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
