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

//! Generated file from `ComposeSelectedRelicCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ComposeSelectedRelicCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ComposeSelectedRelicCsReq {
    // message fields
    // @@protoc_insertion_point(field:ComposeSelectedRelicCsReq.compose_id)
    pub compose_id: u32,
    // @@protoc_insertion_point(field:ComposeSelectedRelicCsReq.compose_item_list)
    pub compose_item_list: ::protobuf::MessageField<super::ItemCostList::ItemCostList>,
    // @@protoc_insertion_point(field:ComposeSelectedRelicCsReq.main_affix_id)
    pub main_affix_id: u32,
    // @@protoc_insertion_point(field:ComposeSelectedRelicCsReq.compose_relic_id)
    pub compose_relic_id: u32,
    // @@protoc_insertion_point(field:ComposeSelectedRelicCsReq.count)
    pub count: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ComposeSelectedRelicCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ComposeSelectedRelicCsReq {
    fn default() -> &'a ComposeSelectedRelicCsReq {
        <ComposeSelectedRelicCsReq as ::protobuf::Message>::default_instance()
    }
}

impl ComposeSelectedRelicCsReq {
    pub fn new() -> ComposeSelectedRelicCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "compose_id",
            |m: &ComposeSelectedRelicCsReq| { &m.compose_id },
            |m: &mut ComposeSelectedRelicCsReq| { &mut m.compose_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemCostList::ItemCostList>(
            "compose_item_list",
            |m: &ComposeSelectedRelicCsReq| { &m.compose_item_list },
            |m: &mut ComposeSelectedRelicCsReq| { &mut m.compose_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "main_affix_id",
            |m: &ComposeSelectedRelicCsReq| { &m.main_affix_id },
            |m: &mut ComposeSelectedRelicCsReq| { &mut m.main_affix_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "compose_relic_id",
            |m: &ComposeSelectedRelicCsReq| { &m.compose_relic_id },
            |m: &mut ComposeSelectedRelicCsReq| { &mut m.compose_relic_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "count",
            |m: &ComposeSelectedRelicCsReq| { &m.count },
            |m: &mut ComposeSelectedRelicCsReq| { &mut m.count },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ComposeSelectedRelicCsReq>(
            "ComposeSelectedRelicCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ComposeSelectedRelicCsReq {
    const NAME: &'static str = "ComposeSelectedRelicCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.compose_id = is.read_uint32()?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.compose_item_list)?;
                },
                24 => {
                    self.main_affix_id = is.read_uint32()?;
                },
                48 => {
                    self.compose_relic_id = is.read_uint32()?;
                },
                56 => {
                    self.count = is.read_uint32()?;
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
        if self.compose_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.compose_id);
        }
        if let Some(v) = self.compose_item_list.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.main_affix_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.main_affix_id);
        }
        if self.compose_relic_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.compose_relic_id);
        }
        if self.count != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.count);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.compose_id != 0 {
            os.write_uint32(2, self.compose_id)?;
        }
        if let Some(v) = self.compose_item_list.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if self.main_affix_id != 0 {
            os.write_uint32(3, self.main_affix_id)?;
        }
        if self.compose_relic_id != 0 {
            os.write_uint32(6, self.compose_relic_id)?;
        }
        if self.count != 0 {
            os.write_uint32(7, self.count)?;
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

    fn new() -> ComposeSelectedRelicCsReq {
        ComposeSelectedRelicCsReq::new()
    }

    fn clear(&mut self) {
        self.compose_id = 0;
        self.compose_item_list.clear();
        self.main_affix_id = 0;
        self.compose_relic_id = 0;
        self.count = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ComposeSelectedRelicCsReq {
        static instance: ComposeSelectedRelicCsReq = ComposeSelectedRelicCsReq {
            compose_id: 0,
            compose_item_list: ::protobuf::MessageField::none(),
            main_affix_id: 0,
            compose_relic_id: 0,
            count: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ComposeSelectedRelicCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ComposeSelectedRelicCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ComposeSelectedRelicCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ComposeSelectedRelicCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fComposeSelectedRelicCsReq.proto\x1a\x12ItemCostList.proto\"\xd9\
    \x01\n\x19ComposeSelectedRelicCsReq\x12\x1d\n\ncompose_id\x18\x02\x20\
    \x01(\rR\tcomposeId\x129\n\x11compose_item_list\x18\x0c\x20\x01(\x0b2\r.\
    ItemCostListR\x0fcomposeItemList\x12\"\n\rmain_affix_id\x18\x03\x20\x01(\
    \rR\x0bmainAffixId\x12(\n\x10compose_relic_id\x18\x06\x20\x01(\rR\x0ecom\
    poseRelicId\x12\x14\n\x05count\x18\x07\x20\x01(\rR\x05countB\x15\n\x13em\
    u.lunarcore.protob\x06proto3\
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
            deps.push(super::ItemCostList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ComposeSelectedRelicCsReq::generated_message_descriptor_data());
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
