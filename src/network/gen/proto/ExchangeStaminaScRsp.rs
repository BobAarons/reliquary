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

//! Generated file from `ExchangeStaminaScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ExchangeStaminaScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ExchangeStaminaScRsp {
    // message fields
    // @@protoc_insertion_point(field:ExchangeStaminaScRsp.item_cost_list)
    pub item_cost_list: ::std::vec::Vec<super::ItemCost::ItemCost>,
    // @@protoc_insertion_point(field:ExchangeStaminaScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:ExchangeStaminaScRsp.stamina_add)
    pub stamina_add: u32,
    // @@protoc_insertion_point(field:ExchangeStaminaScRsp.last_recover_time)
    pub last_recover_time: i64,
    // @@protoc_insertion_point(field:ExchangeStaminaScRsp.exchange_times)
    pub exchange_times: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ExchangeStaminaScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ExchangeStaminaScRsp {
    fn default() -> &'a ExchangeStaminaScRsp {
        <ExchangeStaminaScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ExchangeStaminaScRsp {
    pub fn new() -> ExchangeStaminaScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "item_cost_list",
            |m: &ExchangeStaminaScRsp| { &m.item_cost_list },
            |m: &mut ExchangeStaminaScRsp| { &mut m.item_cost_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ExchangeStaminaScRsp| { &m.retcode },
            |m: &mut ExchangeStaminaScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stamina_add",
            |m: &ExchangeStaminaScRsp| { &m.stamina_add },
            |m: &mut ExchangeStaminaScRsp| { &mut m.stamina_add },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "last_recover_time",
            |m: &ExchangeStaminaScRsp| { &m.last_recover_time },
            |m: &mut ExchangeStaminaScRsp| { &mut m.last_recover_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exchange_times",
            |m: &ExchangeStaminaScRsp| { &m.exchange_times },
            |m: &mut ExchangeStaminaScRsp| { &mut m.exchange_times },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ExchangeStaminaScRsp>(
            "ExchangeStaminaScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ExchangeStaminaScRsp {
    const NAME: &'static str = "ExchangeStaminaScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    self.item_cost_list.push(is.read_message()?);
                },
                8 => {
                    self.retcode = is.read_uint32()?;
                },
                120 => {
                    self.stamina_add = is.read_uint32()?;
                },
                40 => {
                    self.last_recover_time = is.read_int64()?;
                },
                96 => {
                    self.exchange_times = is.read_uint32()?;
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
        for value in &self.item_cost_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.retcode);
        }
        if self.stamina_add != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.stamina_add);
        }
        if self.last_recover_time != 0 {
            my_size += ::protobuf::rt::int64_size(5, self.last_recover_time);
        }
        if self.exchange_times != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.exchange_times);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.item_cost_list {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if self.retcode != 0 {
            os.write_uint32(1, self.retcode)?;
        }
        if self.stamina_add != 0 {
            os.write_uint32(15, self.stamina_add)?;
        }
        if self.last_recover_time != 0 {
            os.write_int64(5, self.last_recover_time)?;
        }
        if self.exchange_times != 0 {
            os.write_uint32(12, self.exchange_times)?;
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

    fn new() -> ExchangeStaminaScRsp {
        ExchangeStaminaScRsp::new()
    }

    fn clear(&mut self) {
        self.item_cost_list.clear();
        self.retcode = 0;
        self.stamina_add = 0;
        self.last_recover_time = 0;
        self.exchange_times = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ExchangeStaminaScRsp {
        static instance: ExchangeStaminaScRsp = ExchangeStaminaScRsp {
            item_cost_list: ::std::vec::Vec::new(),
            retcode: 0,
            stamina_add: 0,
            last_recover_time: 0,
            exchange_times: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ExchangeStaminaScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ExchangeStaminaScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ExchangeStaminaScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExchangeStaminaScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aExchangeStaminaScRsp.proto\x1a\x0eItemCost.proto\"\xd5\x01\n\x14Ex\
    changeStaminaScRsp\x12/\n\x0eitem_cost_list\x18\x0b\x20\x03(\x0b2\t.Item\
    CostR\x0citemCostList\x12\x18\n\x07retcode\x18\x01\x20\x01(\rR\x07retcod\
    e\x12\x1f\n\x0bstamina_add\x18\x0f\x20\x01(\rR\nstaminaAdd\x12*\n\x11las\
    t_recover_time\x18\x05\x20\x01(\x03R\x0flastRecoverTime\x12%\n\x0eexchan\
    ge_times\x18\x0c\x20\x01(\rR\rexchangeTimesb\x06proto3\
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
            deps.push(super::ItemCost::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ExchangeStaminaScRsp::generated_message_descriptor_data());
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