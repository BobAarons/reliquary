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

//! Generated file from `EJNLMNONLHP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EJNLMNONLHP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EJNLMNONLHP {
    // message fields
    // @@protoc_insertion_point(field:EJNLMNONLHP.OMBLHJDKEJA)
    pub OMBLHJDKEJA: u32,
    // @@protoc_insertion_point(field:EJNLMNONLHP.OBKCAELPMJG)
    pub OBKCAELPMJG: u32,
    // @@protoc_insertion_point(field:EJNLMNONLHP.ICAFLJAGDJF)
    pub ICAFLJAGDJF: ::std::vec::Vec<super::GMELPKNHKIO::GMELPKNHKIO>,
    // @@protoc_insertion_point(field:EJNLMNONLHP.FGBHOLLBLJJ)
    pub FGBHOLLBLJJ: u32,
    // @@protoc_insertion_point(field:EJNLMNONLHP.LNGJKFGLHBE)
    pub LNGJKFGLHBE: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EJNLMNONLHP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EJNLMNONLHP {
    fn default() -> &'a EJNLMNONLHP {
        <EJNLMNONLHP as ::protobuf::Message>::default_instance()
    }
}

impl EJNLMNONLHP {
    pub fn new() -> EJNLMNONLHP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OMBLHJDKEJA",
            |m: &EJNLMNONLHP| { &m.OMBLHJDKEJA },
            |m: &mut EJNLMNONLHP| { &mut m.OMBLHJDKEJA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OBKCAELPMJG",
            |m: &EJNLMNONLHP| { &m.OBKCAELPMJG },
            |m: &mut EJNLMNONLHP| { &mut m.OBKCAELPMJG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ICAFLJAGDJF",
            |m: &EJNLMNONLHP| { &m.ICAFLJAGDJF },
            |m: &mut EJNLMNONLHP| { &mut m.ICAFLJAGDJF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FGBHOLLBLJJ",
            |m: &EJNLMNONLHP| { &m.FGBHOLLBLJJ },
            |m: &mut EJNLMNONLHP| { &mut m.FGBHOLLBLJJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LNGJKFGLHBE",
            |m: &EJNLMNONLHP| { &m.LNGJKFGLHBE },
            |m: &mut EJNLMNONLHP| { &mut m.LNGJKFGLHBE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EJNLMNONLHP>(
            "EJNLMNONLHP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EJNLMNONLHP {
    const NAME: &'static str = "EJNLMNONLHP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.OMBLHJDKEJA = is.read_uint32()?;
                },
                40 => {
                    self.OBKCAELPMJG = is.read_uint32()?;
                },
                26 => {
                    self.ICAFLJAGDJF.push(is.read_message()?);
                },
                8 => {
                    self.FGBHOLLBLJJ = is.read_uint32()?;
                },
                112 => {
                    self.LNGJKFGLHBE = is.read_uint32()?;
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
        if self.OMBLHJDKEJA != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.OMBLHJDKEJA);
        }
        if self.OBKCAELPMJG != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.OBKCAELPMJG);
        }
        for value in &self.ICAFLJAGDJF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.FGBHOLLBLJJ != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.FGBHOLLBLJJ);
        }
        if self.LNGJKFGLHBE != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.LNGJKFGLHBE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OMBLHJDKEJA != 0 {
            os.write_uint32(11, self.OMBLHJDKEJA)?;
        }
        if self.OBKCAELPMJG != 0 {
            os.write_uint32(5, self.OBKCAELPMJG)?;
        }
        for v in &self.ICAFLJAGDJF {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.FGBHOLLBLJJ != 0 {
            os.write_uint32(1, self.FGBHOLLBLJJ)?;
        }
        if self.LNGJKFGLHBE != 0 {
            os.write_uint32(14, self.LNGJKFGLHBE)?;
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

    fn new() -> EJNLMNONLHP {
        EJNLMNONLHP::new()
    }

    fn clear(&mut self) {
        self.OMBLHJDKEJA = 0;
        self.OBKCAELPMJG = 0;
        self.ICAFLJAGDJF.clear();
        self.FGBHOLLBLJJ = 0;
        self.LNGJKFGLHBE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EJNLMNONLHP {
        static instance: EJNLMNONLHP = EJNLMNONLHP {
            OMBLHJDKEJA: 0,
            OBKCAELPMJG: 0,
            ICAFLJAGDJF: ::std::vec::Vec::new(),
            FGBHOLLBLJJ: 0,
            LNGJKFGLHBE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EJNLMNONLHP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EJNLMNONLHP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EJNLMNONLHP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EJNLMNONLHP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EJNLMNONLHP.proto\x1a\x11GMELPKNHKIO.proto\"\xc5\x01\n\x0bEJNLMNON\
    LHP\x12\x20\n\x0bOMBLHJDKEJA\x18\x0b\x20\x01(\rR\x0bOMBLHJDKEJA\x12\x20\
    \n\x0bOBKCAELPMJG\x18\x05\x20\x01(\rR\x0bOBKCAELPMJG\x12.\n\x0bICAFLJAGD\
    JF\x18\x03\x20\x03(\x0b2\x0c.GMELPKNHKIOR\x0bICAFLJAGDJF\x12\x20\n\x0bFG\
    BHOLLBLJJ\x18\x01\x20\x01(\rR\x0bFGBHOLLBLJJ\x12\x20\n\x0bLNGJKFGLHBE\
    \x18\x0e\x20\x01(\rR\x0bLNGJKFGLHBEb\x06proto3\
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
            deps.push(super::GMELPKNHKIO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EJNLMNONLHP::generated_message_descriptor_data());
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