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

//! Generated file from `LFIEJOHPDHL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LFIEJOHPDHL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LFIEJOHPDHL {
    // message fields
    // @@protoc_insertion_point(field:LFIEJOHPDHL.EHFIEELJIAE)
    pub EHFIEELJIAE: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LFIEJOHPDHL.MONJOKKFPNH)
    pub MONJOKKFPNH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LFIEJOHPDHL.FLNOFODDJEO)
    pub FLNOFODDJEO: u32,
    // @@protoc_insertion_point(field:LFIEJOHPDHL.JJOJGPOAHFC)
    pub JJOJGPOAHFC: u32,
    // @@protoc_insertion_point(field:LFIEJOHPDHL.GMDFMOMMAFF)
    pub GMDFMOMMAFF: u32,
    // @@protoc_insertion_point(field:LFIEJOHPDHL.EKABPINKLHH)
    pub EKABPINKLHH: bool,
    // special fields
    // @@protoc_insertion_point(special_field:LFIEJOHPDHL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LFIEJOHPDHL {
    fn default() -> &'a LFIEJOHPDHL {
        <LFIEJOHPDHL as ::protobuf::Message>::default_instance()
    }
}

impl LFIEJOHPDHL {
    pub fn new() -> LFIEJOHPDHL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EHFIEELJIAE",
            |m: &LFIEJOHPDHL| { &m.EHFIEELJIAE },
            |m: &mut LFIEJOHPDHL| { &mut m.EHFIEELJIAE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MONJOKKFPNH",
            |m: &LFIEJOHPDHL| { &m.MONJOKKFPNH },
            |m: &mut LFIEJOHPDHL| { &mut m.MONJOKKFPNH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FLNOFODDJEO",
            |m: &LFIEJOHPDHL| { &m.FLNOFODDJEO },
            |m: &mut LFIEJOHPDHL| { &mut m.FLNOFODDJEO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JJOJGPOAHFC",
            |m: &LFIEJOHPDHL| { &m.JJOJGPOAHFC },
            |m: &mut LFIEJOHPDHL| { &mut m.JJOJGPOAHFC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GMDFMOMMAFF",
            |m: &LFIEJOHPDHL| { &m.GMDFMOMMAFF },
            |m: &mut LFIEJOHPDHL| { &mut m.GMDFMOMMAFF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EKABPINKLHH",
            |m: &LFIEJOHPDHL| { &m.EKABPINKLHH },
            |m: &mut LFIEJOHPDHL| { &mut m.EKABPINKLHH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LFIEJOHPDHL>(
            "LFIEJOHPDHL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LFIEJOHPDHL {
    const NAME: &'static str = "LFIEJOHPDHL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.EHFIEELJIAE)?;
                },
                56 => {
                    self.EHFIEELJIAE.push(is.read_uint32()?);
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.MONJOKKFPNH)?;
                },
                72 => {
                    self.MONJOKKFPNH.push(is.read_uint32()?);
                },
                88 => {
                    self.FLNOFODDJEO = is.read_uint32()?;
                },
                80 => {
                    self.JJOJGPOAHFC = is.read_uint32()?;
                },
                96 => {
                    self.GMDFMOMMAFF = is.read_uint32()?;
                },
                32 => {
                    self.EKABPINKLHH = is.read_bool()?;
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
        for value in &self.EHFIEELJIAE {
            my_size += ::protobuf::rt::uint32_size(7, *value);
        };
        for value in &self.MONJOKKFPNH {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        if self.FLNOFODDJEO != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.FLNOFODDJEO);
        }
        if self.JJOJGPOAHFC != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.JJOJGPOAHFC);
        }
        if self.GMDFMOMMAFF != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.GMDFMOMMAFF);
        }
        if self.EKABPINKLHH != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.EHFIEELJIAE {
            os.write_uint32(7, *v)?;
        };
        for v in &self.MONJOKKFPNH {
            os.write_uint32(9, *v)?;
        };
        if self.FLNOFODDJEO != 0 {
            os.write_uint32(11, self.FLNOFODDJEO)?;
        }
        if self.JJOJGPOAHFC != 0 {
            os.write_uint32(10, self.JJOJGPOAHFC)?;
        }
        if self.GMDFMOMMAFF != 0 {
            os.write_uint32(12, self.GMDFMOMMAFF)?;
        }
        if self.EKABPINKLHH != false {
            os.write_bool(4, self.EKABPINKLHH)?;
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

    fn new() -> LFIEJOHPDHL {
        LFIEJOHPDHL::new()
    }

    fn clear(&mut self) {
        self.EHFIEELJIAE.clear();
        self.MONJOKKFPNH.clear();
        self.FLNOFODDJEO = 0;
        self.JJOJGPOAHFC = 0;
        self.GMDFMOMMAFF = 0;
        self.EKABPINKLHH = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LFIEJOHPDHL {
        static instance: LFIEJOHPDHL = LFIEJOHPDHL {
            EHFIEELJIAE: ::std::vec::Vec::new(),
            MONJOKKFPNH: ::std::vec::Vec::new(),
            FLNOFODDJEO: 0,
            JJOJGPOAHFC: 0,
            GMDFMOMMAFF: 0,
            EKABPINKLHH: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LFIEJOHPDHL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LFIEJOHPDHL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LFIEJOHPDHL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LFIEJOHPDHL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LFIEJOHPDHL.proto\"\xd9\x01\n\x0bLFIEJOHPDHL\x12\x20\n\x0bEHFIEELJ\
    IAE\x18\x07\x20\x03(\rR\x0bEHFIEELJIAE\x12\x20\n\x0bMONJOKKFPNH\x18\t\
    \x20\x03(\rR\x0bMONJOKKFPNH\x12\x20\n\x0bFLNOFODDJEO\x18\x0b\x20\x01(\rR\
    \x0bFLNOFODDJEO\x12\x20\n\x0bJJOJGPOAHFC\x18\n\x20\x01(\rR\x0bJJOJGPOAHF\
    C\x12\x20\n\x0bGMDFMOMMAFF\x18\x0c\x20\x01(\rR\x0bGMDFMOMMAFF\x12\x20\n\
    \x0bEKABPINKLHH\x18\x04\x20\x01(\x08R\x0bEKABPINKLHHb\x06proto3\
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
            messages.push(LFIEJOHPDHL::generated_message_descriptor_data());
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
