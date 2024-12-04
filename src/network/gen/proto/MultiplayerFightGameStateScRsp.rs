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

//! Generated file from `MultiplayerFightGameStateScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MultiplayerFightGameStateScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MultiplayerFightGameStateScRsp {
    // message fields
    // @@protoc_insertion_point(field:MultiplayerFightGameStateScRsp.LKLJKGKLHID)
    pub LKLJKGKLHID: ::std::vec::Vec<super::DBBLOFLAAMH::DBBLOFLAAMH>,
    // @@protoc_insertion_point(field:MultiplayerFightGameStateScRsp.DJNKCKJMNIL)
    pub DJNKCKJMNIL: ::protobuf::MessageField<super::MNFBAOKFOPM::MNFBAOKFOPM>,
    // @@protoc_insertion_point(field:MultiplayerFightGameStateScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MultiplayerFightGameStateScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MultiplayerFightGameStateScRsp {
    fn default() -> &'a MultiplayerFightGameStateScRsp {
        <MultiplayerFightGameStateScRsp as ::protobuf::Message>::default_instance()
    }
}

impl MultiplayerFightGameStateScRsp {
    pub fn new() -> MultiplayerFightGameStateScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LKLJKGKLHID",
            |m: &MultiplayerFightGameStateScRsp| { &m.LKLJKGKLHID },
            |m: &mut MultiplayerFightGameStateScRsp| { &mut m.LKLJKGKLHID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MNFBAOKFOPM::MNFBAOKFOPM>(
            "DJNKCKJMNIL",
            |m: &MultiplayerFightGameStateScRsp| { &m.DJNKCKJMNIL },
            |m: &mut MultiplayerFightGameStateScRsp| { &mut m.DJNKCKJMNIL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &MultiplayerFightGameStateScRsp| { &m.ADADHIHDHJC },
            |m: &mut MultiplayerFightGameStateScRsp| { &mut m.ADADHIHDHJC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MultiplayerFightGameStateScRsp>(
            "MultiplayerFightGameStateScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MultiplayerFightGameStateScRsp {
    const NAME: &'static str = "MultiplayerFightGameStateScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    self.LKLJKGKLHID.push(is.read_message()?);
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DJNKCKJMNIL)?;
                },
                40 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
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
        for value in &self.LKLJKGKLHID {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.DJNKCKJMNIL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.ADADHIHDHJC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.LKLJKGKLHID {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        if let Some(v) = self.DJNKCKJMNIL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(5, self.ADADHIHDHJC)?;
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

    fn new() -> MultiplayerFightGameStateScRsp {
        MultiplayerFightGameStateScRsp::new()
    }

    fn clear(&mut self) {
        self.LKLJKGKLHID.clear();
        self.DJNKCKJMNIL.clear();
        self.ADADHIHDHJC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MultiplayerFightGameStateScRsp {
        static instance: MultiplayerFightGameStateScRsp = MultiplayerFightGameStateScRsp {
            LKLJKGKLHID: ::std::vec::Vec::new(),
            DJNKCKJMNIL: ::protobuf::MessageField::none(),
            ADADHIHDHJC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MultiplayerFightGameStateScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MultiplayerFightGameStateScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MultiplayerFightGameStateScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MultiplayerFightGameStateScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$MultiplayerFightGameStateScRsp.proto\x1a\x11DBBLOFLAAMH.proto\x1a\x11\
    MNFBAOKFOPM.proto\"\xa2\x01\n\x1eMultiplayerFightGameStateScRsp\x12.\n\
    \x0bLKLJKGKLHID\x18\x0e\x20\x03(\x0b2\x0c.DBBLOFLAAMHR\x0bLKLJKGKLHID\
    \x12.\n\x0bDJNKCKJMNIL\x18\x0b\x20\x01(\x0b2\x0c.MNFBAOKFOPMR\x0bDJNKCKJ\
    MNIL\x12\x20\n\x0bADADHIHDHJC\x18\x05\x20\x01(\rR\x0bADADHIHDHJCb\x06pro\
    to3\
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
            deps.push(super::DBBLOFLAAMH::file_descriptor().clone());
            deps.push(super::MNFBAOKFOPM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MultiplayerFightGameStateScRsp::generated_message_descriptor_data());
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