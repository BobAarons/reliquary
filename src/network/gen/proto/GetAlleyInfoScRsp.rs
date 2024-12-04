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

//! Generated file from `GetAlleyInfoScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetAlleyInfoScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetAlleyInfoScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.LHCHBPNPHFL)
    pub LHCHBPNPHFL: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.HFKCMKJNNCJ)
    pub HFKCMKJNNCJ: ::protobuf::MessageField<super::LFIEJOHPDHL::LFIEJOHPDHL>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.CPAMDPLGPHM)
    pub CPAMDPLGPHM: ::protobuf::MessageField<super::PKHLAAOBOEK::PKHLAAOBOEK>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.MPLABMDJANB)
    pub MPLABMDJANB: ::std::vec::Vec<super::DCMCLLPDDJO::DCMCLLPDDJO>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.IEGMEPKCFDP)
    pub IEGMEPKCFDP: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.HBECMCGOBGD)
    pub HBECMCGOBGD: u32,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.CAPNJOIANHJ)
    pub CAPNJOIANHJ: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.MCMEMPMLBMP)
    pub MCMEMPMLBMP: ::protobuf::MessageField<super::BPIMHAOAKFL::BPIMHAOAKFL>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.MLFLLKMIADE)
    pub MLFLLKMIADE: u32,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.HJBGPDLKCJN)
    pub HJBGPDLKCJN: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetAlleyInfoScRsp.JKOCJIMAGBN)
    pub JKOCJIMAGBN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetAlleyInfoScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetAlleyInfoScRsp {
    fn default() -> &'a GetAlleyInfoScRsp {
        <GetAlleyInfoScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetAlleyInfoScRsp {
    pub fn new() -> GetAlleyInfoScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(12);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LHCHBPNPHFL",
            |m: &GetAlleyInfoScRsp| { &m.LHCHBPNPHFL },
            |m: &mut GetAlleyInfoScRsp| { &mut m.LHCHBPNPHFL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LFIEJOHPDHL::LFIEJOHPDHL>(
            "HFKCMKJNNCJ",
            |m: &GetAlleyInfoScRsp| { &m.HFKCMKJNNCJ },
            |m: &mut GetAlleyInfoScRsp| { &mut m.HFKCMKJNNCJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PKHLAAOBOEK::PKHLAAOBOEK>(
            "CPAMDPLGPHM",
            |m: &GetAlleyInfoScRsp| { &m.CPAMDPLGPHM },
            |m: &mut GetAlleyInfoScRsp| { &mut m.CPAMDPLGPHM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MPLABMDJANB",
            |m: &GetAlleyInfoScRsp| { &m.MPLABMDJANB },
            |m: &mut GetAlleyInfoScRsp| { &mut m.MPLABMDJANB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "IEGMEPKCFDP",
            |m: &GetAlleyInfoScRsp| { &m.IEGMEPKCFDP },
            |m: &mut GetAlleyInfoScRsp| { &mut m.IEGMEPKCFDP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HBECMCGOBGD",
            |m: &GetAlleyInfoScRsp| { &m.HBECMCGOBGD },
            |m: &mut GetAlleyInfoScRsp| { &mut m.HBECMCGOBGD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &GetAlleyInfoScRsp| { &m.ADADHIHDHJC },
            |m: &mut GetAlleyInfoScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "CAPNJOIANHJ",
            |m: &GetAlleyInfoScRsp| { &m.CAPNJOIANHJ },
            |m: &mut GetAlleyInfoScRsp| { &mut m.CAPNJOIANHJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BPIMHAOAKFL::BPIMHAOAKFL>(
            "MCMEMPMLBMP",
            |m: &GetAlleyInfoScRsp| { &m.MCMEMPMLBMP },
            |m: &mut GetAlleyInfoScRsp| { &mut m.MCMEMPMLBMP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MLFLLKMIADE",
            |m: &GetAlleyInfoScRsp| { &m.MLFLLKMIADE },
            |m: &mut GetAlleyInfoScRsp| { &mut m.MLFLLKMIADE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HJBGPDLKCJN",
            |m: &GetAlleyInfoScRsp| { &m.HJBGPDLKCJN },
            |m: &mut GetAlleyInfoScRsp| { &mut m.HJBGPDLKCJN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JKOCJIMAGBN",
            |m: &GetAlleyInfoScRsp| { &m.JKOCJIMAGBN },
            |m: &mut GetAlleyInfoScRsp| { &mut m.JKOCJIMAGBN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetAlleyInfoScRsp>(
            "GetAlleyInfoScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetAlleyInfoScRsp {
    const NAME: &'static str = "GetAlleyInfoScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.LHCHBPNPHFL)?;
                },
                24 => {
                    self.LHCHBPNPHFL.push(is.read_uint32()?);
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HFKCMKJNNCJ)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CPAMDPLGPHM)?;
                },
                98 => {
                    self.MPLABMDJANB.push(is.read_message()?);
                },
                50 => {
                    is.read_repeated_packed_uint32_into(&mut self.IEGMEPKCFDP)?;
                },
                48 => {
                    self.IEGMEPKCFDP.push(is.read_uint32()?);
                },
                56 => {
                    self.HBECMCGOBGD = is.read_uint32()?;
                },
                72 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                122 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.CAPNJOIANHJ.insert(key, value);
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MCMEMPMLBMP)?;
                },
                104 => {
                    self.MLFLLKMIADE = is.read_uint32()?;
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.HJBGPDLKCJN)?;
                },
                32 => {
                    self.HJBGPDLKCJN.push(is.read_uint32()?);
                },
                64 => {
                    self.JKOCJIMAGBN = is.read_uint32()?;
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
        for value in &self.LHCHBPNPHFL {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        if let Some(v) = self.HFKCMKJNNCJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.CPAMDPLGPHM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.MPLABMDJANB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.IEGMEPKCFDP {
            my_size += ::protobuf::rt::uint32_size(6, *value);
        };
        if self.HBECMCGOBGD != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.HBECMCGOBGD);
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.ADADHIHDHJC);
        }
        for (k, v) in &self.CAPNJOIANHJ {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if let Some(v) = self.MCMEMPMLBMP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.MLFLLKMIADE != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.MLFLLKMIADE);
        }
        for value in &self.HJBGPDLKCJN {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        if self.JKOCJIMAGBN != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.JKOCJIMAGBN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.LHCHBPNPHFL {
            os.write_uint32(3, *v)?;
        };
        if let Some(v) = self.HFKCMKJNNCJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if let Some(v) = self.CPAMDPLGPHM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        for v in &self.MPLABMDJANB {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        for v in &self.IEGMEPKCFDP {
            os.write_uint32(6, *v)?;
        };
        if self.HBECMCGOBGD != 0 {
            os.write_uint32(7, self.HBECMCGOBGD)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(9, self.ADADHIHDHJC)?;
        }
        for (k, v) in &self.CAPNJOIANHJ {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(122)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if let Some(v) = self.MCMEMPMLBMP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.MLFLLKMIADE != 0 {
            os.write_uint32(13, self.MLFLLKMIADE)?;
        }
        for v in &self.HJBGPDLKCJN {
            os.write_uint32(4, *v)?;
        };
        if self.JKOCJIMAGBN != 0 {
            os.write_uint32(8, self.JKOCJIMAGBN)?;
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

    fn new() -> GetAlleyInfoScRsp {
        GetAlleyInfoScRsp::new()
    }

    fn clear(&mut self) {
        self.LHCHBPNPHFL.clear();
        self.HFKCMKJNNCJ.clear();
        self.CPAMDPLGPHM.clear();
        self.MPLABMDJANB.clear();
        self.IEGMEPKCFDP.clear();
        self.HBECMCGOBGD = 0;
        self.ADADHIHDHJC = 0;
        self.CAPNJOIANHJ.clear();
        self.MCMEMPMLBMP.clear();
        self.MLFLLKMIADE = 0;
        self.HJBGPDLKCJN.clear();
        self.JKOCJIMAGBN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetAlleyInfoScRsp {
        static instance: ::protobuf::rt::Lazy<GetAlleyInfoScRsp> = ::protobuf::rt::Lazy::new();
        instance.get(GetAlleyInfoScRsp::new)
    }
}

impl ::protobuf::MessageFull for GetAlleyInfoScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetAlleyInfoScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetAlleyInfoScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetAlleyInfoScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17GetAlleyInfoScRsp.proto\x1a\x11BPIMHAOAKFL.proto\x1a\x11DCMCLLPDDJ\
    O.proto\x1a\x11LFIEJOHPDHL.proto\x1a\x11PKHLAAOBOEK.proto\"\xc8\x04\n\
    \x11GetAlleyInfoScRsp\x12\x20\n\x0bLHCHBPNPHFL\x18\x03\x20\x03(\rR\x0bLH\
    CHBPNPHFL\x12.\n\x0bHFKCMKJNNCJ\x18\x05\x20\x01(\x0b2\x0c.LFIEJOHPDHLR\
    \x0bHFKCMKJNNCJ\x12.\n\x0bCPAMDPLGPHM\x18\x0e\x20\x01(\x0b2\x0c.PKHLAAOB\
    OEKR\x0bCPAMDPLGPHM\x12.\n\x0bMPLABMDJANB\x18\x0c\x20\x03(\x0b2\x0c.DCMC\
    LLPDDJOR\x0bMPLABMDJANB\x12\x20\n\x0bIEGMEPKCFDP\x18\x06\x20\x03(\rR\x0b\
    IEGMEPKCFDP\x12\x20\n\x0bHBECMCGOBGD\x18\x07\x20\x01(\rR\x0bHBECMCGOBGD\
    \x12\x20\n\x0bADADHIHDHJC\x18\t\x20\x01(\rR\x0bADADHIHDHJC\x12E\n\x0bCAP\
    NJOIANHJ\x18\x0f\x20\x03(\x0b2#.GetAlleyInfoScRsp.CAPNJOIANHJEntryR\x0bC\
    APNJOIANHJ\x12.\n\x0bMCMEMPMLBMP\x18\x0b\x20\x01(\x0b2\x0c.BPIMHAOAKFLR\
    \x0bMCMEMPMLBMP\x12\x20\n\x0bMLFLLKMIADE\x18\r\x20\x01(\rR\x0bMLFLLKMIAD\
    E\x12\x20\n\x0bHJBGPDLKCJN\x18\x04\x20\x03(\rR\x0bHJBGPDLKCJN\x12\x20\n\
    \x0bJKOCJIMAGBN\x18\x08\x20\x01(\rR\x0bJKOCJIMAGBN\x1a>\n\x10CAPNJOIANHJ\
    Entry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\
    \x02\x20\x01(\rR\x05value:\x028\x01b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::BPIMHAOAKFL::file_descriptor().clone());
            deps.push(super::DCMCLLPDDJO::file_descriptor().clone());
            deps.push(super::LFIEJOHPDHL::file_descriptor().clone());
            deps.push(super::PKHLAAOBOEK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetAlleyInfoScRsp::generated_message_descriptor_data());
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