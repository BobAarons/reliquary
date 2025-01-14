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

//! Generated file from `KHIALNHGMHN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KHIALNHGMHN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KHIALNHGMHN {
    // message fields
    // @@protoc_insertion_point(field:KHIALNHGMHN.HJCGEIDBICC)
    pub HJCGEIDBICC: bool,
    // @@protoc_insertion_point(field:KHIALNHGMHN.JFBNIJILMMI)
    pub JFBNIJILMMI: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:KHIALNHGMHN.NLPPCEJDAED)
    pub NLPPCEJDAED: u32,
    // special fields
    // @@protoc_insertion_point(special_field:KHIALNHGMHN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KHIALNHGMHN {
    fn default() -> &'a KHIALNHGMHN {
        <KHIALNHGMHN as ::protobuf::Message>::default_instance()
    }
}

impl KHIALNHGMHN {
    pub fn new() -> KHIALNHGMHN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HJCGEIDBICC",
            |m: &KHIALNHGMHN| { &m.HJCGEIDBICC },
            |m: &mut KHIALNHGMHN| { &mut m.HJCGEIDBICC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JFBNIJILMMI",
            |m: &KHIALNHGMHN| { &m.JFBNIJILMMI },
            |m: &mut KHIALNHGMHN| { &mut m.JFBNIJILMMI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NLPPCEJDAED",
            |m: &KHIALNHGMHN| { &m.NLPPCEJDAED },
            |m: &mut KHIALNHGMHN| { &mut m.NLPPCEJDAED },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KHIALNHGMHN>(
            "KHIALNHGMHN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KHIALNHGMHN {
    const NAME: &'static str = "KHIALNHGMHN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.HJCGEIDBICC = is.read_bool()?;
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.JFBNIJILMMI)?;
                },
                24 => {
                    self.JFBNIJILMMI.push(is.read_uint32()?);
                },
                112 => {
                    self.NLPPCEJDAED = is.read_uint32()?;
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
        if self.HJCGEIDBICC != false {
            my_size += 1 + 1;
        }
        for value in &self.JFBNIJILMMI {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        if self.NLPPCEJDAED != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.NLPPCEJDAED);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HJCGEIDBICC != false {
            os.write_bool(6, self.HJCGEIDBICC)?;
        }
        for v in &self.JFBNIJILMMI {
            os.write_uint32(3, *v)?;
        };
        if self.NLPPCEJDAED != 0 {
            os.write_uint32(14, self.NLPPCEJDAED)?;
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

    fn new() -> KHIALNHGMHN {
        KHIALNHGMHN::new()
    }

    fn clear(&mut self) {
        self.HJCGEIDBICC = false;
        self.JFBNIJILMMI.clear();
        self.NLPPCEJDAED = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KHIALNHGMHN {
        static instance: KHIALNHGMHN = KHIALNHGMHN {
            HJCGEIDBICC: false,
            JFBNIJILMMI: ::std::vec::Vec::new(),
            NLPPCEJDAED: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KHIALNHGMHN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KHIALNHGMHN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KHIALNHGMHN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KHIALNHGMHN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KHIALNHGMHN.proto\"s\n\x0bKHIALNHGMHN\x12\x20\n\x0bHJCGEIDBICC\x18\
    \x06\x20\x01(\x08R\x0bHJCGEIDBICC\x12\x20\n\x0bJFBNIJILMMI\x18\x03\x20\
    \x03(\rR\x0bJFBNIJILMMI\x12\x20\n\x0bNLPPCEJDAED\x18\x0e\x20\x01(\rR\x0b\
    NLPPCEJDAEDb\x06proto3\
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
            messages.push(KHIALNHGMHN::generated_message_descriptor_data());
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
