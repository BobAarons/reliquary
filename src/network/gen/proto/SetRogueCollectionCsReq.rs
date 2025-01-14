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

//! Generated file from `SetRogueCollectionCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SetRogueCollectionCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SetRogueCollectionCsReq {
    // message fields
    // @@protoc_insertion_point(field:SetRogueCollectionCsReq.LAECCEAFAKD)
    pub LAECCEAFAKD: ::std::vec::Vec<::protobuf::EnumOrUnknown<super::RogueCollectionExhibitionOperateType::RogueCollectionExhibitionOperateType>>,
    // @@protoc_insertion_point(field:SetRogueCollectionCsReq.PKMIOMOOILK)
    pub PKMIOMOOILK: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SetRogueCollectionCsReq.KGBJDEPIGLC)
    pub KGBJDEPIGLC: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:SetRogueCollectionCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SetRogueCollectionCsReq {
    fn default() -> &'a SetRogueCollectionCsReq {
        <SetRogueCollectionCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SetRogueCollectionCsReq {
    pub fn new() -> SetRogueCollectionCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LAECCEAFAKD",
            |m: &SetRogueCollectionCsReq| { &m.LAECCEAFAKD },
            |m: &mut SetRogueCollectionCsReq| { &mut m.LAECCEAFAKD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PKMIOMOOILK",
            |m: &SetRogueCollectionCsReq| { &m.PKMIOMOOILK },
            |m: &mut SetRogueCollectionCsReq| { &mut m.PKMIOMOOILK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KGBJDEPIGLC",
            |m: &SetRogueCollectionCsReq| { &m.KGBJDEPIGLC },
            |m: &mut SetRogueCollectionCsReq| { &mut m.KGBJDEPIGLC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SetRogueCollectionCsReq>(
            "SetRogueCollectionCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SetRogueCollectionCsReq {
    const NAME: &'static str = "SetRogueCollectionCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.LAECCEAFAKD.push(is.read_enum_or_unknown()?);
                },
                18 => {
                    ::protobuf::rt::read_repeated_packed_enum_or_unknown_into(is, &mut self.LAECCEAFAKD)?
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.PKMIOMOOILK)?;
                },
                120 => {
                    self.PKMIOMOOILK.push(is.read_uint32()?);
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.KGBJDEPIGLC)?;
                },
                56 => {
                    self.KGBJDEPIGLC.push(is.read_uint32()?);
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
        for value in &self.LAECCEAFAKD {
            my_size += ::protobuf::rt::int32_size(2, value.value());
        };
        for value in &self.PKMIOMOOILK {
            my_size += ::protobuf::rt::uint32_size(15, *value);
        };
        for value in &self.KGBJDEPIGLC {
            my_size += ::protobuf::rt::uint32_size(7, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.LAECCEAFAKD {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(v))?;
        };
        for v in &self.PKMIOMOOILK {
            os.write_uint32(15, *v)?;
        };
        for v in &self.KGBJDEPIGLC {
            os.write_uint32(7, *v)?;
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

    fn new() -> SetRogueCollectionCsReq {
        SetRogueCollectionCsReq::new()
    }

    fn clear(&mut self) {
        self.LAECCEAFAKD.clear();
        self.PKMIOMOOILK.clear();
        self.KGBJDEPIGLC.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SetRogueCollectionCsReq {
        static instance: SetRogueCollectionCsReq = SetRogueCollectionCsReq {
            LAECCEAFAKD: ::std::vec::Vec::new(),
            PKMIOMOOILK: ::std::vec::Vec::new(),
            KGBJDEPIGLC: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SetRogueCollectionCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SetRogueCollectionCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SetRogueCollectionCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetRogueCollectionCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dSetRogueCollectionCsReq.proto\x1a*RogueCollectionExhibitionOperate\
    Type.proto\"\xa6\x01\n\x17SetRogueCollectionCsReq\x12G\n\x0bLAECCEAFAKD\
    \x18\x02\x20\x03(\x0e2%.RogueCollectionExhibitionOperateTypeR\x0bLAECCEA\
    FAKD\x12\x20\n\x0bPKMIOMOOILK\x18\x0f\x20\x03(\rR\x0bPKMIOMOOILK\x12\x20\
    \n\x0bKGBJDEPIGLC\x18\x07\x20\x03(\rR\x0bKGBJDEPIGLCb\x06proto3\
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
            deps.push(super::RogueCollectionExhibitionOperateType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SetRogueCollectionCsReq::generated_message_descriptor_data());
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
