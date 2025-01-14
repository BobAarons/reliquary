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

//! Generated file from `LAELHENDCOG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LAELHENDCOG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LAELHENDCOG {
    // message fields
    // @@protoc_insertion_point(field:LAELHENDCOG.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:LAELHENDCOG.JNFBNHBGCOJ)
    pub JNFBNHBGCOJ: ::std::string::String,
    // @@protoc_insertion_point(field:LAELHENDCOG.LGFGDDDDJBN)
    pub LGFGDDDDJBN: ::std::string::String,
    // @@protoc_insertion_point(field:LAELHENDCOG.OCHCLHDIJGG)
    pub OCHCLHDIJGG: ::std::vec::Vec<super::OHHJFLCLCCD::OHHJFLCLCCD>,
    // @@protoc_insertion_point(field:LAELHENDCOG.CLGIONDNGMP)
    pub CLGIONDNGMP: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:LAELHENDCOG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LAELHENDCOG {
    fn default() -> &'a LAELHENDCOG {
        <LAELHENDCOG as ::protobuf::Message>::default_instance()
    }
}

impl LAELHENDCOG {
    pub fn new() -> LAELHENDCOG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &LAELHENDCOG| { &m.ADADHIHDHJC },
            |m: &mut LAELHENDCOG| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JNFBNHBGCOJ",
            |m: &LAELHENDCOG| { &m.JNFBNHBGCOJ },
            |m: &mut LAELHENDCOG| { &mut m.JNFBNHBGCOJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LGFGDDDDJBN",
            |m: &LAELHENDCOG| { &m.LGFGDDDDJBN },
            |m: &mut LAELHENDCOG| { &mut m.LGFGDDDDJBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OCHCLHDIJGG",
            |m: &LAELHENDCOG| { &m.OCHCLHDIJGG },
            |m: &mut LAELHENDCOG| { &mut m.OCHCLHDIJGG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CLGIONDNGMP",
            |m: &LAELHENDCOG| { &m.CLGIONDNGMP },
            |m: &mut LAELHENDCOG| { &mut m.CLGIONDNGMP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LAELHENDCOG>(
            "LAELHENDCOG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LAELHENDCOG {
    const NAME: &'static str = "LAELHENDCOG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                18 => {
                    self.JNFBNHBGCOJ = is.read_string()?;
                },
                26 => {
                    self.LGFGDDDDJBN = is.read_string()?;
                },
                34 => {
                    self.OCHCLHDIJGG.push(is.read_message()?);
                },
                42 => {
                    self.CLGIONDNGMP = is.read_string()?;
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
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.ADADHIHDHJC);
        }
        if !self.JNFBNHBGCOJ.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.JNFBNHBGCOJ);
        }
        if !self.LGFGDDDDJBN.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.LGFGDDDDJBN);
        }
        for value in &self.OCHCLHDIJGG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if !self.CLGIONDNGMP.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.CLGIONDNGMP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(1, self.ADADHIHDHJC)?;
        }
        if !self.JNFBNHBGCOJ.is_empty() {
            os.write_string(2, &self.JNFBNHBGCOJ)?;
        }
        if !self.LGFGDDDDJBN.is_empty() {
            os.write_string(3, &self.LGFGDDDDJBN)?;
        }
        for v in &self.OCHCLHDIJGG {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if !self.CLGIONDNGMP.is_empty() {
            os.write_string(5, &self.CLGIONDNGMP)?;
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

    fn new() -> LAELHENDCOG {
        LAELHENDCOG::new()
    }

    fn clear(&mut self) {
        self.ADADHIHDHJC = 0;
        self.JNFBNHBGCOJ.clear();
        self.LGFGDDDDJBN.clear();
        self.OCHCLHDIJGG.clear();
        self.CLGIONDNGMP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LAELHENDCOG {
        static instance: LAELHENDCOG = LAELHENDCOG {
            ADADHIHDHJC: 0,
            JNFBNHBGCOJ: ::std::string::String::new(),
            LGFGDDDDJBN: ::std::string::String::new(),
            OCHCLHDIJGG: ::std::vec::Vec::new(),
            CLGIONDNGMP: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LAELHENDCOG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LAELHENDCOG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LAELHENDCOG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LAELHENDCOG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LAELHENDCOG.proto\x1a\x11OHHJFLCLCCD.proto\"\xc5\x01\n\x0bLAELHEND\
    COG\x12\x20\n\x0bADADHIHDHJC\x18\x01\x20\x01(\rR\x0bADADHIHDHJC\x12\x20\
    \n\x0bJNFBNHBGCOJ\x18\x02\x20\x01(\tR\x0bJNFBNHBGCOJ\x12\x20\n\x0bLGFGDD\
    DDJBN\x18\x03\x20\x01(\tR\x0bLGFGDDDDJBN\x12.\n\x0bOCHCLHDIJGG\x18\x04\
    \x20\x03(\x0b2\x0c.OHHJFLCLCCDR\x0bOCHCLHDIJGG\x12\x20\n\x0bCLGIONDNGMP\
    \x18\x05\x20\x01(\tR\x0bCLGIONDNGMPb\x06proto3\
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
            deps.push(super::OHHJFLCLCCD::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LAELHENDCOG::generated_message_descriptor_data());
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
