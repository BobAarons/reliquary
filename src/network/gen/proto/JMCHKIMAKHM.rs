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

//! Generated file from `JMCHKIMAKHM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:JMCHKIMAKHM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JMCHKIMAKHM {
    // message fields
    // @@protoc_insertion_point(field:JMCHKIMAKHM.FKMOJLILEDA)
    pub FKMOJLILEDA: u32,
    // @@protoc_insertion_point(field:JMCHKIMAKHM.JOLOKJLDABC)
    pub JOLOKJLDABC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:JMCHKIMAKHM.OEENAMOKFOO)
    pub OEENAMOKFOO: u32,
    // @@protoc_insertion_point(field:JMCHKIMAKHM.IELKKJBIOJA)
    pub IELKKJBIOJA: u32,
    // @@protoc_insertion_point(field:JMCHKIMAKHM.CMGGJGKHEOB)
    pub CMGGJGKHEOB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:JMCHKIMAKHM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JMCHKIMAKHM {
    fn default() -> &'a JMCHKIMAKHM {
        <JMCHKIMAKHM as ::protobuf::Message>::default_instance()
    }
}

impl JMCHKIMAKHM {
    pub fn new() -> JMCHKIMAKHM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FKMOJLILEDA",
            |m: &JMCHKIMAKHM| { &m.FKMOJLILEDA },
            |m: &mut JMCHKIMAKHM| { &mut m.FKMOJLILEDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JOLOKJLDABC",
            |m: &JMCHKIMAKHM| { &m.JOLOKJLDABC },
            |m: &mut JMCHKIMAKHM| { &mut m.JOLOKJLDABC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OEENAMOKFOO",
            |m: &JMCHKIMAKHM| { &m.OEENAMOKFOO },
            |m: &mut JMCHKIMAKHM| { &mut m.OEENAMOKFOO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IELKKJBIOJA",
            |m: &JMCHKIMAKHM| { &m.IELKKJBIOJA },
            |m: &mut JMCHKIMAKHM| { &mut m.IELKKJBIOJA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CMGGJGKHEOB",
            |m: &JMCHKIMAKHM| { &m.CMGGJGKHEOB },
            |m: &mut JMCHKIMAKHM| { &mut m.CMGGJGKHEOB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JMCHKIMAKHM>(
            "JMCHKIMAKHM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JMCHKIMAKHM {
    const NAME: &'static str = "JMCHKIMAKHM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.FKMOJLILEDA = is.read_uint32()?;
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.JOLOKJLDABC)?;
                },
                96 => {
                    self.JOLOKJLDABC.push(is.read_uint32()?);
                },
                56 => {
                    self.OEENAMOKFOO = is.read_uint32()?;
                },
                40 => {
                    self.IELKKJBIOJA = is.read_uint32()?;
                },
                104 => {
                    self.CMGGJGKHEOB = is.read_uint32()?;
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
        if self.FKMOJLILEDA != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.FKMOJLILEDA);
        }
        for value in &self.JOLOKJLDABC {
            my_size += ::protobuf::rt::uint32_size(12, *value);
        };
        if self.OEENAMOKFOO != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.OEENAMOKFOO);
        }
        if self.IELKKJBIOJA != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.IELKKJBIOJA);
        }
        if self.CMGGJGKHEOB != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.CMGGJGKHEOB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FKMOJLILEDA != 0 {
            os.write_uint32(4, self.FKMOJLILEDA)?;
        }
        for v in &self.JOLOKJLDABC {
            os.write_uint32(12, *v)?;
        };
        if self.OEENAMOKFOO != 0 {
            os.write_uint32(7, self.OEENAMOKFOO)?;
        }
        if self.IELKKJBIOJA != 0 {
            os.write_uint32(5, self.IELKKJBIOJA)?;
        }
        if self.CMGGJGKHEOB != 0 {
            os.write_uint32(13, self.CMGGJGKHEOB)?;
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

    fn new() -> JMCHKIMAKHM {
        JMCHKIMAKHM::new()
    }

    fn clear(&mut self) {
        self.FKMOJLILEDA = 0;
        self.JOLOKJLDABC.clear();
        self.OEENAMOKFOO = 0;
        self.IELKKJBIOJA = 0;
        self.CMGGJGKHEOB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JMCHKIMAKHM {
        static instance: JMCHKIMAKHM = JMCHKIMAKHM {
            FKMOJLILEDA: 0,
            JOLOKJLDABC: ::std::vec::Vec::new(),
            OEENAMOKFOO: 0,
            IELKKJBIOJA: 0,
            CMGGJGKHEOB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JMCHKIMAKHM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JMCHKIMAKHM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JMCHKIMAKHM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JMCHKIMAKHM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JMCHKIMAKHM.proto\"\xb7\x01\n\x0bJMCHKIMAKHM\x12\x20\n\x0bFKMOJLIL\
    EDA\x18\x04\x20\x01(\rR\x0bFKMOJLILEDA\x12\x20\n\x0bJOLOKJLDABC\x18\x0c\
    \x20\x03(\rR\x0bJOLOKJLDABC\x12\x20\n\x0bOEENAMOKFOO\x18\x07\x20\x01(\rR\
    \x0bOEENAMOKFOO\x12\x20\n\x0bIELKKJBIOJA\x18\x05\x20\x01(\rR\x0bIELKKJBI\
    OJA\x12\x20\n\x0bCMGGJGKHEOB\x18\r\x20\x01(\rR\x0bCMGGJGKHEOBb\x06proto3\
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
            messages.push(JMCHKIMAKHM::generated_message_descriptor_data());
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