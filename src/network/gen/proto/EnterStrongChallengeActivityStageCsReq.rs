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

//! Generated file from `EnterStrongChallengeActivityStageCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EnterStrongChallengeActivityStageCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EnterStrongChallengeActivityStageCsReq {
    // message fields
    // @@protoc_insertion_point(field:EnterStrongChallengeActivityStageCsReq.EMALNMLGANJ)
    pub EMALNMLGANJ: ::std::vec::Vec<super::StrongChallengeAvatar::StrongChallengeAvatar>,
    // @@protoc_insertion_point(field:EnterStrongChallengeActivityStageCsReq.EBDDNGHLIGH)
    pub EBDDNGHLIGH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:EnterStrongChallengeActivityStageCsReq.KAGEGBLHJDJ)
    pub KAGEGBLHJDJ: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EnterStrongChallengeActivityStageCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnterStrongChallengeActivityStageCsReq {
    fn default() -> &'a EnterStrongChallengeActivityStageCsReq {
        <EnterStrongChallengeActivityStageCsReq as ::protobuf::Message>::default_instance()
    }
}

impl EnterStrongChallengeActivityStageCsReq {
    pub fn new() -> EnterStrongChallengeActivityStageCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EMALNMLGANJ",
            |m: &EnterStrongChallengeActivityStageCsReq| { &m.EMALNMLGANJ },
            |m: &mut EnterStrongChallengeActivityStageCsReq| { &mut m.EMALNMLGANJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EBDDNGHLIGH",
            |m: &EnterStrongChallengeActivityStageCsReq| { &m.EBDDNGHLIGH },
            |m: &mut EnterStrongChallengeActivityStageCsReq| { &mut m.EBDDNGHLIGH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KAGEGBLHJDJ",
            |m: &EnterStrongChallengeActivityStageCsReq| { &m.KAGEGBLHJDJ },
            |m: &mut EnterStrongChallengeActivityStageCsReq| { &mut m.KAGEGBLHJDJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnterStrongChallengeActivityStageCsReq>(
            "EnterStrongChallengeActivityStageCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnterStrongChallengeActivityStageCsReq {
    const NAME: &'static str = "EnterStrongChallengeActivityStageCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    self.EMALNMLGANJ.push(is.read_message()?);
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.EBDDNGHLIGH)?;
                },
                80 => {
                    self.EBDDNGHLIGH.push(is.read_uint32()?);
                },
                112 => {
                    self.KAGEGBLHJDJ = is.read_uint32()?;
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
        for value in &self.EMALNMLGANJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.EBDDNGHLIGH {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        if self.KAGEGBLHJDJ != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.KAGEGBLHJDJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.EMALNMLGANJ {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        for v in &self.EBDDNGHLIGH {
            os.write_uint32(10, *v)?;
        };
        if self.KAGEGBLHJDJ != 0 {
            os.write_uint32(14, self.KAGEGBLHJDJ)?;
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

    fn new() -> EnterStrongChallengeActivityStageCsReq {
        EnterStrongChallengeActivityStageCsReq::new()
    }

    fn clear(&mut self) {
        self.EMALNMLGANJ.clear();
        self.EBDDNGHLIGH.clear();
        self.KAGEGBLHJDJ = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnterStrongChallengeActivityStageCsReq {
        static instance: EnterStrongChallengeActivityStageCsReq = EnterStrongChallengeActivityStageCsReq {
            EMALNMLGANJ: ::std::vec::Vec::new(),
            EBDDNGHLIGH: ::std::vec::Vec::new(),
            KAGEGBLHJDJ: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnterStrongChallengeActivityStageCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnterStrongChallengeActivityStageCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnterStrongChallengeActivityStageCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnterStrongChallengeActivityStageCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n,EnterStrongChallengeActivityStageCsReq.proto\x1a\x1bStrongChallengeAv\
    atar.proto\"\xa6\x01\n&EnterStrongChallengeActivityStageCsReq\x128\n\x0b\
    EMALNMLGANJ\x18\x0c\x20\x03(\x0b2\x16.StrongChallengeAvatarR\x0bEMALNMLG\
    ANJ\x12\x20\n\x0bEBDDNGHLIGH\x18\n\x20\x03(\rR\x0bEBDDNGHLIGH\x12\x20\n\
    \x0bKAGEGBLHJDJ\x18\x0e\x20\x01(\rR\x0bKAGEGBLHJDJb\x06proto3\
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
            deps.push(super::StrongChallengeAvatar::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EnterStrongChallengeActivityStageCsReq::generated_message_descriptor_data());
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