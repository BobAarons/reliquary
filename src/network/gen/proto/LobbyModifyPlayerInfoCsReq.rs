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

//! Generated file from `LobbyModifyPlayerInfoCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LobbyModifyPlayerInfoCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LobbyModifyPlayerInfoCsReq {
    // message fields
    // @@protoc_insertion_point(field:LobbyModifyPlayerInfoCsReq.MPNJPFDCBDG)
    pub MPNJPFDCBDG: ::protobuf::EnumOrUnknown<super::LobbyModifyType::LobbyModifyType>,
    // @@protoc_insertion_point(field:LobbyModifyPlayerInfoCsReq.PDLAHDEBOIL)
    pub PDLAHDEBOIL: ::protobuf::MessageField<super::JCEDNEMPMAJ::JCEDNEMPMAJ>,
    // @@protoc_insertion_point(field:LobbyModifyPlayerInfoCsReq.NLNGJCJLDFO)
    pub NLNGJCJLDFO: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LobbyModifyPlayerInfoCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LobbyModifyPlayerInfoCsReq {
    fn default() -> &'a LobbyModifyPlayerInfoCsReq {
        <LobbyModifyPlayerInfoCsReq as ::protobuf::Message>::default_instance()
    }
}

impl LobbyModifyPlayerInfoCsReq {
    pub fn new() -> LobbyModifyPlayerInfoCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MPNJPFDCBDG",
            |m: &LobbyModifyPlayerInfoCsReq| { &m.MPNJPFDCBDG },
            |m: &mut LobbyModifyPlayerInfoCsReq| { &mut m.MPNJPFDCBDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JCEDNEMPMAJ::JCEDNEMPMAJ>(
            "PDLAHDEBOIL",
            |m: &LobbyModifyPlayerInfoCsReq| { &m.PDLAHDEBOIL },
            |m: &mut LobbyModifyPlayerInfoCsReq| { &mut m.PDLAHDEBOIL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NLNGJCJLDFO",
            |m: &LobbyModifyPlayerInfoCsReq| { &m.NLNGJCJLDFO },
            |m: &mut LobbyModifyPlayerInfoCsReq| { &mut m.NLNGJCJLDFO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LobbyModifyPlayerInfoCsReq>(
            "LobbyModifyPlayerInfoCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LobbyModifyPlayerInfoCsReq {
    const NAME: &'static str = "LobbyModifyPlayerInfoCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.MPNJPFDCBDG = is.read_enum_or_unknown()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PDLAHDEBOIL)?;
                },
                104 => {
                    self.NLNGJCJLDFO = is.read_uint32()?;
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
        if self.MPNJPFDCBDG != ::protobuf::EnumOrUnknown::new(super::LobbyModifyType::LobbyModifyType::LobbyModifyType_None) {
            my_size += ::protobuf::rt::int32_size(1, self.MPNJPFDCBDG.value());
        }
        if let Some(v) = self.PDLAHDEBOIL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.NLNGJCJLDFO != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.NLNGJCJLDFO);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.MPNJPFDCBDG != ::protobuf::EnumOrUnknown::new(super::LobbyModifyType::LobbyModifyType::LobbyModifyType_None) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.MPNJPFDCBDG))?;
        }
        if let Some(v) = self.PDLAHDEBOIL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.NLNGJCJLDFO != 0 {
            os.write_uint32(13, self.NLNGJCJLDFO)?;
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

    fn new() -> LobbyModifyPlayerInfoCsReq {
        LobbyModifyPlayerInfoCsReq::new()
    }

    fn clear(&mut self) {
        self.MPNJPFDCBDG = ::protobuf::EnumOrUnknown::new(super::LobbyModifyType::LobbyModifyType::LobbyModifyType_None);
        self.PDLAHDEBOIL.clear();
        self.NLNGJCJLDFO = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LobbyModifyPlayerInfoCsReq {
        static instance: LobbyModifyPlayerInfoCsReq = LobbyModifyPlayerInfoCsReq {
            MPNJPFDCBDG: ::protobuf::EnumOrUnknown::from_i32(0),
            PDLAHDEBOIL: ::protobuf::MessageField::none(),
            NLNGJCJLDFO: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LobbyModifyPlayerInfoCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LobbyModifyPlayerInfoCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LobbyModifyPlayerInfoCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LobbyModifyPlayerInfoCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20LobbyModifyPlayerInfoCsReq.proto\x1a\x11JCEDNEMPMAJ.proto\x1a\x15L\
    obbyModifyType.proto\"\xa2\x01\n\x1aLobbyModifyPlayerInfoCsReq\x122\n\
    \x0bMPNJPFDCBDG\x18\x01\x20\x01(\x0e2\x10.LobbyModifyTypeR\x0bMPNJPFDCBD\
    G\x12.\n\x0bPDLAHDEBOIL\x18\x05\x20\x01(\x0b2\x0c.JCEDNEMPMAJR\x0bPDLAHD\
    EBOIL\x12\x20\n\x0bNLNGJCJLDFO\x18\r\x20\x01(\rR\x0bNLNGJCJLDFOb\x06prot\
    o3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::JCEDNEMPMAJ::file_descriptor().clone());
            deps.push(super::LobbyModifyType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LobbyModifyPlayerInfoCsReq::generated_message_descriptor_data());
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