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

//! Generated file from `GNIJNJBLDIG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GNIJNJBLDIG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GNIJNJBLDIG {
    // message fields
    // @@protoc_insertion_point(field:GNIJNJBLDIG.EABCMNNHGMB)
    pub EABCMNNHGMB: ::std::vec::Vec<super::FGELIIINHIM::FGELIIINHIM>,
    // @@protoc_insertion_point(field:GNIJNJBLDIG.CIDHIIIBIGJ)
    pub CIDHIIIBIGJ: ::std::vec::Vec<super::LDLAJMOPBID::LDLAJMOPBID>,
    // @@protoc_insertion_point(field:GNIJNJBLDIG.OGHBIDJJCOD)
    pub OGHBIDJJCOD: ::std::vec::Vec<super::MFKMIANNCCN::MFKMIANNCCN>,
    // @@protoc_insertion_point(field:GNIJNJBLDIG.FCMMICJHJBI)
    pub FCMMICJHJBI: ::protobuf::MessageField<super::KHEENPJKCHO::KHEENPJKCHO>,
    // @@protoc_insertion_point(field:GNIJNJBLDIG.MINKJBADONI)
    pub MINKJBADONI: ::protobuf::MessageField<super::BMIEADMKCKL::BMIEADMKCKL>,
    // @@protoc_insertion_point(field:GNIJNJBLDIG.KAAPHKEBKAO)
    pub KAAPHKEBKAO: ::protobuf::MessageField<super::AENCACEEHHK::AENCACEEHHK>,
    // @@protoc_insertion_point(field:GNIJNJBLDIG.POAGFFCEHGI)
    pub POAGFFCEHGI: ::protobuf::MessageField<super::ECPGDNMOLKI::ECPGDNMOLKI>,
    // @@protoc_insertion_point(field:GNIJNJBLDIG.LNGIOKAPBOE)
    pub LNGIOKAPBOE: ::protobuf::MessageField<super::BMOHOKEGNIM::BMOHOKEGNIM>,
    // special fields
    // @@protoc_insertion_point(special_field:GNIJNJBLDIG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GNIJNJBLDIG {
    fn default() -> &'a GNIJNJBLDIG {
        <GNIJNJBLDIG as ::protobuf::Message>::default_instance()
    }
}

impl GNIJNJBLDIG {
    pub fn new() -> GNIJNJBLDIG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EABCMNNHGMB",
            |m: &GNIJNJBLDIG| { &m.EABCMNNHGMB },
            |m: &mut GNIJNJBLDIG| { &mut m.EABCMNNHGMB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CIDHIIIBIGJ",
            |m: &GNIJNJBLDIG| { &m.CIDHIIIBIGJ },
            |m: &mut GNIJNJBLDIG| { &mut m.CIDHIIIBIGJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OGHBIDJJCOD",
            |m: &GNIJNJBLDIG| { &m.OGHBIDJJCOD },
            |m: &mut GNIJNJBLDIG| { &mut m.OGHBIDJJCOD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KHEENPJKCHO::KHEENPJKCHO>(
            "FCMMICJHJBI",
            |m: &GNIJNJBLDIG| { &m.FCMMICJHJBI },
            |m: &mut GNIJNJBLDIG| { &mut m.FCMMICJHJBI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BMIEADMKCKL::BMIEADMKCKL>(
            "MINKJBADONI",
            |m: &GNIJNJBLDIG| { &m.MINKJBADONI },
            |m: &mut GNIJNJBLDIG| { &mut m.MINKJBADONI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AENCACEEHHK::AENCACEEHHK>(
            "KAAPHKEBKAO",
            |m: &GNIJNJBLDIG| { &m.KAAPHKEBKAO },
            |m: &mut GNIJNJBLDIG| { &mut m.KAAPHKEBKAO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ECPGDNMOLKI::ECPGDNMOLKI>(
            "POAGFFCEHGI",
            |m: &GNIJNJBLDIG| { &m.POAGFFCEHGI },
            |m: &mut GNIJNJBLDIG| { &mut m.POAGFFCEHGI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BMOHOKEGNIM::BMOHOKEGNIM>(
            "LNGIOKAPBOE",
            |m: &GNIJNJBLDIG| { &m.LNGIOKAPBOE },
            |m: &mut GNIJNJBLDIG| { &mut m.LNGIOKAPBOE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GNIJNJBLDIG>(
            "GNIJNJBLDIG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GNIJNJBLDIG {
    const NAME: &'static str = "GNIJNJBLDIG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    self.EABCMNNHGMB.push(is.read_message()?);
                },
                98 => {
                    self.CIDHIIIBIGJ.push(is.read_message()?);
                },
                66 => {
                    self.OGHBIDJJCOD.push(is.read_message()?);
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FCMMICJHJBI)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MINKJBADONI)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KAAPHKEBKAO)?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.POAGFFCEHGI)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LNGIOKAPBOE)?;
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
        for value in &self.EABCMNNHGMB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.CIDHIIIBIGJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.OGHBIDJJCOD {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.FCMMICJHJBI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.MINKJBADONI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.KAAPHKEBKAO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.POAGFFCEHGI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.LNGIOKAPBOE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.EABCMNNHGMB {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        for v in &self.CIDHIIIBIGJ {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        for v in &self.OGHBIDJJCOD {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if let Some(v) = self.FCMMICJHJBI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.MINKJBADONI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.KAAPHKEBKAO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.POAGFFCEHGI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if let Some(v) = self.LNGIOKAPBOE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
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

    fn new() -> GNIJNJBLDIG {
        GNIJNJBLDIG::new()
    }

    fn clear(&mut self) {
        self.EABCMNNHGMB.clear();
        self.CIDHIIIBIGJ.clear();
        self.OGHBIDJJCOD.clear();
        self.FCMMICJHJBI.clear();
        self.MINKJBADONI.clear();
        self.KAAPHKEBKAO.clear();
        self.POAGFFCEHGI.clear();
        self.LNGIOKAPBOE.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GNIJNJBLDIG {
        static instance: GNIJNJBLDIG = GNIJNJBLDIG {
            EABCMNNHGMB: ::std::vec::Vec::new(),
            CIDHIIIBIGJ: ::std::vec::Vec::new(),
            OGHBIDJJCOD: ::std::vec::Vec::new(),
            FCMMICJHJBI: ::protobuf::MessageField::none(),
            MINKJBADONI: ::protobuf::MessageField::none(),
            KAAPHKEBKAO: ::protobuf::MessageField::none(),
            POAGFFCEHGI: ::protobuf::MessageField::none(),
            LNGIOKAPBOE: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GNIJNJBLDIG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GNIJNJBLDIG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GNIJNJBLDIG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GNIJNJBLDIG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GNIJNJBLDIG.proto\x1a\x11AENCACEEHHK.proto\x1a\x11BMIEADMKCKL.prot\
    o\x1a\x11BMOHOKEGNIM.proto\x1a\x11ECPGDNMOLKI.proto\x1a\x11FGELIIINHIM.p\
    roto\x1a\x11KHEENPJKCHO.proto\x1a\x11LDLAJMOPBID.proto\x1a\x11MFKMIANNCC\
    N.proto\"\x8d\x03\n\x0bGNIJNJBLDIG\x12.\n\x0bEABCMNNHGMB\x18\x03\x20\x03\
    (\x0b2\x0c.FGELIIINHIMR\x0bEABCMNNHGMB\x12.\n\x0bCIDHIIIBIGJ\x18\x0c\x20\
    \x03(\x0b2\x0c.LDLAJMOPBIDR\x0bCIDHIIIBIGJ\x12.\n\x0bOGHBIDJJCOD\x18\x08\
    \x20\x03(\x0b2\x0c.MFKMIANNCCNR\x0bOGHBIDJJCOD\x12.\n\x0bFCMMICJHJBI\x18\
    \x04\x20\x01(\x0b2\x0c.KHEENPJKCHOR\x0bFCMMICJHJBI\x12.\n\x0bMINKJBADONI\
    \x18\x01\x20\x01(\x0b2\x0c.BMIEADMKCKLR\x0bMINKJBADONI\x12.\n\x0bKAAPHKE\
    BKAO\x18\x02\x20\x01(\x0b2\x0c.AENCACEEHHKR\x0bKAAPHKEBKAO\x12.\n\x0bPOA\
    GFFCEHGI\x18\x06\x20\x01(\x0b2\x0c.ECPGDNMOLKIR\x0bPOAGFFCEHGI\x12.\n\
    \x0bLNGIOKAPBOE\x18\x0e\x20\x01(\x0b2\x0c.BMOHOKEGNIMR\x0bLNGIOKAPBOEb\
    \x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(8);
            deps.push(super::AENCACEEHHK::file_descriptor().clone());
            deps.push(super::BMIEADMKCKL::file_descriptor().clone());
            deps.push(super::BMOHOKEGNIM::file_descriptor().clone());
            deps.push(super::ECPGDNMOLKI::file_descriptor().clone());
            deps.push(super::FGELIIINHIM::file_descriptor().clone());
            deps.push(super::KHEENPJKCHO::file_descriptor().clone());
            deps.push(super::LDLAJMOPBID::file_descriptor().clone());
            deps.push(super::MFKMIANNCCN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GNIJNJBLDIG::generated_message_descriptor_data());
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