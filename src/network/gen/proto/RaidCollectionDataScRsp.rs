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

//! Generated file from `RaidCollectionDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RaidCollectionDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RaidCollectionDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:RaidCollectionDataScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:RaidCollectionDataScRsp.PPNKHDECJBP)
    pub PPNKHDECJBP: ::std::vec::Vec<super::OKFNNPMACMO::OKFNNPMACMO>,
    // special fields
    // @@protoc_insertion_point(special_field:RaidCollectionDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RaidCollectionDataScRsp {
    fn default() -> &'a RaidCollectionDataScRsp {
        <RaidCollectionDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl RaidCollectionDataScRsp {
    pub fn new() -> RaidCollectionDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &RaidCollectionDataScRsp| { &m.ADADHIHDHJC },
            |m: &mut RaidCollectionDataScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PPNKHDECJBP",
            |m: &RaidCollectionDataScRsp| { &m.PPNKHDECJBP },
            |m: &mut RaidCollectionDataScRsp| { &mut m.PPNKHDECJBP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RaidCollectionDataScRsp>(
            "RaidCollectionDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RaidCollectionDataScRsp {
    const NAME: &'static str = "RaidCollectionDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                26 => {
                    self.PPNKHDECJBP.push(is.read_message()?);
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
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.ADADHIHDHJC);
        }
        for value in &self.PPNKHDECJBP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(8, self.ADADHIHDHJC)?;
        }
        for v in &self.PPNKHDECJBP {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> RaidCollectionDataScRsp {
        RaidCollectionDataScRsp::new()
    }

    fn clear(&mut self) {
        self.ADADHIHDHJC = 0;
        self.PPNKHDECJBP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RaidCollectionDataScRsp {
        static instance: RaidCollectionDataScRsp = RaidCollectionDataScRsp {
            ADADHIHDHJC: 0,
            PPNKHDECJBP: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RaidCollectionDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RaidCollectionDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RaidCollectionDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaidCollectionDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dRaidCollectionDataScRsp.proto\x1a\x11OKFNNPMACMO.proto\"k\n\x17Rai\
    dCollectionDataScRsp\x12\x20\n\x0bADADHIHDHJC\x18\x08\x20\x01(\rR\x0bADA\
    DHIHDHJC\x12.\n\x0bPPNKHDECJBP\x18\x03\x20\x03(\x0b2\x0c.OKFNNPMACMOR\
    \x0bPPNKHDECJBPb\x06proto3\
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
            deps.push(super::OKFNNPMACMO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RaidCollectionDataScRsp::generated_message_descriptor_data());
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