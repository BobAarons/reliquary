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

//! Generated file from `LLKOHFCICJB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LLKOHFCICJB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LLKOHFCICJB {
    // message fields
    // @@protoc_insertion_point(field:LLKOHFCICJB.EMALNMLGANJ)
    pub EMALNMLGANJ: ::std::vec::Vec<super::KAOOHLAKBPN::KAOOHLAKBPN>,
    // @@protoc_insertion_point(field:LLKOHFCICJB.KHOPBJOOJNC)
    pub KHOPBJOOJNC: ::std::vec::Vec<super::CDDBLMIAIKF::CDDBLMIAIKF>,
    // @@protoc_insertion_point(field:LLKOHFCICJB.EBDDNGHLIGH)
    pub EBDDNGHLIGH: ::std::vec::Vec<super::KIGHHJDHGOA::KIGHHJDHGOA>,
    // @@protoc_insertion_point(field:LLKOHFCICJB.DNMJBNNJLEL)
    pub DNMJBNNJLEL: u32,
    // @@protoc_insertion_point(field:LLKOHFCICJB.PGKJOGNBOPO)
    pub PGKJOGNBOPO: ::std::collections::HashMap<u32, super::JGOIHDOBEEK::JGOIHDOBEEK>,
    // @@protoc_insertion_point(field:LLKOHFCICJB.LNEFJMMOMKB)
    pub LNEFJMMOMKB: ::protobuf::MessageField<super::HMCNJLLHBLI::HMCNJLLHBLI>,
    // @@protoc_insertion_point(field:LLKOHFCICJB.JCFNBCABDNB)
    pub JCFNBCABDNB: ::std::vec::Vec<super::KAOOHLAKBPN::KAOOHLAKBPN>,
    // @@protoc_insertion_point(field:LLKOHFCICJB.KAHOEKAEFHD)
    pub KAHOEKAEFHD: ::protobuf::MessageField<super::EvolveBuild::EvolveBuildBattleInfo>,
    // @@protoc_insertion_point(field:LLKOHFCICJB.DHNDKIFPOLF)
    pub DHNDKIFPOLF: ::protobuf::MessageField<super::KEJJGGCCGAN::KEJJGGCCGAN>,
    // @@protoc_insertion_point(field:LLKOHFCICJB.EPECBKPMBHE)
    pub EPECBKPMBHE: ::protobuf::MessageField<super::BIMKJOHKBNO::BIMKJOHKBNO>,
    // special fields
    // @@protoc_insertion_point(special_field:LLKOHFCICJB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LLKOHFCICJB {
    fn default() -> &'a LLKOHFCICJB {
        <LLKOHFCICJB as ::protobuf::Message>::default_instance()
    }
}

impl LLKOHFCICJB {
    pub fn new() -> LLKOHFCICJB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EMALNMLGANJ",
            |m: &LLKOHFCICJB| { &m.EMALNMLGANJ },
            |m: &mut LLKOHFCICJB| { &mut m.EMALNMLGANJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KHOPBJOOJNC",
            |m: &LLKOHFCICJB| { &m.KHOPBJOOJNC },
            |m: &mut LLKOHFCICJB| { &mut m.KHOPBJOOJNC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EBDDNGHLIGH",
            |m: &LLKOHFCICJB| { &m.EBDDNGHLIGH },
            |m: &mut LLKOHFCICJB| { &mut m.EBDDNGHLIGH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNMJBNNJLEL",
            |m: &LLKOHFCICJB| { &m.DNMJBNNJLEL },
            |m: &mut LLKOHFCICJB| { &mut m.DNMJBNNJLEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "PGKJOGNBOPO",
            |m: &LLKOHFCICJB| { &m.PGKJOGNBOPO },
            |m: &mut LLKOHFCICJB| { &mut m.PGKJOGNBOPO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HMCNJLLHBLI::HMCNJLLHBLI>(
            "LNEFJMMOMKB",
            |m: &LLKOHFCICJB| { &m.LNEFJMMOMKB },
            |m: &mut LLKOHFCICJB| { &mut m.LNEFJMMOMKB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JCFNBCABDNB",
            |m: &LLKOHFCICJB| { &m.JCFNBCABDNB },
            |m: &mut LLKOHFCICJB| { &mut m.JCFNBCABDNB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EvolveBuild::EvolveBuildBattleInfo>(
            "KAHOEKAEFHD",
            |m: &LLKOHFCICJB| { &m.KAHOEKAEFHD },
            |m: &mut LLKOHFCICJB| { &mut m.KAHOEKAEFHD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KEJJGGCCGAN::KEJJGGCCGAN>(
            "DHNDKIFPOLF",
            |m: &LLKOHFCICJB| { &m.DHNDKIFPOLF },
            |m: &mut LLKOHFCICJB| { &mut m.DHNDKIFPOLF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BIMKJOHKBNO::BIMKJOHKBNO>(
            "EPECBKPMBHE",
            |m: &LLKOHFCICJB| { &m.EPECBKPMBHE },
            |m: &mut LLKOHFCICJB| { &mut m.EPECBKPMBHE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LLKOHFCICJB>(
            "LLKOHFCICJB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LLKOHFCICJB {
    const NAME: &'static str = "LLKOHFCICJB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.EMALNMLGANJ.push(is.read_message()?);
                },
                18 => {
                    self.KHOPBJOOJNC.push(is.read_message()?);
                },
                26 => {
                    self.EBDDNGHLIGH.push(is.read_message()?);
                },
                56 => {
                    self.DNMJBNNJLEL = is.read_uint32()?;
                },
                74 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.PGKJOGNBOPO.insert(key, value);
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LNEFJMMOMKB)?;
                },
                90 => {
                    self.JCFNBCABDNB.push(is.read_message()?);
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KAHOEKAEFHD)?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DHNDKIFPOLF)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EPECBKPMBHE)?;
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
        for value in &self.EMALNMLGANJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.KHOPBJOOJNC {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.EBDDNGHLIGH {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.DNMJBNNJLEL != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.DNMJBNNJLEL);
        }
        for (k, v) in &self.PGKJOGNBOPO {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if let Some(v) = self.LNEFJMMOMKB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.JCFNBCABDNB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.KAHOEKAEFHD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.DHNDKIFPOLF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EPECBKPMBHE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.EMALNMLGANJ {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        for v in &self.KHOPBJOOJNC {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        for v in &self.EBDDNGHLIGH {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.DNMJBNNJLEL != 0 {
            os.write_uint32(7, self.DNMJBNNJLEL)?;
        }
        for (k, v) in &self.PGKJOGNBOPO {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(74)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if let Some(v) = self.LNEFJMMOMKB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        for v in &self.JCFNBCABDNB {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if let Some(v) = self.KAHOEKAEFHD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if let Some(v) = self.DHNDKIFPOLF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.EPECBKPMBHE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
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

    fn new() -> LLKOHFCICJB {
        LLKOHFCICJB::new()
    }

    fn clear(&mut self) {
        self.EMALNMLGANJ.clear();
        self.KHOPBJOOJNC.clear();
        self.EBDDNGHLIGH.clear();
        self.DNMJBNNJLEL = 0;
        self.PGKJOGNBOPO.clear();
        self.LNEFJMMOMKB.clear();
        self.JCFNBCABDNB.clear();
        self.KAHOEKAEFHD.clear();
        self.DHNDKIFPOLF.clear();
        self.EPECBKPMBHE.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LLKOHFCICJB {
        static instance: ::protobuf::rt::Lazy<LLKOHFCICJB> = ::protobuf::rt::Lazy::new();
        instance.get(LLKOHFCICJB::new)
    }
}

impl ::protobuf::MessageFull for LLKOHFCICJB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LLKOHFCICJB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LLKOHFCICJB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LLKOHFCICJB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LLKOHFCICJB.proto\x1a\x11BIMKJOHKBNO.proto\x1a\x11CDDBLMIAIKF.prot\
    o\x1a\x1bEvolveBuildBattleInfo.proto\x1a\x11HMCNJLLHBLI.proto\x1a\x11JGO\
    IHDOBEEK.proto\x1a\x11KAOOHLAKBPN.proto\x1a\x11KEJJGGCCGAN.proto\x1a\x11\
    KIGHHJDHGOA.proto\"\xc8\x04\n\x0bLLKOHFCICJB\x12.\n\x0bEMALNMLGANJ\x18\
    \x01\x20\x03(\x0b2\x0c.KAOOHLAKBPNR\x0bEMALNMLGANJ\x12.\n\x0bKHOPBJOOJNC\
    \x18\x02\x20\x03(\x0b2\x0c.CDDBLMIAIKFR\x0bKHOPBJOOJNC\x12.\n\x0bEBDDNGH\
    LIGH\x18\x03\x20\x03(\x0b2\x0c.KIGHHJDHGOAR\x0bEBDDNGHLIGH\x12\x20\n\x0b\
    DNMJBNNJLEL\x18\x07\x20\x01(\rR\x0bDNMJBNNJLEL\x12?\n\x0bPGKJOGNBOPO\x18\
    \t\x20\x03(\x0b2\x1d.LLKOHFCICJB.PGKJOGNBOPOEntryR\x0bPGKJOGNBOPO\x12.\n\
    \x0bLNEFJMMOMKB\x18\n\x20\x01(\x0b2\x0c.HMCNJLLHBLIR\x0bLNEFJMMOMKB\x12.\
    \n\x0bJCFNBCABDNB\x18\x0b\x20\x03(\x0b2\x0c.KAOOHLAKBPNR\x0bJCFNBCABDNB\
    \x128\n\x0bKAHOEKAEFHD\x18\x0c\x20\x01(\x0b2\x16.EvolveBuildBattleInfoR\
    \x0bKAHOEKAEFHD\x12.\n\x0bDHNDKIFPOLF\x18\r\x20\x01(\x0b2\x0c.KEJJGGCCGA\
    NR\x0bDHNDKIFPOLF\x12.\n\x0bEPECBKPMBHE\x18\x0e\x20\x01(\x0b2\x0c.BIMKJO\
    HKBNOR\x0bEPECBKPMBHE\x1aL\n\x10PGKJOGNBOPOEntry\x12\x10\n\x03key\x18\
    \x01\x20\x01(\rR\x03key\x12\"\n\x05value\x18\x02\x20\x01(\x0b2\x0c.JGOIH\
    DOBEEKR\x05value:\x028\x01b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(8);
            deps.push(super::BIMKJOHKBNO::file_descriptor().clone());
            deps.push(super::CDDBLMIAIKF::file_descriptor().clone());
            deps.push(super::EvolveBuildBattleInfo::file_descriptor().clone());
            deps.push(super::HMCNJLLHBLI::file_descriptor().clone());
            deps.push(super::JGOIHDOBEEK::file_descriptor().clone());
            deps.push(super::KAOOHLAKBPN::file_descriptor().clone());
            deps.push(super::KEJJGGCCGAN::file_descriptor().clone());
            deps.push(super::KIGHHJDHGOA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LLKOHFCICJB::generated_message_descriptor_data());
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
