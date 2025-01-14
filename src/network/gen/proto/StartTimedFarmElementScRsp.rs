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

//! Generated file from `StartTimedFarmElementScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:StartTimedFarmElementScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartTimedFarmElementScRsp {
    // message fields
    // @@protoc_insertion_point(field:StartTimedFarmElementScRsp.BBKGPAJCCBM)
    pub BBKGPAJCCBM: ::protobuf::MessageField<super::CHDPLFOHLCN::CHDPLFOHLCN>,
    // @@protoc_insertion_point(field:StartTimedFarmElementScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:StartTimedFarmElementScRsp.DNMJBNNJLEL)
    pub DNMJBNNJLEL: u32,
    // @@protoc_insertion_point(field:StartTimedFarmElementScRsp.MEGEJFGAKDL)
    pub MEGEJFGAKDL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:StartTimedFarmElementScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartTimedFarmElementScRsp {
    fn default() -> &'a StartTimedFarmElementScRsp {
        <StartTimedFarmElementScRsp as ::protobuf::Message>::default_instance()
    }
}

impl StartTimedFarmElementScRsp {
    pub fn new() -> StartTimedFarmElementScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CHDPLFOHLCN::CHDPLFOHLCN>(
            "BBKGPAJCCBM",
            |m: &StartTimedFarmElementScRsp| { &m.BBKGPAJCCBM },
            |m: &mut StartTimedFarmElementScRsp| { &mut m.BBKGPAJCCBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &StartTimedFarmElementScRsp| { &m.ADADHIHDHJC },
            |m: &mut StartTimedFarmElementScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNMJBNNJLEL",
            |m: &StartTimedFarmElementScRsp| { &m.DNMJBNNJLEL },
            |m: &mut StartTimedFarmElementScRsp| { &mut m.DNMJBNNJLEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MEGEJFGAKDL",
            |m: &StartTimedFarmElementScRsp| { &m.MEGEJFGAKDL },
            |m: &mut StartTimedFarmElementScRsp| { &mut m.MEGEJFGAKDL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartTimedFarmElementScRsp>(
            "StartTimedFarmElementScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartTimedFarmElementScRsp {
    const NAME: &'static str = "StartTimedFarmElementScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BBKGPAJCCBM)?;
                },
                112 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                8 => {
                    self.DNMJBNNJLEL = is.read_uint32()?;
                },
                64 => {
                    self.MEGEJFGAKDL = is.read_uint32()?;
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
        if let Some(v) = self.BBKGPAJCCBM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.ADADHIHDHJC);
        }
        if self.DNMJBNNJLEL != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.DNMJBNNJLEL);
        }
        if self.MEGEJFGAKDL != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.MEGEJFGAKDL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.BBKGPAJCCBM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(14, self.ADADHIHDHJC)?;
        }
        if self.DNMJBNNJLEL != 0 {
            os.write_uint32(1, self.DNMJBNNJLEL)?;
        }
        if self.MEGEJFGAKDL != 0 {
            os.write_uint32(8, self.MEGEJFGAKDL)?;
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

    fn new() -> StartTimedFarmElementScRsp {
        StartTimedFarmElementScRsp::new()
    }

    fn clear(&mut self) {
        self.BBKGPAJCCBM.clear();
        self.ADADHIHDHJC = 0;
        self.DNMJBNNJLEL = 0;
        self.MEGEJFGAKDL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartTimedFarmElementScRsp {
        static instance: StartTimedFarmElementScRsp = StartTimedFarmElementScRsp {
            BBKGPAJCCBM: ::protobuf::MessageField::none(),
            ADADHIHDHJC: 0,
            DNMJBNNJLEL: 0,
            MEGEJFGAKDL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartTimedFarmElementScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartTimedFarmElementScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartTimedFarmElementScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartTimedFarmElementScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20StartTimedFarmElementScRsp.proto\x1a\x11CHDPLFOHLCN.proto\"\xb2\
    \x01\n\x1aStartTimedFarmElementScRsp\x12.\n\x0bBBKGPAJCCBM\x18\x03\x20\
    \x01(\x0b2\x0c.CHDPLFOHLCNR\x0bBBKGPAJCCBM\x12\x20\n\x0bADADHIHDHJC\x18\
    \x0e\x20\x01(\rR\x0bADADHIHDHJC\x12\x20\n\x0bDNMJBNNJLEL\x18\x01\x20\x01\
    (\rR\x0bDNMJBNNJLEL\x12\x20\n\x0bMEGEJFGAKDL\x18\x08\x20\x01(\rR\x0bMEGE\
    JFGAKDLb\x06proto3\
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
            deps.push(super::CHDPLFOHLCN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(StartTimedFarmElementScRsp::generated_message_descriptor_data());
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
