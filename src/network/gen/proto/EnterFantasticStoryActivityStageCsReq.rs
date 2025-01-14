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

//! Generated file from `EnterFantasticStoryActivityStageCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EnterFantasticStoryActivityStageCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EnterFantasticStoryActivityStageCsReq {
    // message fields
    // @@protoc_insertion_point(field:EnterFantasticStoryActivityStageCsReq.MGIEBBICLCK)
    pub MGIEBBICLCK: u32,
    // @@protoc_insertion_point(field:EnterFantasticStoryActivityStageCsReq.EMALNMLGANJ)
    pub EMALNMLGANJ: ::std::vec::Vec<super::FFMHCHHHGBA::FFMHCHHHGBA>,
    // @@protoc_insertion_point(field:EnterFantasticStoryActivityStageCsReq.EBDDNGHLIGH)
    pub EBDDNGHLIGH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:EnterFantasticStoryActivityStageCsReq.AIHBDNKBNMH)
    pub AIHBDNKBNMH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EnterFantasticStoryActivityStageCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnterFantasticStoryActivityStageCsReq {
    fn default() -> &'a EnterFantasticStoryActivityStageCsReq {
        <EnterFantasticStoryActivityStageCsReq as ::protobuf::Message>::default_instance()
    }
}

impl EnterFantasticStoryActivityStageCsReq {
    pub fn new() -> EnterFantasticStoryActivityStageCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MGIEBBICLCK",
            |m: &EnterFantasticStoryActivityStageCsReq| { &m.MGIEBBICLCK },
            |m: &mut EnterFantasticStoryActivityStageCsReq| { &mut m.MGIEBBICLCK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EMALNMLGANJ",
            |m: &EnterFantasticStoryActivityStageCsReq| { &m.EMALNMLGANJ },
            |m: &mut EnterFantasticStoryActivityStageCsReq| { &mut m.EMALNMLGANJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EBDDNGHLIGH",
            |m: &EnterFantasticStoryActivityStageCsReq| { &m.EBDDNGHLIGH },
            |m: &mut EnterFantasticStoryActivityStageCsReq| { &mut m.EBDDNGHLIGH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AIHBDNKBNMH",
            |m: &EnterFantasticStoryActivityStageCsReq| { &m.AIHBDNKBNMH },
            |m: &mut EnterFantasticStoryActivityStageCsReq| { &mut m.AIHBDNKBNMH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnterFantasticStoryActivityStageCsReq>(
            "EnterFantasticStoryActivityStageCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnterFantasticStoryActivityStageCsReq {
    const NAME: &'static str = "EnterFantasticStoryActivityStageCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.MGIEBBICLCK = is.read_uint32()?;
                },
                10 => {
                    self.EMALNMLGANJ.push(is.read_message()?);
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.EBDDNGHLIGH)?;
                },
                64 => {
                    self.EBDDNGHLIGH.push(is.read_uint32()?);
                },
                96 => {
                    self.AIHBDNKBNMH = is.read_uint32()?;
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
        if self.MGIEBBICLCK != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.MGIEBBICLCK);
        }
        for value in &self.EMALNMLGANJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.EBDDNGHLIGH {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        if self.AIHBDNKBNMH != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.AIHBDNKBNMH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.MGIEBBICLCK != 0 {
            os.write_uint32(9, self.MGIEBBICLCK)?;
        }
        for v in &self.EMALNMLGANJ {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        for v in &self.EBDDNGHLIGH {
            os.write_uint32(8, *v)?;
        };
        if self.AIHBDNKBNMH != 0 {
            os.write_uint32(12, self.AIHBDNKBNMH)?;
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

    fn new() -> EnterFantasticStoryActivityStageCsReq {
        EnterFantasticStoryActivityStageCsReq::new()
    }

    fn clear(&mut self) {
        self.MGIEBBICLCK = 0;
        self.EMALNMLGANJ.clear();
        self.EBDDNGHLIGH.clear();
        self.AIHBDNKBNMH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnterFantasticStoryActivityStageCsReq {
        static instance: EnterFantasticStoryActivityStageCsReq = EnterFantasticStoryActivityStageCsReq {
            MGIEBBICLCK: 0,
            EMALNMLGANJ: ::std::vec::Vec::new(),
            EBDDNGHLIGH: ::std::vec::Vec::new(),
            AIHBDNKBNMH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnterFantasticStoryActivityStageCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnterFantasticStoryActivityStageCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnterFantasticStoryActivityStageCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnterFantasticStoryActivityStageCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n+EnterFantasticStoryActivityStageCsReq.proto\x1a\x11FFMHCHHHGBA.proto\
    \"\xbd\x01\n%EnterFantasticStoryActivityStageCsReq\x12\x20\n\x0bMGIEBBIC\
    LCK\x18\t\x20\x01(\rR\x0bMGIEBBICLCK\x12.\n\x0bEMALNMLGANJ\x18\x01\x20\
    \x03(\x0b2\x0c.FFMHCHHHGBAR\x0bEMALNMLGANJ\x12\x20\n\x0bEBDDNGHLIGH\x18\
    \x08\x20\x03(\rR\x0bEBDDNGHLIGH\x12\x20\n\x0bAIHBDNKBNMH\x18\x0c\x20\x01\
    (\rR\x0bAIHBDNKBNMHb\x06proto3\
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
            deps.push(super::FFMHCHHHGBA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EnterFantasticStoryActivityStageCsReq::generated_message_descriptor_data());
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
