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

//! Generated file from `GetPunkLordBattleRecordScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetPunkLordBattleRecordScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetPunkLordBattleRecordScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetPunkLordBattleRecordScRsp.ANAGKANMOBM)
    pub ANAGKANMOBM: ::protobuf::MessageField<super::JMEOKPPCBHL::JMEOKPPCBHL>,
    // @@protoc_insertion_point(field:GetPunkLordBattleRecordScRsp.LMJJEJMPMCJ)
    pub LMJJEJMPMCJ: ::std::vec::Vec<super::PunkLordBattleRecord::PunkLordBattleRecord>,
    // @@protoc_insertion_point(field:GetPunkLordBattleRecordScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:GetPunkLordBattleRecordScRsp.KHDICNCFIAP)
    pub KHDICNCFIAP: ::std::vec::Vec<super::PunkLordBattleReplay::PunkLordBattleReplay>,
    // special fields
    // @@protoc_insertion_point(special_field:GetPunkLordBattleRecordScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetPunkLordBattleRecordScRsp {
    fn default() -> &'a GetPunkLordBattleRecordScRsp {
        <GetPunkLordBattleRecordScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetPunkLordBattleRecordScRsp {
    pub fn new() -> GetPunkLordBattleRecordScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JMEOKPPCBHL::JMEOKPPCBHL>(
            "ANAGKANMOBM",
            |m: &GetPunkLordBattleRecordScRsp| { &m.ANAGKANMOBM },
            |m: &mut GetPunkLordBattleRecordScRsp| { &mut m.ANAGKANMOBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LMJJEJMPMCJ",
            |m: &GetPunkLordBattleRecordScRsp| { &m.LMJJEJMPMCJ },
            |m: &mut GetPunkLordBattleRecordScRsp| { &mut m.LMJJEJMPMCJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &GetPunkLordBattleRecordScRsp| { &m.ADADHIHDHJC },
            |m: &mut GetPunkLordBattleRecordScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KHDICNCFIAP",
            |m: &GetPunkLordBattleRecordScRsp| { &m.KHDICNCFIAP },
            |m: &mut GetPunkLordBattleRecordScRsp| { &mut m.KHDICNCFIAP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetPunkLordBattleRecordScRsp>(
            "GetPunkLordBattleRecordScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetPunkLordBattleRecordScRsp {
    const NAME: &'static str = "GetPunkLordBattleRecordScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ANAGKANMOBM)?;
                },
                66 => {
                    self.LMJJEJMPMCJ.push(is.read_message()?);
                },
                72 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                10 => {
                    self.KHDICNCFIAP.push(is.read_message()?);
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
        if let Some(v) = self.ANAGKANMOBM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.LMJJEJMPMCJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.ADADHIHDHJC);
        }
        for value in &self.KHDICNCFIAP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.ANAGKANMOBM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        for v in &self.LMJJEJMPMCJ {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(9, self.ADADHIHDHJC)?;
        }
        for v in &self.KHDICNCFIAP {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> GetPunkLordBattleRecordScRsp {
        GetPunkLordBattleRecordScRsp::new()
    }

    fn clear(&mut self) {
        self.ANAGKANMOBM.clear();
        self.LMJJEJMPMCJ.clear();
        self.ADADHIHDHJC = 0;
        self.KHDICNCFIAP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetPunkLordBattleRecordScRsp {
        static instance: GetPunkLordBattleRecordScRsp = GetPunkLordBattleRecordScRsp {
            ANAGKANMOBM: ::protobuf::MessageField::none(),
            LMJJEJMPMCJ: ::std::vec::Vec::new(),
            ADADHIHDHJC: 0,
            KHDICNCFIAP: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetPunkLordBattleRecordScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetPunkLordBattleRecordScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetPunkLordBattleRecordScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetPunkLordBattleRecordScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"GetPunkLordBattleRecordScRsp.proto\x1a\x11JMEOKPPCBHL.proto\x1a\x1aP\
    unkLordBattleRecord.proto\x1a\x1aPunkLordBattleReplay.proto\"\xe2\x01\n\
    \x1cGetPunkLordBattleRecordScRsp\x12.\n\x0bANAGKANMOBM\x18\x07\x20\x01(\
    \x0b2\x0c.JMEOKPPCBHLR\x0bANAGKANMOBM\x127\n\x0bLMJJEJMPMCJ\x18\x08\x20\
    \x03(\x0b2\x15.PunkLordBattleRecordR\x0bLMJJEJMPMCJ\x12\x20\n\x0bADADHIH\
    DHJC\x18\t\x20\x01(\rR\x0bADADHIHDHJC\x127\n\x0bKHDICNCFIAP\x18\x01\x20\
    \x03(\x0b2\x15.PunkLordBattleReplayR\x0bKHDICNCFIAPb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::JMEOKPPCBHL::file_descriptor().clone());
            deps.push(super::PunkLordBattleRecord::file_descriptor().clone());
            deps.push(super::PunkLordBattleReplay::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetPunkLordBattleRecordScRsp::generated_message_descriptor_data());
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