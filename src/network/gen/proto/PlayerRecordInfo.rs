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

//! Generated file from `PlayerRecordInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlayerRecordInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerRecordInfo {
    // message fields
    // @@protoc_insertion_point(field:PlayerRecordInfo.OLDDNDGIDBE)
    pub OLDDNDGIDBE: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.DAAIBGHALGJ)
    pub DAAIBGHALGJ: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.FCPGOMHFBCL)
    pub FCPGOMHFBCL: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.ADEBLLPLFHA)
    pub ADEBLLPLFHA: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.AAJHEOFGPEH)
    pub AAJHEOFGPEH: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.MMJAGLMAKMO)
    pub MMJAGLMAKMO: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.JILEFGGEABH)
    pub JILEFGGEABH: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.KFKPJPJKJDE)
    pub KFKPJPJKJDE: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.collection_info)
    pub collection_info: ::protobuf::MessageField<super::PlayerCollectionInfo::PlayerCollectionInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerRecordInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerRecordInfo {
    fn default() -> &'a PlayerRecordInfo {
        <PlayerRecordInfo as ::protobuf::Message>::default_instance()
    }
}

impl PlayerRecordInfo {
    pub fn new() -> PlayerRecordInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OLDDNDGIDBE",
            |m: &PlayerRecordInfo| { &m.OLDDNDGIDBE },
            |m: &mut PlayerRecordInfo| { &mut m.OLDDNDGIDBE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DAAIBGHALGJ",
            |m: &PlayerRecordInfo| { &m.DAAIBGHALGJ },
            |m: &mut PlayerRecordInfo| { &mut m.DAAIBGHALGJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FCPGOMHFBCL",
            |m: &PlayerRecordInfo| { &m.FCPGOMHFBCL },
            |m: &mut PlayerRecordInfo| { &mut m.FCPGOMHFBCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADEBLLPLFHA",
            |m: &PlayerRecordInfo| { &m.ADEBLLPLFHA },
            |m: &mut PlayerRecordInfo| { &mut m.ADEBLLPLFHA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AAJHEOFGPEH",
            |m: &PlayerRecordInfo| { &m.AAJHEOFGPEH },
            |m: &mut PlayerRecordInfo| { &mut m.AAJHEOFGPEH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MMJAGLMAKMO",
            |m: &PlayerRecordInfo| { &m.MMJAGLMAKMO },
            |m: &mut PlayerRecordInfo| { &mut m.MMJAGLMAKMO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JILEFGGEABH",
            |m: &PlayerRecordInfo| { &m.JILEFGGEABH },
            |m: &mut PlayerRecordInfo| { &mut m.JILEFGGEABH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KFKPJPJKJDE",
            |m: &PlayerRecordInfo| { &m.KFKPJPJKJDE },
            |m: &mut PlayerRecordInfo| { &mut m.KFKPJPJKJDE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PlayerCollectionInfo::PlayerCollectionInfo>(
            "collection_info",
            |m: &PlayerRecordInfo| { &m.collection_info },
            |m: &mut PlayerRecordInfo| { &mut m.collection_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerRecordInfo>(
            "PlayerRecordInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerRecordInfo {
    const NAME: &'static str = "PlayerRecordInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.OLDDNDGIDBE = is.read_uint32()?;
                },
                8 => {
                    self.DAAIBGHALGJ = is.read_uint32()?;
                },
                48 => {
                    self.FCPGOMHFBCL = is.read_uint32()?;
                },
                16 => {
                    self.ADEBLLPLFHA = is.read_uint32()?;
                },
                56 => {
                    self.AAJHEOFGPEH = is.read_uint32()?;
                },
                96 => {
                    self.MMJAGLMAKMO = is.read_uint32()?;
                },
                32 => {
                    self.JILEFGGEABH = is.read_uint32()?;
                },
                120 => {
                    self.KFKPJPJKJDE = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.collection_info)?;
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
        if self.OLDDNDGIDBE != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.OLDDNDGIDBE);
        }
        if self.DAAIBGHALGJ != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.DAAIBGHALGJ);
        }
        if self.FCPGOMHFBCL != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.FCPGOMHFBCL);
        }
        if self.ADEBLLPLFHA != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.ADEBLLPLFHA);
        }
        if self.AAJHEOFGPEH != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.AAJHEOFGPEH);
        }
        if self.MMJAGLMAKMO != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.MMJAGLMAKMO);
        }
        if self.JILEFGGEABH != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.JILEFGGEABH);
        }
        if self.KFKPJPJKJDE != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.KFKPJPJKJDE);
        }
        if let Some(v) = self.collection_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OLDDNDGIDBE != 0 {
            os.write_uint32(3, self.OLDDNDGIDBE)?;
        }
        if self.DAAIBGHALGJ != 0 {
            os.write_uint32(1, self.DAAIBGHALGJ)?;
        }
        if self.FCPGOMHFBCL != 0 {
            os.write_uint32(6, self.FCPGOMHFBCL)?;
        }
        if self.ADEBLLPLFHA != 0 {
            os.write_uint32(2, self.ADEBLLPLFHA)?;
        }
        if self.AAJHEOFGPEH != 0 {
            os.write_uint32(7, self.AAJHEOFGPEH)?;
        }
        if self.MMJAGLMAKMO != 0 {
            os.write_uint32(12, self.MMJAGLMAKMO)?;
        }
        if self.JILEFGGEABH != 0 {
            os.write_uint32(4, self.JILEFGGEABH)?;
        }
        if self.KFKPJPJKJDE != 0 {
            os.write_uint32(15, self.KFKPJPJKJDE)?;
        }
        if let Some(v) = self.collection_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
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

    fn new() -> PlayerRecordInfo {
        PlayerRecordInfo::new()
    }

    fn clear(&mut self) {
        self.OLDDNDGIDBE = 0;
        self.DAAIBGHALGJ = 0;
        self.FCPGOMHFBCL = 0;
        self.ADEBLLPLFHA = 0;
        self.AAJHEOFGPEH = 0;
        self.MMJAGLMAKMO = 0;
        self.JILEFGGEABH = 0;
        self.KFKPJPJKJDE = 0;
        self.collection_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerRecordInfo {
        static instance: PlayerRecordInfo = PlayerRecordInfo {
            OLDDNDGIDBE: 0,
            DAAIBGHALGJ: 0,
            FCPGOMHFBCL: 0,
            ADEBLLPLFHA: 0,
            AAJHEOFGPEH: 0,
            MMJAGLMAKMO: 0,
            JILEFGGEABH: 0,
            KFKPJPJKJDE: 0,
            collection_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerRecordInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerRecordInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerRecordInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerRecordInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16PlayerRecordInfo.proto\x1a\x1aPlayerCollectionInfo.proto\"\xe2\x02\
    \n\x10PlayerRecordInfo\x12\x20\n\x0bOLDDNDGIDBE\x18\x03\x20\x01(\rR\x0bO\
    LDDNDGIDBE\x12\x20\n\x0bDAAIBGHALGJ\x18\x01\x20\x01(\rR\x0bDAAIBGHALGJ\
    \x12\x20\n\x0bFCPGOMHFBCL\x18\x06\x20\x01(\rR\x0bFCPGOMHFBCL\x12\x20\n\
    \x0bADEBLLPLFHA\x18\x02\x20\x01(\rR\x0bADEBLLPLFHA\x12\x20\n\x0bAAJHEOFG\
    PEH\x18\x07\x20\x01(\rR\x0bAAJHEOFGPEH\x12\x20\n\x0bMMJAGLMAKMO\x18\x0c\
    \x20\x01(\rR\x0bMMJAGLMAKMO\x12\x20\n\x0bJILEFGGEABH\x18\x04\x20\x01(\rR\
    \x0bJILEFGGEABH\x12\x20\n\x0bKFKPJPJKJDE\x18\x0f\x20\x01(\rR\x0bKFKPJPJK\
    JDE\x12>\n\x0fcollection_info\x18\t\x20\x01(\x0b2\x15.PlayerCollectionIn\
    foR\x0ecollectionInfoB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::PlayerCollectionInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlayerRecordInfo::generated_message_descriptor_data());
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
