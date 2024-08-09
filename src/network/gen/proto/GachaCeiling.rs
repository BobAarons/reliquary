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

//! Generated file from `GachaCeiling.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GachaCeiling)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GachaCeiling {
    // message fields
    // @@protoc_insertion_point(field:GachaCeiling.is_claimed)
    pub is_claimed: bool,
    // @@protoc_insertion_point(field:GachaCeiling.avatar_list)
    pub avatar_list: ::std::vec::Vec<super::GachaCeilingAvatar::GachaCeilingAvatar>,
    // @@protoc_insertion_point(field:GachaCeiling.ceiling_num)
    pub ceiling_num: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GachaCeiling.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GachaCeiling {
    fn default() -> &'a GachaCeiling {
        <GachaCeiling as ::protobuf::Message>::default_instance()
    }
}

impl GachaCeiling {
    pub fn new() -> GachaCeiling {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_claimed",
            |m: &GachaCeiling| { &m.is_claimed },
            |m: &mut GachaCeiling| { &mut m.is_claimed },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &GachaCeiling| { &m.avatar_list },
            |m: &mut GachaCeiling| { &mut m.avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ceiling_num",
            |m: &GachaCeiling| { &m.ceiling_num },
            |m: &mut GachaCeiling| { &mut m.ceiling_num },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GachaCeiling>(
            "GachaCeiling",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GachaCeiling {
    const NAME: &'static str = "GachaCeiling";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.is_claimed = is.read_bool()?;
                },
                26 => {
                    self.avatar_list.push(is.read_message()?);
                },
                40 => {
                    self.ceiling_num = is.read_uint32()?;
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
        if self.is_claimed != false {
            my_size += 1 + 1;
        }
        for value in &self.avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.ceiling_num != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.ceiling_num);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_claimed != false {
            os.write_bool(4, self.is_claimed)?;
        }
        for v in &self.avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.ceiling_num != 0 {
            os.write_uint32(5, self.ceiling_num)?;
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

    fn new() -> GachaCeiling {
        GachaCeiling::new()
    }

    fn clear(&mut self) {
        self.is_claimed = false;
        self.avatar_list.clear();
        self.ceiling_num = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GachaCeiling {
        static instance: GachaCeiling = GachaCeiling {
            is_claimed: false,
            avatar_list: ::std::vec::Vec::new(),
            ceiling_num: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GachaCeiling {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GachaCeiling").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GachaCeiling {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GachaCeiling {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12GachaCeiling.proto\x1a\x18GachaCeilingAvatar.proto\"\x84\x01\n\x0c\
    GachaCeiling\x12\x1d\n\nis_claimed\x18\x04\x20\x01(\x08R\tisClaimed\x124\
    \n\x0bavatar_list\x18\x03\x20\x03(\x0b2\x13.GachaCeilingAvatarR\navatarL\
    ist\x12\x1f\n\x0bceiling_num\x18\x05\x20\x01(\rR\nceilingNumB\x15\n\x13e\
    mu.lunarcore.protob\x06proto3\
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
            deps.push(super::GachaCeilingAvatar::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GachaCeiling::generated_message_descriptor_data());
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
