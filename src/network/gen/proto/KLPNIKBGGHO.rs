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

//! Generated file from `KLPNIKBGGHO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KLPNIKBGGHO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KLPNIKBGGHO {
    // message fields
    // @@protoc_insertion_point(field:KLPNIKBGGHO.CLNGIMNBHPG)
    pub CLNGIMNBHPG: u32,
    // @@protoc_insertion_point(field:KLPNIKBGGHO.NANJOFKIELP)
    pub NANJOFKIELP: ::std::vec::Vec<super::MABPAECJNNF::MABPAECJNNF>,
    // @@protoc_insertion_point(field:KLPNIKBGGHO.DCBDGNCODGD)
    pub DCBDGNCODGD: ::std::vec::Vec<super::MABPAECJNNF::MABPAECJNNF>,
    // @@protoc_insertion_point(field:KLPNIKBGGHO.IKKDELCFKGM)
    pub IKKDELCFKGM: ::protobuf::MessageField<super::MABPAECJNNF::MABPAECJNNF>,
    // @@protoc_insertion_point(field:KLPNIKBGGHO.AIHABKFMIGC)
    pub AIHABKFMIGC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:KLPNIKBGGHO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KLPNIKBGGHO {
    fn default() -> &'a KLPNIKBGGHO {
        <KLPNIKBGGHO as ::protobuf::Message>::default_instance()
    }
}

impl KLPNIKBGGHO {
    pub fn new() -> KLPNIKBGGHO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CLNGIMNBHPG",
            |m: &KLPNIKBGGHO| { &m.CLNGIMNBHPG },
            |m: &mut KLPNIKBGGHO| { &mut m.CLNGIMNBHPG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NANJOFKIELP",
            |m: &KLPNIKBGGHO| { &m.NANJOFKIELP },
            |m: &mut KLPNIKBGGHO| { &mut m.NANJOFKIELP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DCBDGNCODGD",
            |m: &KLPNIKBGGHO| { &m.DCBDGNCODGD },
            |m: &mut KLPNIKBGGHO| { &mut m.DCBDGNCODGD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MABPAECJNNF::MABPAECJNNF>(
            "IKKDELCFKGM",
            |m: &KLPNIKBGGHO| { &m.IKKDELCFKGM },
            |m: &mut KLPNIKBGGHO| { &mut m.IKKDELCFKGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AIHABKFMIGC",
            |m: &KLPNIKBGGHO| { &m.AIHABKFMIGC },
            |m: &mut KLPNIKBGGHO| { &mut m.AIHABKFMIGC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KLPNIKBGGHO>(
            "KLPNIKBGGHO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KLPNIKBGGHO {
    const NAME: &'static str = "KLPNIKBGGHO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.CLNGIMNBHPG = is.read_uint32()?;
                },
                74 => {
                    self.NANJOFKIELP.push(is.read_message()?);
                },
                90 => {
                    self.DCBDGNCODGD.push(is.read_message()?);
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IKKDELCFKGM)?;
                },
                32 => {
                    self.AIHABKFMIGC = is.read_uint32()?;
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
        if self.CLNGIMNBHPG != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.CLNGIMNBHPG);
        }
        for value in &self.NANJOFKIELP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.DCBDGNCODGD {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.IKKDELCFKGM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.AIHABKFMIGC != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.AIHABKFMIGC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.CLNGIMNBHPG != 0 {
            os.write_uint32(1, self.CLNGIMNBHPG)?;
        }
        for v in &self.NANJOFKIELP {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        for v in &self.DCBDGNCODGD {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if let Some(v) = self.IKKDELCFKGM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.AIHABKFMIGC != 0 {
            os.write_uint32(4, self.AIHABKFMIGC)?;
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

    fn new() -> KLPNIKBGGHO {
        KLPNIKBGGHO::new()
    }

    fn clear(&mut self) {
        self.CLNGIMNBHPG = 0;
        self.NANJOFKIELP.clear();
        self.DCBDGNCODGD.clear();
        self.IKKDELCFKGM.clear();
        self.AIHABKFMIGC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KLPNIKBGGHO {
        static instance: KLPNIKBGGHO = KLPNIKBGGHO {
            CLNGIMNBHPG: 0,
            NANJOFKIELP: ::std::vec::Vec::new(),
            DCBDGNCODGD: ::std::vec::Vec::new(),
            IKKDELCFKGM: ::protobuf::MessageField::none(),
            AIHABKFMIGC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KLPNIKBGGHO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KLPNIKBGGHO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KLPNIKBGGHO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KLPNIKBGGHO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KLPNIKBGGHO.proto\x1a\x11MABPAECJNNF.proto\"\xe1\x01\n\x0bKLPNIKBG\
    GHO\x12\x20\n\x0bCLNGIMNBHPG\x18\x01\x20\x01(\rR\x0bCLNGIMNBHPG\x12.\n\
    \x0bNANJOFKIELP\x18\t\x20\x03(\x0b2\x0c.MABPAECJNNFR\x0bNANJOFKIELP\x12.\
    \n\x0bDCBDGNCODGD\x18\x0b\x20\x03(\x0b2\x0c.MABPAECJNNFR\x0bDCBDGNCODGD\
    \x12.\n\x0bIKKDELCFKGM\x18\r\x20\x01(\x0b2\x0c.MABPAECJNNFR\x0bIKKDELCFK\
    GM\x12\x20\n\x0bAIHABKFMIGC\x18\x04\x20\x01(\rR\x0bAIHABKFMIGCb\x06proto\
    3\
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
            deps.push(super::MABPAECJNNF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(KLPNIKBGGHO::generated_message_descriptor_data());
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
