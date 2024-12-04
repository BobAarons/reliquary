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

//! Generated file from `RaidKickByServerScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RaidKickByServerScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RaidKickByServerScNotify {
    // message fields
    // @@protoc_insertion_point(field:RaidKickByServerScNotify.ODOAJJGMBCL)
    pub ODOAJJGMBCL: u32,
    // @@protoc_insertion_point(field:RaidKickByServerScNotify.GOOIEHKAGLC)
    pub GOOIEHKAGLC: ::protobuf::MessageField<super::GDHMBEFBPHM::GDHMBEFBPHM>,
    // @@protoc_insertion_point(field:RaidKickByServerScNotify.KGGHLADEKGP)
    pub KGGHLADEKGP: ::protobuf::EnumOrUnknown<super::OJLOFJKJBOL::OJLOFJKJBOL>,
    // @@protoc_insertion_point(field:RaidKickByServerScNotify.DNMJBNNJLEL)
    pub DNMJBNNJLEL: u32,
    // @@protoc_insertion_point(field:RaidKickByServerScNotify.HGGFOJICNCG)
    pub HGGFOJICNCG: ::protobuf::MessageField<super::IJAIFMPFJDN::IJAIFMPFJDN>,
    // special fields
    // @@protoc_insertion_point(special_field:RaidKickByServerScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RaidKickByServerScNotify {
    fn default() -> &'a RaidKickByServerScNotify {
        <RaidKickByServerScNotify as ::protobuf::Message>::default_instance()
    }
}

impl RaidKickByServerScNotify {
    pub fn new() -> RaidKickByServerScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ODOAJJGMBCL",
            |m: &RaidKickByServerScNotify| { &m.ODOAJJGMBCL },
            |m: &mut RaidKickByServerScNotify| { &mut m.ODOAJJGMBCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GDHMBEFBPHM::GDHMBEFBPHM>(
            "GOOIEHKAGLC",
            |m: &RaidKickByServerScNotify| { &m.GOOIEHKAGLC },
            |m: &mut RaidKickByServerScNotify| { &mut m.GOOIEHKAGLC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGGHLADEKGP",
            |m: &RaidKickByServerScNotify| { &m.KGGHLADEKGP },
            |m: &mut RaidKickByServerScNotify| { &mut m.KGGHLADEKGP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNMJBNNJLEL",
            |m: &RaidKickByServerScNotify| { &m.DNMJBNNJLEL },
            |m: &mut RaidKickByServerScNotify| { &mut m.DNMJBNNJLEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IJAIFMPFJDN::IJAIFMPFJDN>(
            "HGGFOJICNCG",
            |m: &RaidKickByServerScNotify| { &m.HGGFOJICNCG },
            |m: &mut RaidKickByServerScNotify| { &mut m.HGGFOJICNCG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RaidKickByServerScNotify>(
            "RaidKickByServerScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RaidKickByServerScNotify {
    const NAME: &'static str = "RaidKickByServerScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.ODOAJJGMBCL = is.read_uint32()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GOOIEHKAGLC)?;
                },
                8 => {
                    self.KGGHLADEKGP = is.read_enum_or_unknown()?;
                },
                32 => {
                    self.DNMJBNNJLEL = is.read_uint32()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HGGFOJICNCG)?;
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
        if self.ODOAJJGMBCL != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.ODOAJJGMBCL);
        }
        if let Some(v) = self.GOOIEHKAGLC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.KGGHLADEKGP != ::protobuf::EnumOrUnknown::new(super::OJLOFJKJBOL::OJLOFJKJBOL::RAID_KICK_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(1, self.KGGHLADEKGP.value());
        }
        if self.DNMJBNNJLEL != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.DNMJBNNJLEL);
        }
        if let Some(v) = self.HGGFOJICNCG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ODOAJJGMBCL != 0 {
            os.write_uint32(9, self.ODOAJJGMBCL)?;
        }
        if let Some(v) = self.GOOIEHKAGLC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.KGGHLADEKGP != ::protobuf::EnumOrUnknown::new(super::OJLOFJKJBOL::OJLOFJKJBOL::RAID_KICK_REASON_NONE) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.KGGHLADEKGP))?;
        }
        if self.DNMJBNNJLEL != 0 {
            os.write_uint32(4, self.DNMJBNNJLEL)?;
        }
        if let Some(v) = self.HGGFOJICNCG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
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

    fn new() -> RaidKickByServerScNotify {
        RaidKickByServerScNotify::new()
    }

    fn clear(&mut self) {
        self.ODOAJJGMBCL = 0;
        self.GOOIEHKAGLC.clear();
        self.KGGHLADEKGP = ::protobuf::EnumOrUnknown::new(super::OJLOFJKJBOL::OJLOFJKJBOL::RAID_KICK_REASON_NONE);
        self.DNMJBNNJLEL = 0;
        self.HGGFOJICNCG.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RaidKickByServerScNotify {
        static instance: RaidKickByServerScNotify = RaidKickByServerScNotify {
            ODOAJJGMBCL: 0,
            GOOIEHKAGLC: ::protobuf::MessageField::none(),
            KGGHLADEKGP: ::protobuf::EnumOrUnknown::from_i32(0),
            DNMJBNNJLEL: 0,
            HGGFOJICNCG: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RaidKickByServerScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RaidKickByServerScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RaidKickByServerScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaidKickByServerScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eRaidKickByServerScNotify.proto\x1a\x11GDHMBEFBPHM.proto\x1a\x11IJA\
    IFMPFJDN.proto\x1a\x11OJLOFJKJBOL.proto\"\xee\x01\n\x18RaidKickByServerS\
    cNotify\x12\x20\n\x0bODOAJJGMBCL\x18\t\x20\x01(\rR\x0bODOAJJGMBCL\x12.\n\
    \x0bGOOIEHKAGLC\x18\x05\x20\x01(\x0b2\x0c.GDHMBEFBPHMR\x0bGOOIEHKAGLC\
    \x12.\n\x0bKGGHLADEKGP\x18\x01\x20\x01(\x0e2\x0c.OJLOFJKJBOLR\x0bKGGHLAD\
    EKGP\x12\x20\n\x0bDNMJBNNJLEL\x18\x04\x20\x01(\rR\x0bDNMJBNNJLEL\x12.\n\
    \x0bHGGFOJICNCG\x18\r\x20\x01(\x0b2\x0c.IJAIFMPFJDNR\x0bHGGFOJICNCGb\x06\
    proto3\
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
            deps.push(super::GDHMBEFBPHM::file_descriptor().clone());
            deps.push(super::IJAIFMPFJDN::file_descriptor().clone());
            deps.push(super::OJLOFJKJBOL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RaidKickByServerScNotify::generated_message_descriptor_data());
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