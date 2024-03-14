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

//! Generated file from `SceneEntityMoveCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SceneEntityMoveCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneEntityMoveCsReq {
    // message fields
    // @@protoc_insertion_point(field:SceneEntityMoveCsReq.entry_id)
    pub entry_id: u32,
    // @@protoc_insertion_point(field:SceneEntityMoveCsReq.entity_motion_list)
    pub entity_motion_list: ::std::vec::Vec<super::EntityMotion::EntityMotion>,
    // special fields
    // @@protoc_insertion_point(special_field:SceneEntityMoveCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneEntityMoveCsReq {
    fn default() -> &'a SceneEntityMoveCsReq {
        <SceneEntityMoveCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SceneEntityMoveCsReq {
    pub fn new() -> SceneEntityMoveCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entry_id",
            |m: &SceneEntityMoveCsReq| { &m.entry_id },
            |m: &mut SceneEntityMoveCsReq| { &mut m.entry_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "entity_motion_list",
            |m: &SceneEntityMoveCsReq| { &m.entity_motion_list },
            |m: &mut SceneEntityMoveCsReq| { &mut m.entity_motion_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneEntityMoveCsReq>(
            "SceneEntityMoveCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneEntityMoveCsReq {
    const NAME: &'static str = "SceneEntityMoveCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.entry_id = is.read_uint32()?;
                },
                58 => {
                    self.entity_motion_list.push(is.read_message()?);
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
        if self.entry_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.entry_id);
        }
        for value in &self.entity_motion_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.entry_id != 0 {
            os.write_uint32(8, self.entry_id)?;
        }
        for v in &self.entity_motion_list {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
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

    fn new() -> SceneEntityMoveCsReq {
        SceneEntityMoveCsReq::new()
    }

    fn clear(&mut self) {
        self.entry_id = 0;
        self.entity_motion_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneEntityMoveCsReq {
        static instance: SceneEntityMoveCsReq = SceneEntityMoveCsReq {
            entry_id: 0,
            entity_motion_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneEntityMoveCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneEntityMoveCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneEntityMoveCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneEntityMoveCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aSceneEntityMoveCsReq.proto\x1a\x12EntityMotion.proto\"n\n\x14Scene\
    EntityMoveCsReq\x12\x19\n\x08entry_id\x18\x08\x20\x01(\rR\x07entryId\x12\
    ;\n\x12entity_motion_list\x18\x07\x20\x03(\x0b2\r.EntityMotionR\x10entit\
    yMotionListB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::EntityMotion::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneEntityMoveCsReq::generated_message_descriptor_data());
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