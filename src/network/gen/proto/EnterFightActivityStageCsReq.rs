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

//! Generated file from `EnterFightActivityStageCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EnterFightActivityStageCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EnterFightActivityStageCsReq {
    // message fields
    // @@protoc_insertion_point(field:EnterFightActivityStageCsReq.AAKFBNJMLEJ)
    pub AAKFBNJMLEJ: ::std::vec::Vec<super::DCOEJEOFDON::DCOEJEOFDON>,
    // @@protoc_insertion_point(field:EnterFightActivityStageCsReq.KONGAADEJEL)
    pub KONGAADEJEL: u32,
    // @@protoc_insertion_point(field:EnterFightActivityStageCsReq.AGPKHOOCMPE)
    pub AGPKHOOCMPE: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:EnterFightActivityStageCsReq.EMALNMLGANJ)
    pub EMALNMLGANJ: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:EnterFightActivityStageCsReq.ILIFHHJFMIH)
    pub ILIFHHJFMIH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EnterFightActivityStageCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnterFightActivityStageCsReq {
    fn default() -> &'a EnterFightActivityStageCsReq {
        <EnterFightActivityStageCsReq as ::protobuf::Message>::default_instance()
    }
}

impl EnterFightActivityStageCsReq {
    pub fn new() -> EnterFightActivityStageCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AAKFBNJMLEJ",
            |m: &EnterFightActivityStageCsReq| { &m.AAKFBNJMLEJ },
            |m: &mut EnterFightActivityStageCsReq| { &mut m.AAKFBNJMLEJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KONGAADEJEL",
            |m: &EnterFightActivityStageCsReq| { &m.KONGAADEJEL },
            |m: &mut EnterFightActivityStageCsReq| { &mut m.KONGAADEJEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AGPKHOOCMPE",
            |m: &EnterFightActivityStageCsReq| { &m.AGPKHOOCMPE },
            |m: &mut EnterFightActivityStageCsReq| { &mut m.AGPKHOOCMPE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EMALNMLGANJ",
            |m: &EnterFightActivityStageCsReq| { &m.EMALNMLGANJ },
            |m: &mut EnterFightActivityStageCsReq| { &mut m.EMALNMLGANJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ILIFHHJFMIH",
            |m: &EnterFightActivityStageCsReq| { &m.ILIFHHJFMIH },
            |m: &mut EnterFightActivityStageCsReq| { &mut m.ILIFHHJFMIH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnterFightActivityStageCsReq>(
            "EnterFightActivityStageCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnterFightActivityStageCsReq {
    const NAME: &'static str = "EnterFightActivityStageCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    self.AAKFBNJMLEJ.push(is.read_message()?);
                },
                48 => {
                    self.KONGAADEJEL = is.read_uint32()?;
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.AGPKHOOCMPE)?;
                },
                24 => {
                    self.AGPKHOOCMPE.push(is.read_uint32()?);
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.EMALNMLGANJ)?;
                },
                8 => {
                    self.EMALNMLGANJ.push(is.read_uint32()?);
                },
                72 => {
                    self.ILIFHHJFMIH = is.read_uint32()?;
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
        for value in &self.AAKFBNJMLEJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.KONGAADEJEL != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.KONGAADEJEL);
        }
        for value in &self.AGPKHOOCMPE {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        for value in &self.EMALNMLGANJ {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        if self.ILIFHHJFMIH != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.ILIFHHJFMIH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.AAKFBNJMLEJ {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        if self.KONGAADEJEL != 0 {
            os.write_uint32(6, self.KONGAADEJEL)?;
        }
        for v in &self.AGPKHOOCMPE {
            os.write_uint32(3, *v)?;
        };
        for v in &self.EMALNMLGANJ {
            os.write_uint32(1, *v)?;
        };
        if self.ILIFHHJFMIH != 0 {
            os.write_uint32(9, self.ILIFHHJFMIH)?;
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

    fn new() -> EnterFightActivityStageCsReq {
        EnterFightActivityStageCsReq::new()
    }

    fn clear(&mut self) {
        self.AAKFBNJMLEJ.clear();
        self.KONGAADEJEL = 0;
        self.AGPKHOOCMPE.clear();
        self.EMALNMLGANJ.clear();
        self.ILIFHHJFMIH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnterFightActivityStageCsReq {
        static instance: EnterFightActivityStageCsReq = EnterFightActivityStageCsReq {
            AAKFBNJMLEJ: ::std::vec::Vec::new(),
            KONGAADEJEL: 0,
            AGPKHOOCMPE: ::std::vec::Vec::new(),
            EMALNMLGANJ: ::std::vec::Vec::new(),
            ILIFHHJFMIH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnterFightActivityStageCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnterFightActivityStageCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnterFightActivityStageCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnterFightActivityStageCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"EnterFightActivityStageCsReq.proto\x1a\x11DCOEJEOFDON.proto\"\xd6\
    \x01\n\x1cEnterFightActivityStageCsReq\x12.\n\x0bAAKFBNJMLEJ\x18\x0f\x20\
    \x03(\x0b2\x0c.DCOEJEOFDONR\x0bAAKFBNJMLEJ\x12\x20\n\x0bKONGAADEJEL\x18\
    \x06\x20\x01(\rR\x0bKONGAADEJEL\x12\x20\n\x0bAGPKHOOCMPE\x18\x03\x20\x03\
    (\rR\x0bAGPKHOOCMPE\x12\x20\n\x0bEMALNMLGANJ\x18\x01\x20\x03(\rR\x0bEMAL\
    NMLGANJ\x12\x20\n\x0bILIFHHJFMIH\x18\t\x20\x01(\rR\x0bILIFHHJFMIHb\x06pr\
    oto3\
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
            deps.push(super::DCOEJEOFDON::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EnterFightActivityStageCsReq::generated_message_descriptor_data());
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