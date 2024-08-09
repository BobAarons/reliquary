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

//! Generated file from `ChallengeSettleNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChallengeSettleNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChallengeSettleNotify {
    // message fields
    // @@protoc_insertion_point(field:ChallengeSettleNotify.reward)
    pub reward: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:ChallengeSettleNotify.stars)
    pub stars: u32,
    // @@protoc_insertion_point(field:ChallengeSettleNotify.score_two)
    pub score_two: u32,
    // @@protoc_insertion_point(field:ChallengeSettleNotify.challenge_id)
    pub challenge_id: u32,
    // @@protoc_insertion_point(field:ChallengeSettleNotify.is_win)
    pub is_win: bool,
    // @@protoc_insertion_point(field:ChallengeSettleNotify.challenge_score)
    pub challenge_score: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ChallengeSettleNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChallengeSettleNotify {
    fn default() -> &'a ChallengeSettleNotify {
        <ChallengeSettleNotify as ::protobuf::Message>::default_instance()
    }
}

impl ChallengeSettleNotify {
    pub fn new() -> ChallengeSettleNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "reward",
            |m: &ChallengeSettleNotify| { &m.reward },
            |m: &mut ChallengeSettleNotify| { &mut m.reward },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stars",
            |m: &ChallengeSettleNotify| { &m.stars },
            |m: &mut ChallengeSettleNotify| { &mut m.stars },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score_two",
            |m: &ChallengeSettleNotify| { &m.score_two },
            |m: &mut ChallengeSettleNotify| { &mut m.score_two },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "challenge_id",
            |m: &ChallengeSettleNotify| { &m.challenge_id },
            |m: &mut ChallengeSettleNotify| { &mut m.challenge_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_win",
            |m: &ChallengeSettleNotify| { &m.is_win },
            |m: &mut ChallengeSettleNotify| { &mut m.is_win },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "challenge_score",
            |m: &ChallengeSettleNotify| { &m.challenge_score },
            |m: &mut ChallengeSettleNotify| { &mut m.challenge_score },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChallengeSettleNotify>(
            "ChallengeSettleNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChallengeSettleNotify {
    const NAME: &'static str = "ChallengeSettleNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.reward)?;
                },
                64 => {
                    self.stars = is.read_uint32()?;
                },
                8 => {
                    self.score_two = is.read_uint32()?;
                },
                112 => {
                    self.challenge_id = is.read_uint32()?;
                },
                40 => {
                    self.is_win = is.read_bool()?;
                },
                80 => {
                    self.challenge_score = is.read_uint32()?;
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
        if let Some(v) = self.reward.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.stars != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.stars);
        }
        if self.score_two != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.score_two);
        }
        if self.challenge_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.challenge_id);
        }
        if self.is_win != false {
            my_size += 1 + 1;
        }
        if self.challenge_score != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.challenge_score);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.reward.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.stars != 0 {
            os.write_uint32(8, self.stars)?;
        }
        if self.score_two != 0 {
            os.write_uint32(1, self.score_two)?;
        }
        if self.challenge_id != 0 {
            os.write_uint32(14, self.challenge_id)?;
        }
        if self.is_win != false {
            os.write_bool(5, self.is_win)?;
        }
        if self.challenge_score != 0 {
            os.write_uint32(10, self.challenge_score)?;
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

    fn new() -> ChallengeSettleNotify {
        ChallengeSettleNotify::new()
    }

    fn clear(&mut self) {
        self.reward.clear();
        self.stars = 0;
        self.score_two = 0;
        self.challenge_id = 0;
        self.is_win = false;
        self.challenge_score = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChallengeSettleNotify {
        static instance: ChallengeSettleNotify = ChallengeSettleNotify {
            reward: ::protobuf::MessageField::none(),
            stars: 0,
            score_two: 0,
            challenge_id: 0,
            is_win: false,
            challenge_score: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChallengeSettleNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChallengeSettleNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChallengeSettleNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChallengeSettleNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bChallengeSettleNotify.proto\x1a\x0eItemList.proto\"\xd0\x01\n\x15C\
    hallengeSettleNotify\x12!\n\x06reward\x18\t\x20\x01(\x0b2\t.ItemListR\
    \x06reward\x12\x14\n\x05stars\x18\x08\x20\x01(\rR\x05stars\x12\x1b\n\tsc\
    ore_two\x18\x01\x20\x01(\rR\x08scoreTwo\x12!\n\x0cchallenge_id\x18\x0e\
    \x20\x01(\rR\x0bchallengeId\x12\x15\n\x06is_win\x18\x05\x20\x01(\x08R\
    \x05isWin\x12'\n\x0fchallenge_score\x18\n\x20\x01(\rR\x0echallengeScoreB\
    \x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChallengeSettleNotify::generated_message_descriptor_data());
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
