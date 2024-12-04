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

//! Generated file from `NHOGHMMEOEO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NHOGHMMEOEO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NHOGHMMEOEO {
    // message fields
    // @@protoc_insertion_point(field:NHOGHMMEOEO.IHCNKIPIIFJ)
    pub IHCNKIPIIFJ: f64,
    // @@protoc_insertion_point(field:NHOGHMMEOEO.OJBAILGKLBM)
    pub OJBAILGKLBM: u32,
    // @@protoc_insertion_point(field:NHOGHMMEOEO.JOCEJHGHLEM)
    pub JOCEJHGHLEM: ::protobuf::MessageField<super::JAKLGMJJHNJ::JAKLGMJJHNJ>,
    // @@protoc_insertion_point(field:NHOGHMMEOEO.NIHPBAAFKEP)
    pub NIHPBAAFKEP: u32,
    // @@protoc_insertion_point(field:NHOGHMMEOEO.OFAGGKBMPJN)
    pub OFAGGKBMPJN: u32,
    // @@protoc_insertion_point(field:NHOGHMMEOEO.LAPBAEFFPOC)
    pub LAPBAEFFPOC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:NHOGHMMEOEO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NHOGHMMEOEO {
    fn default() -> &'a NHOGHMMEOEO {
        <NHOGHMMEOEO as ::protobuf::Message>::default_instance()
    }
}

impl NHOGHMMEOEO {
    pub fn new() -> NHOGHMMEOEO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IHCNKIPIIFJ",
            |m: &NHOGHMMEOEO| { &m.IHCNKIPIIFJ },
            |m: &mut NHOGHMMEOEO| { &mut m.IHCNKIPIIFJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OJBAILGKLBM",
            |m: &NHOGHMMEOEO| { &m.OJBAILGKLBM },
            |m: &mut NHOGHMMEOEO| { &mut m.OJBAILGKLBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JAKLGMJJHNJ::JAKLGMJJHNJ>(
            "JOCEJHGHLEM",
            |m: &NHOGHMMEOEO| { &m.JOCEJHGHLEM },
            |m: &mut NHOGHMMEOEO| { &mut m.JOCEJHGHLEM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NIHPBAAFKEP",
            |m: &NHOGHMMEOEO| { &m.NIHPBAAFKEP },
            |m: &mut NHOGHMMEOEO| { &mut m.NIHPBAAFKEP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OFAGGKBMPJN",
            |m: &NHOGHMMEOEO| { &m.OFAGGKBMPJN },
            |m: &mut NHOGHMMEOEO| { &mut m.OFAGGKBMPJN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LAPBAEFFPOC",
            |m: &NHOGHMMEOEO| { &m.LAPBAEFFPOC },
            |m: &mut NHOGHMMEOEO| { &mut m.LAPBAEFFPOC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NHOGHMMEOEO>(
            "NHOGHMMEOEO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NHOGHMMEOEO {
    const NAME: &'static str = "NHOGHMMEOEO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                17 => {
                    self.IHCNKIPIIFJ = is.read_double()?;
                },
                32 => {
                    self.OJBAILGKLBM = is.read_uint32()?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JOCEJHGHLEM)?;
                },
                88 => {
                    self.NIHPBAAFKEP = is.read_uint32()?;
                },
                96 => {
                    self.OFAGGKBMPJN = is.read_uint32()?;
                },
                104 => {
                    self.LAPBAEFFPOC = is.read_uint32()?;
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
        if self.IHCNKIPIIFJ != 0. {
            my_size += 1 + 8;
        }
        if self.OJBAILGKLBM != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.OJBAILGKLBM);
        }
        if let Some(v) = self.JOCEJHGHLEM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.NIHPBAAFKEP != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.NIHPBAAFKEP);
        }
        if self.OFAGGKBMPJN != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.OFAGGKBMPJN);
        }
        if self.LAPBAEFFPOC != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.LAPBAEFFPOC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IHCNKIPIIFJ != 0. {
            os.write_double(2, self.IHCNKIPIIFJ)?;
        }
        if self.OJBAILGKLBM != 0 {
            os.write_uint32(4, self.OJBAILGKLBM)?;
        }
        if let Some(v) = self.JOCEJHGHLEM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if self.NIHPBAAFKEP != 0 {
            os.write_uint32(11, self.NIHPBAAFKEP)?;
        }
        if self.OFAGGKBMPJN != 0 {
            os.write_uint32(12, self.OFAGGKBMPJN)?;
        }
        if self.LAPBAEFFPOC != 0 {
            os.write_uint32(13, self.LAPBAEFFPOC)?;
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

    fn new() -> NHOGHMMEOEO {
        NHOGHMMEOEO::new()
    }

    fn clear(&mut self) {
        self.IHCNKIPIIFJ = 0.;
        self.OJBAILGKLBM = 0;
        self.JOCEJHGHLEM.clear();
        self.NIHPBAAFKEP = 0;
        self.OFAGGKBMPJN = 0;
        self.LAPBAEFFPOC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NHOGHMMEOEO {
        static instance: NHOGHMMEOEO = NHOGHMMEOEO {
            IHCNKIPIIFJ: 0.,
            OJBAILGKLBM: 0,
            JOCEJHGHLEM: ::protobuf::MessageField::none(),
            NIHPBAAFKEP: 0,
            OFAGGKBMPJN: 0,
            LAPBAEFFPOC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NHOGHMMEOEO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NHOGHMMEOEO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NHOGHMMEOEO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NHOGHMMEOEO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NHOGHMMEOEO.proto\x1a\x11JAKLGMJJHNJ.proto\"\xe7\x01\n\x0bNHOGHMME\
    OEO\x12\x20\n\x0bIHCNKIPIIFJ\x18\x02\x20\x01(\x01R\x0bIHCNKIPIIFJ\x12\
    \x20\n\x0bOJBAILGKLBM\x18\x04\x20\x01(\rR\x0bOJBAILGKLBM\x12.\n\x0bJOCEJ\
    HGHLEM\x18\n\x20\x01(\x0b2\x0c.JAKLGMJJHNJR\x0bJOCEJHGHLEM\x12\x20\n\x0b\
    NIHPBAAFKEP\x18\x0b\x20\x01(\rR\x0bNIHPBAAFKEP\x12\x20\n\x0bOFAGGKBMPJN\
    \x18\x0c\x20\x01(\rR\x0bOFAGGKBMPJN\x12\x20\n\x0bLAPBAEFFPOC\x18\r\x20\
    \x01(\rR\x0bLAPBAEFFPOCb\x06proto3\
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
            deps.push(super::JAKLGMJJHNJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(NHOGHMMEOEO::generated_message_descriptor_data());
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