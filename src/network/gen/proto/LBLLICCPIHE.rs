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

//! Generated file from `LBLLICCPIHE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LBLLICCPIHE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LBLLICCPIHE {
    // message fields
    // @@protoc_insertion_point(field:LBLLICCPIHE.IPNHCCODNDI)
    pub IPNHCCODNDI: u64,
    // @@protoc_insertion_point(field:LBLLICCPIHE.MKBMIPELMOB)
    pub MKBMIPELMOB: ::protobuf::EnumOrUnknown<super::FightGameMode::FightGameMode>,
    // @@protoc_insertion_point(field:LBLLICCPIHE.EJBJDGIMIPP)
    pub EJBJDGIMIPP: ::std::vec::Vec<super::DBBLOFLAAMH::DBBLOFLAAMH>,
    // @@protoc_insertion_point(field:LBLLICCPIHE.MPNJPFDCBDG)
    pub MPNJPFDCBDG: ::protobuf::EnumOrUnknown<super::KMPGLAILIDF::KMPGLAILIDF>,
    // @@protoc_insertion_point(field:LBLLICCPIHE.EEFBHBMOCPL)
    pub EEFBHBMOCPL: u64,
    // @@protoc_insertion_point(field:LBLLICCPIHE.FFICMJHMFJG)
    pub FFICMJHMFJG: u64,
    // @@protoc_insertion_point(field:LBLLICCPIHE.MONNADEJJGM)
    pub MONNADEJJGM: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LBLLICCPIHE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LBLLICCPIHE {
    fn default() -> &'a LBLLICCPIHE {
        <LBLLICCPIHE as ::protobuf::Message>::default_instance()
    }
}

impl LBLLICCPIHE {
    pub fn new() -> LBLLICCPIHE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IPNHCCODNDI",
            |m: &LBLLICCPIHE| { &m.IPNHCCODNDI },
            |m: &mut LBLLICCPIHE| { &mut m.IPNHCCODNDI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MKBMIPELMOB",
            |m: &LBLLICCPIHE| { &m.MKBMIPELMOB },
            |m: &mut LBLLICCPIHE| { &mut m.MKBMIPELMOB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EJBJDGIMIPP",
            |m: &LBLLICCPIHE| { &m.EJBJDGIMIPP },
            |m: &mut LBLLICCPIHE| { &mut m.EJBJDGIMIPP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MPNJPFDCBDG",
            |m: &LBLLICCPIHE| { &m.MPNJPFDCBDG },
            |m: &mut LBLLICCPIHE| { &mut m.MPNJPFDCBDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EEFBHBMOCPL",
            |m: &LBLLICCPIHE| { &m.EEFBHBMOCPL },
            |m: &mut LBLLICCPIHE| { &mut m.EEFBHBMOCPL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FFICMJHMFJG",
            |m: &LBLLICCPIHE| { &m.FFICMJHMFJG },
            |m: &mut LBLLICCPIHE| { &mut m.FFICMJHMFJG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MONNADEJJGM",
            |m: &LBLLICCPIHE| { &m.MONNADEJJGM },
            |m: &mut LBLLICCPIHE| { &mut m.MONNADEJJGM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LBLLICCPIHE>(
            "LBLLICCPIHE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LBLLICCPIHE {
    const NAME: &'static str = "LBLLICCPIHE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.IPNHCCODNDI = is.read_uint64()?;
                },
                16 => {
                    self.MKBMIPELMOB = is.read_enum_or_unknown()?;
                },
                26 => {
                    self.EJBJDGIMIPP.push(is.read_message()?);
                },
                32 => {
                    self.MPNJPFDCBDG = is.read_enum_or_unknown()?;
                },
                40 => {
                    self.EEFBHBMOCPL = is.read_uint64()?;
                },
                48 => {
                    self.FFICMJHMFJG = is.read_uint64()?;
                },
                56 => {
                    self.MONNADEJJGM = is.read_uint32()?;
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
        if self.IPNHCCODNDI != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.IPNHCCODNDI);
        }
        if self.MKBMIPELMOB != ::protobuf::EnumOrUnknown::new(super::FightGameMode::FightGameMode::FIGHT_GAME_MODE_NONE) {
            my_size += ::protobuf::rt::int32_size(2, self.MKBMIPELMOB.value());
        }
        for value in &self.EJBJDGIMIPP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.MPNJPFDCBDG != ::protobuf::EnumOrUnknown::new(super::KMPGLAILIDF::KMPGLAILIDF::MATCH_UNIT_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(4, self.MPNJPFDCBDG.value());
        }
        if self.EEFBHBMOCPL != 0 {
            my_size += ::protobuf::rt::uint64_size(5, self.EEFBHBMOCPL);
        }
        if self.FFICMJHMFJG != 0 {
            my_size += ::protobuf::rt::uint64_size(6, self.FFICMJHMFJG);
        }
        if self.MONNADEJJGM != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.MONNADEJJGM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IPNHCCODNDI != 0 {
            os.write_uint64(1, self.IPNHCCODNDI)?;
        }
        if self.MKBMIPELMOB != ::protobuf::EnumOrUnknown::new(super::FightGameMode::FightGameMode::FIGHT_GAME_MODE_NONE) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.MKBMIPELMOB))?;
        }
        for v in &self.EJBJDGIMIPP {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.MPNJPFDCBDG != ::protobuf::EnumOrUnknown::new(super::KMPGLAILIDF::KMPGLAILIDF::MATCH_UNIT_TYPE_NONE) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.MPNJPFDCBDG))?;
        }
        if self.EEFBHBMOCPL != 0 {
            os.write_uint64(5, self.EEFBHBMOCPL)?;
        }
        if self.FFICMJHMFJG != 0 {
            os.write_uint64(6, self.FFICMJHMFJG)?;
        }
        if self.MONNADEJJGM != 0 {
            os.write_uint32(7, self.MONNADEJJGM)?;
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

    fn new() -> LBLLICCPIHE {
        LBLLICCPIHE::new()
    }

    fn clear(&mut self) {
        self.IPNHCCODNDI = 0;
        self.MKBMIPELMOB = ::protobuf::EnumOrUnknown::new(super::FightGameMode::FightGameMode::FIGHT_GAME_MODE_NONE);
        self.EJBJDGIMIPP.clear();
        self.MPNJPFDCBDG = ::protobuf::EnumOrUnknown::new(super::KMPGLAILIDF::KMPGLAILIDF::MATCH_UNIT_TYPE_NONE);
        self.EEFBHBMOCPL = 0;
        self.FFICMJHMFJG = 0;
        self.MONNADEJJGM = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LBLLICCPIHE {
        static instance: LBLLICCPIHE = LBLLICCPIHE {
            IPNHCCODNDI: 0,
            MKBMIPELMOB: ::protobuf::EnumOrUnknown::from_i32(0),
            EJBJDGIMIPP: ::std::vec::Vec::new(),
            MPNJPFDCBDG: ::protobuf::EnumOrUnknown::from_i32(0),
            EEFBHBMOCPL: 0,
            FFICMJHMFJG: 0,
            MONNADEJJGM: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LBLLICCPIHE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LBLLICCPIHE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LBLLICCPIHE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LBLLICCPIHE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LBLLICCPIHE.proto\x1a\x11DBBLOFLAAMH.proto\x1a\x13FightGameMode.pr\
    oto\x1a\x11KMPGLAILIDF.proto\"\xa7\x02\n\x0bLBLLICCPIHE\x12\x20\n\x0bIPN\
    HCCODNDI\x18\x01\x20\x01(\x04R\x0bIPNHCCODNDI\x120\n\x0bMKBMIPELMOB\x18\
    \x02\x20\x01(\x0e2\x0e.FightGameModeR\x0bMKBMIPELMOB\x12.\n\x0bEJBJDGIMI\
    PP\x18\x03\x20\x03(\x0b2\x0c.DBBLOFLAAMHR\x0bEJBJDGIMIPP\x12.\n\x0bMPNJP\
    FDCBDG\x18\x04\x20\x01(\x0e2\x0c.KMPGLAILIDFR\x0bMPNJPFDCBDG\x12\x20\n\
    \x0bEEFBHBMOCPL\x18\x05\x20\x01(\x04R\x0bEEFBHBMOCPL\x12\x20\n\x0bFFICMJ\
    HMFJG\x18\x06\x20\x01(\x04R\x0bFFICMJHMFJG\x12\x20\n\x0bMONNADEJJGM\x18\
    \x07\x20\x01(\rR\x0bMONNADEJJGMb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::DBBLOFLAAMH::file_descriptor().clone());
            deps.push(super::FightGameMode::file_descriptor().clone());
            deps.push(super::KMPGLAILIDF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LBLLICCPIHE::generated_message_descriptor_data());
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