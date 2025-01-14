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

//! Generated file from `NNIDMJHAPKK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NNIDMJHAPKK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NNIDMJHAPKK {
    // message fields
    // @@protoc_insertion_point(field:NNIDMJHAPKK.APKEFKGPHIE)
    pub APKEFKGPHIE: ::protobuf::EnumOrUnknown<super::MuseumRandomEventState::MuseumRandomEventState>,
    // @@protoc_insertion_point(field:NNIDMJHAPKK.OMFGMILDJDO)
    pub OMFGMILDJDO: u32,
    // @@protoc_insertion_point(field:NNIDMJHAPKK.NNOLOOIACLG)
    pub NNOLOOIACLG: u32,
    // @@protoc_insertion_point(field:NNIDMJHAPKK.MKNHJFFHCDA)
    pub MKNHJFFHCDA: u32,
    // @@protoc_insertion_point(field:NNIDMJHAPKK.MHFANLAADFF)
    pub MHFANLAADFF: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:NNIDMJHAPKK.FKMOJLILEDA)
    pub FKMOJLILEDA: u32,
    // @@protoc_insertion_point(field:NNIDMJHAPKK.DDCGDKHHPAK)
    pub DDCGDKHHPAK: u32,
    // special fields
    // @@protoc_insertion_point(special_field:NNIDMJHAPKK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NNIDMJHAPKK {
    fn default() -> &'a NNIDMJHAPKK {
        <NNIDMJHAPKK as ::protobuf::Message>::default_instance()
    }
}

impl NNIDMJHAPKK {
    pub fn new() -> NNIDMJHAPKK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "APKEFKGPHIE",
            |m: &NNIDMJHAPKK| { &m.APKEFKGPHIE },
            |m: &mut NNIDMJHAPKK| { &mut m.APKEFKGPHIE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OMFGMILDJDO",
            |m: &NNIDMJHAPKK| { &m.OMFGMILDJDO },
            |m: &mut NNIDMJHAPKK| { &mut m.OMFGMILDJDO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NNOLOOIACLG",
            |m: &NNIDMJHAPKK| { &m.NNOLOOIACLG },
            |m: &mut NNIDMJHAPKK| { &mut m.NNOLOOIACLG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MKNHJFFHCDA",
            |m: &NNIDMJHAPKK| { &m.MKNHJFFHCDA },
            |m: &mut NNIDMJHAPKK| { &mut m.MKNHJFFHCDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MHFANLAADFF",
            |m: &NNIDMJHAPKK| { &m.MHFANLAADFF },
            |m: &mut NNIDMJHAPKK| { &mut m.MHFANLAADFF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FKMOJLILEDA",
            |m: &NNIDMJHAPKK| { &m.FKMOJLILEDA },
            |m: &mut NNIDMJHAPKK| { &mut m.FKMOJLILEDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DDCGDKHHPAK",
            |m: &NNIDMJHAPKK| { &m.DDCGDKHHPAK },
            |m: &mut NNIDMJHAPKK| { &mut m.DDCGDKHHPAK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NNIDMJHAPKK>(
            "NNIDMJHAPKK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NNIDMJHAPKK {
    const NAME: &'static str = "NNIDMJHAPKK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.APKEFKGPHIE = is.read_enum_or_unknown()?;
                },
                112 => {
                    self.OMFGMILDJDO = is.read_uint32()?;
                },
                16 => {
                    self.NNOLOOIACLG = is.read_uint32()?;
                },
                24 => {
                    self.MKNHJFFHCDA = is.read_uint32()?;
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.MHFANLAADFF)?;
                },
                120 => {
                    self.MHFANLAADFF.push(is.read_uint32()?);
                },
                64 => {
                    self.FKMOJLILEDA = is.read_uint32()?;
                },
                32 => {
                    self.DDCGDKHHPAK = is.read_uint32()?;
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
        if self.APKEFKGPHIE != ::protobuf::EnumOrUnknown::new(super::MuseumRandomEventState::MuseumRandomEventState::MUSEUM_RANDOM_EVENT_STATE_NONE) {
            my_size += ::protobuf::rt::int32_size(13, self.APKEFKGPHIE.value());
        }
        if self.OMFGMILDJDO != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.OMFGMILDJDO);
        }
        if self.NNOLOOIACLG != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.NNOLOOIACLG);
        }
        if self.MKNHJFFHCDA != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.MKNHJFFHCDA);
        }
        for value in &self.MHFANLAADFF {
            my_size += ::protobuf::rt::uint32_size(15, *value);
        };
        if self.FKMOJLILEDA != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.FKMOJLILEDA);
        }
        if self.DDCGDKHHPAK != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.DDCGDKHHPAK);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.APKEFKGPHIE != ::protobuf::EnumOrUnknown::new(super::MuseumRandomEventState::MuseumRandomEventState::MUSEUM_RANDOM_EVENT_STATE_NONE) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.APKEFKGPHIE))?;
        }
        if self.OMFGMILDJDO != 0 {
            os.write_uint32(14, self.OMFGMILDJDO)?;
        }
        if self.NNOLOOIACLG != 0 {
            os.write_uint32(2, self.NNOLOOIACLG)?;
        }
        if self.MKNHJFFHCDA != 0 {
            os.write_uint32(3, self.MKNHJFFHCDA)?;
        }
        for v in &self.MHFANLAADFF {
            os.write_uint32(15, *v)?;
        };
        if self.FKMOJLILEDA != 0 {
            os.write_uint32(8, self.FKMOJLILEDA)?;
        }
        if self.DDCGDKHHPAK != 0 {
            os.write_uint32(4, self.DDCGDKHHPAK)?;
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

    fn new() -> NNIDMJHAPKK {
        NNIDMJHAPKK::new()
    }

    fn clear(&mut self) {
        self.APKEFKGPHIE = ::protobuf::EnumOrUnknown::new(super::MuseumRandomEventState::MuseumRandomEventState::MUSEUM_RANDOM_EVENT_STATE_NONE);
        self.OMFGMILDJDO = 0;
        self.NNOLOOIACLG = 0;
        self.MKNHJFFHCDA = 0;
        self.MHFANLAADFF.clear();
        self.FKMOJLILEDA = 0;
        self.DDCGDKHHPAK = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NNIDMJHAPKK {
        static instance: NNIDMJHAPKK = NNIDMJHAPKK {
            APKEFKGPHIE: ::protobuf::EnumOrUnknown::from_i32(0),
            OMFGMILDJDO: 0,
            NNOLOOIACLG: 0,
            MKNHJFFHCDA: 0,
            MHFANLAADFF: ::std::vec::Vec::new(),
            FKMOJLILEDA: 0,
            DDCGDKHHPAK: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NNIDMJHAPKK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NNIDMJHAPKK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NNIDMJHAPKK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NNIDMJHAPKK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NNIDMJHAPKK.proto\x1a\x1cMuseumRandomEventState.proto\"\x94\x02\n\
    \x0bNNIDMJHAPKK\x129\n\x0bAPKEFKGPHIE\x18\r\x20\x01(\x0e2\x17.MuseumRand\
    omEventStateR\x0bAPKEFKGPHIE\x12\x20\n\x0bOMFGMILDJDO\x18\x0e\x20\x01(\r\
    R\x0bOMFGMILDJDO\x12\x20\n\x0bNNOLOOIACLG\x18\x02\x20\x01(\rR\x0bNNOLOOI\
    ACLG\x12\x20\n\x0bMKNHJFFHCDA\x18\x03\x20\x01(\rR\x0bMKNHJFFHCDA\x12\x20\
    \n\x0bMHFANLAADFF\x18\x0f\x20\x03(\rR\x0bMHFANLAADFF\x12\x20\n\x0bFKMOJL\
    ILEDA\x18\x08\x20\x01(\rR\x0bFKMOJLILEDA\x12\x20\n\x0bDDCGDKHHPAK\x18\
    \x04\x20\x01(\rR\x0bDDCGDKHHPAKb\x06proto3\
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
            deps.push(super::MuseumRandomEventState::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(NNIDMJHAPKK::generated_message_descriptor_data());
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
