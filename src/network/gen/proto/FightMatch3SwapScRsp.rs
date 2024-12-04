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

//! Generated file from `FightMatch3SwapScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FightMatch3SwapScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FightMatch3SwapScRsp {
    // message fields
    // @@protoc_insertion_point(field:FightMatch3SwapScRsp.HOFBEKABNNL)
    pub HOFBEKABNNL: bool,
    // @@protoc_insertion_point(field:FightMatch3SwapScRsp.NAGKACACHGD)
    pub NAGKACACHGD: u32,
    // @@protoc_insertion_point(field:FightMatch3SwapScRsp.NBEKPGDILJE)
    pub NBEKPGDILJE: ::protobuf::MessageField<super::ELAMGBPKDFA::ELAMGBPKDFA>,
    // @@protoc_insertion_point(field:FightMatch3SwapScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FightMatch3SwapScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FightMatch3SwapScRsp {
    fn default() -> &'a FightMatch3SwapScRsp {
        <FightMatch3SwapScRsp as ::protobuf::Message>::default_instance()
    }
}

impl FightMatch3SwapScRsp {
    pub fn new() -> FightMatch3SwapScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HOFBEKABNNL",
            |m: &FightMatch3SwapScRsp| { &m.HOFBEKABNNL },
            |m: &mut FightMatch3SwapScRsp| { &mut m.HOFBEKABNNL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NAGKACACHGD",
            |m: &FightMatch3SwapScRsp| { &m.NAGKACACHGD },
            |m: &mut FightMatch3SwapScRsp| { &mut m.NAGKACACHGD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ELAMGBPKDFA::ELAMGBPKDFA>(
            "NBEKPGDILJE",
            |m: &FightMatch3SwapScRsp| { &m.NBEKPGDILJE },
            |m: &mut FightMatch3SwapScRsp| { &mut m.NBEKPGDILJE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &FightMatch3SwapScRsp| { &m.ADADHIHDHJC },
            |m: &mut FightMatch3SwapScRsp| { &mut m.ADADHIHDHJC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FightMatch3SwapScRsp>(
            "FightMatch3SwapScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FightMatch3SwapScRsp {
    const NAME: &'static str = "FightMatch3SwapScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.HOFBEKABNNL = is.read_bool()?;
                },
                80 => {
                    self.NAGKACACHGD = is.read_uint32()?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NBEKPGDILJE)?;
                },
                48 => {
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
        if self.HOFBEKABNNL != false {
            my_size += 1 + 1;
        }
        if self.NAGKACACHGD != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.NAGKACACHGD);
        }
        if let Some(v) = self.NBEKPGDILJE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.ADADHIHDHJC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HOFBEKABNNL != false {
            os.write_bool(9, self.HOFBEKABNNL)?;
        }
        if self.NAGKACACHGD != 0 {
            os.write_uint32(10, self.NAGKACACHGD)?;
        }
        if let Some(v) = self.NBEKPGDILJE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(6, self.ADADHIHDHJC)?;
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

    fn new() -> FightMatch3SwapScRsp {
        FightMatch3SwapScRsp::new()
    }

    fn clear(&mut self) {
        self.HOFBEKABNNL = false;
        self.NAGKACACHGD = 0;
        self.NBEKPGDILJE.clear();
        self.ADADHIHDHJC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FightMatch3SwapScRsp {
        static instance: FightMatch3SwapScRsp = FightMatch3SwapScRsp {
            HOFBEKABNNL: false,
            NAGKACACHGD: 0,
            NBEKPGDILJE: ::protobuf::MessageField::none(),
            ADADHIHDHJC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FightMatch3SwapScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FightMatch3SwapScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FightMatch3SwapScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FightMatch3SwapScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aFightMatch3SwapScRsp.proto\x1a\x11ELAMGBPKDFA.proto\"\xac\x01\n\
    \x14FightMatch3SwapScRsp\x12\x20\n\x0bHOFBEKABNNL\x18\t\x20\x01(\x08R\
    \x0bHOFBEKABNNL\x12\x20\n\x0bNAGKACACHGD\x18\n\x20\x01(\rR\x0bNAGKACACHG\
    D\x12.\n\x0bNBEKPGDILJE\x18\x08\x20\x01(\x0b2\x0c.ELAMGBPKDFAR\x0bNBEKPG\
    DILJE\x12\x20\n\x0bADADHIHDHJC\x18\x06\x20\x01(\rR\x0bADADHIHDHJCb\x06pr\
    oto3\
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
            deps.push(super::ELAMGBPKDFA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FightMatch3SwapScRsp::generated_message_descriptor_data());
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