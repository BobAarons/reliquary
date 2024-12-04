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

//! Generated file from `BMIJKHMDCJJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BMIJKHMDCJJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BMIJKHMDCJJ {
    // message fields
    // @@protoc_insertion_point(field:BMIJKHMDCJJ.FJEKAIHEKPK)
    pub FJEKAIHEKPK: bool,
    // @@protoc_insertion_point(field:BMIJKHMDCJJ.OPHNNMBCKFJ)
    pub OPHNNMBCKFJ: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BMIJKHMDCJJ.LFPECAAHNIC)
    pub LFPECAAHNIC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BMIJKHMDCJJ.IMGGGODPLNN)
    pub IMGGGODPLNN: u32,
    // @@protoc_insertion_point(field:BMIJKHMDCJJ.HCHCKJINFDF)
    pub HCHCKJINFDF: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:BMIJKHMDCJJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BMIJKHMDCJJ {
    fn default() -> &'a BMIJKHMDCJJ {
        <BMIJKHMDCJJ as ::protobuf::Message>::default_instance()
    }
}

impl BMIJKHMDCJJ {
    pub fn new() -> BMIJKHMDCJJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJEKAIHEKPK",
            |m: &BMIJKHMDCJJ| { &m.FJEKAIHEKPK },
            |m: &mut BMIJKHMDCJJ| { &mut m.FJEKAIHEKPK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OPHNNMBCKFJ",
            |m: &BMIJKHMDCJJ| { &m.OPHNNMBCKFJ },
            |m: &mut BMIJKHMDCJJ| { &mut m.OPHNNMBCKFJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LFPECAAHNIC",
            |m: &BMIJKHMDCJJ| { &m.LFPECAAHNIC },
            |m: &mut BMIJKHMDCJJ| { &mut m.LFPECAAHNIC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IMGGGODPLNN",
            |m: &BMIJKHMDCJJ| { &m.IMGGGODPLNN },
            |m: &mut BMIJKHMDCJJ| { &mut m.IMGGGODPLNN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HCHCKJINFDF",
            |m: &BMIJKHMDCJJ| { &m.HCHCKJINFDF },
            |m: &mut BMIJKHMDCJJ| { &mut m.HCHCKJINFDF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BMIJKHMDCJJ>(
            "BMIJKHMDCJJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BMIJKHMDCJJ {
    const NAME: &'static str = "BMIJKHMDCJJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.FJEKAIHEKPK = is.read_bool()?;
                },
                114 => {
                    is.read_repeated_packed_uint32_into(&mut self.OPHNNMBCKFJ)?;
                },
                112 => {
                    self.OPHNNMBCKFJ.push(is.read_uint32()?);
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.LFPECAAHNIC)?;
                },
                56 => {
                    self.LFPECAAHNIC.push(is.read_uint32()?);
                },
                64 => {
                    self.IMGGGODPLNN = is.read_uint32()?;
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.HCHCKJINFDF)?;
                },
                72 => {
                    self.HCHCKJINFDF.push(is.read_uint32()?);
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
        if self.FJEKAIHEKPK != false {
            my_size += 1 + 1;
        }
        for value in &self.OPHNNMBCKFJ {
            my_size += ::protobuf::rt::uint32_size(14, *value);
        };
        for value in &self.LFPECAAHNIC {
            my_size += ::protobuf::rt::uint32_size(7, *value);
        };
        if self.IMGGGODPLNN != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.IMGGGODPLNN);
        }
        for value in &self.HCHCKJINFDF {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FJEKAIHEKPK != false {
            os.write_bool(13, self.FJEKAIHEKPK)?;
        }
        for v in &self.OPHNNMBCKFJ {
            os.write_uint32(14, *v)?;
        };
        for v in &self.LFPECAAHNIC {
            os.write_uint32(7, *v)?;
        };
        if self.IMGGGODPLNN != 0 {
            os.write_uint32(8, self.IMGGGODPLNN)?;
        }
        for v in &self.HCHCKJINFDF {
            os.write_uint32(9, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> BMIJKHMDCJJ {
        BMIJKHMDCJJ::new()
    }

    fn clear(&mut self) {
        self.FJEKAIHEKPK = false;
        self.OPHNNMBCKFJ.clear();
        self.LFPECAAHNIC.clear();
        self.IMGGGODPLNN = 0;
        self.HCHCKJINFDF.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BMIJKHMDCJJ {
        static instance: BMIJKHMDCJJ = BMIJKHMDCJJ {
            FJEKAIHEKPK: false,
            OPHNNMBCKFJ: ::std::vec::Vec::new(),
            LFPECAAHNIC: ::std::vec::Vec::new(),
            IMGGGODPLNN: 0,
            HCHCKJINFDF: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BMIJKHMDCJJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BMIJKHMDCJJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BMIJKHMDCJJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BMIJKHMDCJJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BMIJKHMDCJJ.proto\"\xb7\x01\n\x0bBMIJKHMDCJJ\x12\x20\n\x0bFJEKAIHE\
    KPK\x18\r\x20\x01(\x08R\x0bFJEKAIHEKPK\x12\x20\n\x0bOPHNNMBCKFJ\x18\x0e\
    \x20\x03(\rR\x0bOPHNNMBCKFJ\x12\x20\n\x0bLFPECAAHNIC\x18\x07\x20\x03(\rR\
    \x0bLFPECAAHNIC\x12\x20\n\x0bIMGGGODPLNN\x18\x08\x20\x01(\rR\x0bIMGGGODP\
    LNN\x12\x20\n\x0bHCHCKJINFDF\x18\t\x20\x03(\rR\x0bHCHCKJINFDFb\x06proto3\
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
            messages.push(BMIJKHMDCJJ::generated_message_descriptor_data());
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