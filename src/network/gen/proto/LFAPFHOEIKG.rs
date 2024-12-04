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

//! Generated file from `LFAPFHOEIKG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LFAPFHOEIKG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LFAPFHOEIKG {
    // message fields
    // @@protoc_insertion_point(field:LFAPFHOEIKG.HGMFEGGJEFB)
    pub HGMFEGGJEFB: u32,
    // @@protoc_insertion_point(field:LFAPFHOEIKG.BPFBJKCFLKH)
    pub BPFBJKCFLKH: u32,
    // @@protoc_insertion_point(field:LFAPFHOEIKG.HHBNIFIJLKJ)
    pub HHBNIFIJLKJ: u32,
    // @@protoc_insertion_point(field:LFAPFHOEIKG.OJBAILGKLBM)
    pub OJBAILGKLBM: ::protobuf::EnumOrUnknown<super::DFLNDOIODNG::DFLNDOIODNG>,
    // special fields
    // @@protoc_insertion_point(special_field:LFAPFHOEIKG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LFAPFHOEIKG {
    fn default() -> &'a LFAPFHOEIKG {
        <LFAPFHOEIKG as ::protobuf::Message>::default_instance()
    }
}

impl LFAPFHOEIKG {
    pub fn new() -> LFAPFHOEIKG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HGMFEGGJEFB",
            |m: &LFAPFHOEIKG| { &m.HGMFEGGJEFB },
            |m: &mut LFAPFHOEIKG| { &mut m.HGMFEGGJEFB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BPFBJKCFLKH",
            |m: &LFAPFHOEIKG| { &m.BPFBJKCFLKH },
            |m: &mut LFAPFHOEIKG| { &mut m.BPFBJKCFLKH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HHBNIFIJLKJ",
            |m: &LFAPFHOEIKG| { &m.HHBNIFIJLKJ },
            |m: &mut LFAPFHOEIKG| { &mut m.HHBNIFIJLKJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OJBAILGKLBM",
            |m: &LFAPFHOEIKG| { &m.OJBAILGKLBM },
            |m: &mut LFAPFHOEIKG| { &mut m.OJBAILGKLBM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LFAPFHOEIKG>(
            "LFAPFHOEIKG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LFAPFHOEIKG {
    const NAME: &'static str = "LFAPFHOEIKG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.HGMFEGGJEFB = is.read_uint32()?;
                },
                32 => {
                    self.BPFBJKCFLKH = is.read_uint32()?;
                },
                16 => {
                    self.HHBNIFIJLKJ = is.read_uint32()?;
                },
                8 => {
                    self.OJBAILGKLBM = is.read_enum_or_unknown()?;
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
        if self.HGMFEGGJEFB != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.HGMFEGGJEFB);
        }
        if self.BPFBJKCFLKH != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.BPFBJKCFLKH);
        }
        if self.HHBNIFIJLKJ != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.HHBNIFIJLKJ);
        }
        if self.OJBAILGKLBM != ::protobuf::EnumOrUnknown::new(super::DFLNDOIODNG::DFLNDOIODNG::ROGUE_TOURN_ROOM_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(1, self.OJBAILGKLBM.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HGMFEGGJEFB != 0 {
            os.write_uint32(9, self.HGMFEGGJEFB)?;
        }
        if self.BPFBJKCFLKH != 0 {
            os.write_uint32(4, self.BPFBJKCFLKH)?;
        }
        if self.HHBNIFIJLKJ != 0 {
            os.write_uint32(2, self.HHBNIFIJLKJ)?;
        }
        if self.OJBAILGKLBM != ::protobuf::EnumOrUnknown::new(super::DFLNDOIODNG::DFLNDOIODNG::ROGUE_TOURN_ROOM_STATUS_NONE) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.OJBAILGKLBM))?;
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

    fn new() -> LFAPFHOEIKG {
        LFAPFHOEIKG::new()
    }

    fn clear(&mut self) {
        self.HGMFEGGJEFB = 0;
        self.BPFBJKCFLKH = 0;
        self.HHBNIFIJLKJ = 0;
        self.OJBAILGKLBM = ::protobuf::EnumOrUnknown::new(super::DFLNDOIODNG::DFLNDOIODNG::ROGUE_TOURN_ROOM_STATUS_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LFAPFHOEIKG {
        static instance: LFAPFHOEIKG = LFAPFHOEIKG {
            HGMFEGGJEFB: 0,
            BPFBJKCFLKH: 0,
            HHBNIFIJLKJ: 0,
            OJBAILGKLBM: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LFAPFHOEIKG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LFAPFHOEIKG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LFAPFHOEIKG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LFAPFHOEIKG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LFAPFHOEIKG.proto\x1a\x11DFLNDOIODNG.proto\"\xa3\x01\n\x0bLFAPFHOE\
    IKG\x12\x20\n\x0bHGMFEGGJEFB\x18\t\x20\x01(\rR\x0bHGMFEGGJEFB\x12\x20\n\
    \x0bBPFBJKCFLKH\x18\x04\x20\x01(\rR\x0bBPFBJKCFLKH\x12\x20\n\x0bHHBNIFIJ\
    LKJ\x18\x02\x20\x01(\rR\x0bHHBNIFIJLKJ\x12.\n\x0bOJBAILGKLBM\x18\x01\x20\
    \x01(\x0e2\x0c.DFLNDOIODNGR\x0bOJBAILGKLBMb\x06proto3\
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
            deps.push(super::DFLNDOIODNG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LFAPFHOEIKG::generated_message_descriptor_data());
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