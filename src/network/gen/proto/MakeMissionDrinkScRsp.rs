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

//! Generated file from `MakeMissionDrinkScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MakeMissionDrinkScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MakeMissionDrinkScRsp {
    // message fields
    // @@protoc_insertion_point(field:MakeMissionDrinkScRsp.is_save)
    pub is_save: bool,
    // @@protoc_insertion_point(field:MakeMissionDrinkScRsp.is_succ)
    pub is_succ: bool,
    // @@protoc_insertion_point(field:MakeMissionDrinkScRsp.custom_drink)
    pub custom_drink: ::protobuf::MessageField<super::PNLHADCBNMN::PNLHADCBNMN>,
    // @@protoc_insertion_point(field:MakeMissionDrinkScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MakeMissionDrinkScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MakeMissionDrinkScRsp {
    fn default() -> &'a MakeMissionDrinkScRsp {
        <MakeMissionDrinkScRsp as ::protobuf::Message>::default_instance()
    }
}

impl MakeMissionDrinkScRsp {
    pub fn new() -> MakeMissionDrinkScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_save",
            |m: &MakeMissionDrinkScRsp| { &m.is_save },
            |m: &mut MakeMissionDrinkScRsp| { &mut m.is_save },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_succ",
            |m: &MakeMissionDrinkScRsp| { &m.is_succ },
            |m: &mut MakeMissionDrinkScRsp| { &mut m.is_succ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PNLHADCBNMN::PNLHADCBNMN>(
            "custom_drink",
            |m: &MakeMissionDrinkScRsp| { &m.custom_drink },
            |m: &mut MakeMissionDrinkScRsp| { &mut m.custom_drink },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &MakeMissionDrinkScRsp| { &m.retcode },
            |m: &mut MakeMissionDrinkScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MakeMissionDrinkScRsp>(
            "MakeMissionDrinkScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MakeMissionDrinkScRsp {
    const NAME: &'static str = "MakeMissionDrinkScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.is_save = is.read_bool()?;
                },
                72 => {
                    self.is_succ = is.read_bool()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.custom_drink)?;
                },
                104 => {
                    self.retcode = is.read_uint32()?;
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
        if self.is_save != false {
            my_size += 1 + 1;
        }
        if self.is_succ != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.custom_drink.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_save != false {
            os.write_bool(12, self.is_save)?;
        }
        if self.is_succ != false {
            os.write_bool(9, self.is_succ)?;
        }
        if let Some(v) = self.custom_drink.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(13, self.retcode)?;
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

    fn new() -> MakeMissionDrinkScRsp {
        MakeMissionDrinkScRsp::new()
    }

    fn clear(&mut self) {
        self.is_save = false;
        self.is_succ = false;
        self.custom_drink.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MakeMissionDrinkScRsp {
        static instance: MakeMissionDrinkScRsp = MakeMissionDrinkScRsp {
            is_save: false,
            is_succ: false,
            custom_drink: ::protobuf::MessageField::none(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MakeMissionDrinkScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MakeMissionDrinkScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MakeMissionDrinkScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MakeMissionDrinkScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bMakeMissionDrinkScRsp.proto\x1a\x11PNLHADCBNMN.proto\"\x94\x01\n\
    \x15MakeMissionDrinkScRsp\x12\x17\n\x07is_save\x18\x0c\x20\x01(\x08R\x06\
    isSave\x12\x17\n\x07is_succ\x18\t\x20\x01(\x08R\x06isSucc\x12/\n\x0ccust\
    om_drink\x18\x04\x20\x01(\x0b2\x0c.PNLHADCBNMNR\x0bcustomDrink\x12\x18\n\
    \x07retcode\x18\r\x20\x01(\rR\x07retcodeb\x06proto3\
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
            deps.push(super::PNLHADCBNMN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MakeMissionDrinkScRsp::generated_message_descriptor_data());
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