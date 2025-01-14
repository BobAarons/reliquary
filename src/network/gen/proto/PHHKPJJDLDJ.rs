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

//! Generated file from `PHHKPJJDLDJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PHHKPJJDLDJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PHHKPJJDLDJ {
    // message fields
    // @@protoc_insertion_point(field:PHHKPJJDLDJ.LJBBGBBKNDE)
    pub LJBBGBBKNDE: u32,
    // @@protoc_insertion_point(field:PHHKPJJDLDJ.JDBKMAACMMK)
    pub JDBKMAACMMK: u32,
    // @@protoc_insertion_point(field:PHHKPJJDLDJ.DDMDPBEGKHC)
    pub DDMDPBEGKHC: u32,
    // @@protoc_insertion_point(field:PHHKPJJDLDJ.OKCPHFONBHI)
    pub OKCPHFONBHI: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PHHKPJJDLDJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PHHKPJJDLDJ {
    fn default() -> &'a PHHKPJJDLDJ {
        <PHHKPJJDLDJ as ::protobuf::Message>::default_instance()
    }
}

impl PHHKPJJDLDJ {
    pub fn new() -> PHHKPJJDLDJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LJBBGBBKNDE",
            |m: &PHHKPJJDLDJ| { &m.LJBBGBBKNDE },
            |m: &mut PHHKPJJDLDJ| { &mut m.LJBBGBBKNDE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JDBKMAACMMK",
            |m: &PHHKPJJDLDJ| { &m.JDBKMAACMMK },
            |m: &mut PHHKPJJDLDJ| { &mut m.JDBKMAACMMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DDMDPBEGKHC",
            |m: &PHHKPJJDLDJ| { &m.DDMDPBEGKHC },
            |m: &mut PHHKPJJDLDJ| { &mut m.DDMDPBEGKHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OKCPHFONBHI",
            |m: &PHHKPJJDLDJ| { &m.OKCPHFONBHI },
            |m: &mut PHHKPJJDLDJ| { &mut m.OKCPHFONBHI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PHHKPJJDLDJ>(
            "PHHKPJJDLDJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PHHKPJJDLDJ {
    const NAME: &'static str = "PHHKPJJDLDJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.LJBBGBBKNDE = is.read_uint32()?;
                },
                24 => {
                    self.JDBKMAACMMK = is.read_uint32()?;
                },
                16 => {
                    self.DDMDPBEGKHC = is.read_uint32()?;
                },
                120 => {
                    self.OKCPHFONBHI = is.read_uint32()?;
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
        if self.LJBBGBBKNDE != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.LJBBGBBKNDE);
        }
        if self.JDBKMAACMMK != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.JDBKMAACMMK);
        }
        if self.DDMDPBEGKHC != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.DDMDPBEGKHC);
        }
        if self.OKCPHFONBHI != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.OKCPHFONBHI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LJBBGBBKNDE != 0 {
            os.write_uint32(13, self.LJBBGBBKNDE)?;
        }
        if self.JDBKMAACMMK != 0 {
            os.write_uint32(3, self.JDBKMAACMMK)?;
        }
        if self.DDMDPBEGKHC != 0 {
            os.write_uint32(2, self.DDMDPBEGKHC)?;
        }
        if self.OKCPHFONBHI != 0 {
            os.write_uint32(15, self.OKCPHFONBHI)?;
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

    fn new() -> PHHKPJJDLDJ {
        PHHKPJJDLDJ::new()
    }

    fn clear(&mut self) {
        self.LJBBGBBKNDE = 0;
        self.JDBKMAACMMK = 0;
        self.DDMDPBEGKHC = 0;
        self.OKCPHFONBHI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PHHKPJJDLDJ {
        static instance: PHHKPJJDLDJ = PHHKPJJDLDJ {
            LJBBGBBKNDE: 0,
            JDBKMAACMMK: 0,
            DDMDPBEGKHC: 0,
            OKCPHFONBHI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PHHKPJJDLDJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PHHKPJJDLDJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PHHKPJJDLDJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PHHKPJJDLDJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PHHKPJJDLDJ.proto\"\x95\x01\n\x0bPHHKPJJDLDJ\x12\x20\n\x0bLJBBGBBK\
    NDE\x18\r\x20\x01(\rR\x0bLJBBGBBKNDE\x12\x20\n\x0bJDBKMAACMMK\x18\x03\
    \x20\x01(\rR\x0bJDBKMAACMMK\x12\x20\n\x0bDDMDPBEGKHC\x18\x02\x20\x01(\rR\
    \x0bDDMDPBEGKHC\x12\x20\n\x0bOKCPHFONBHI\x18\x0f\x20\x01(\rR\x0bOKCPHFON\
    BHIb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PHHKPJJDLDJ::generated_message_descriptor_data());
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
