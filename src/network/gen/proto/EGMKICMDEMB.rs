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

//! Generated file from `EGMKICMDEMB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EGMKICMDEMB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EGMKICMDEMB {
    // message oneof groups
    pub NJACEEMDDKI: ::std::option::Option<egmkicmdemb::NJACEEMDDKI>,
    // special fields
    // @@protoc_insertion_point(special_field:EGMKICMDEMB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EGMKICMDEMB {
    fn default() -> &'a EGMKICMDEMB {
        <EGMKICMDEMB as ::protobuf::Message>::default_instance()
    }
}

impl EGMKICMDEMB {
    pub fn new() -> EGMKICMDEMB {
        ::std::default::Default::default()
    }

    // .ABOOHDMJFBF JBLGFIEHAAI = 12;

    pub fn JBLGFIEHAAI(&self) -> &super::ABOOHDMJFBF::ABOOHDMJFBF {
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::JBLGFIEHAAI(ref v)) => v,
            _ => <super::ABOOHDMJFBF::ABOOHDMJFBF as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_JBLGFIEHAAI(&mut self) {
        self.NJACEEMDDKI = ::std::option::Option::None;
    }

    pub fn has_JBLGFIEHAAI(&self) -> bool {
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::JBLGFIEHAAI(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JBLGFIEHAAI(&mut self, v: super::ABOOHDMJFBF::ABOOHDMJFBF) {
        self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::JBLGFIEHAAI(v))
    }

    // Mutable pointer to the field.
    pub fn mut_JBLGFIEHAAI(&mut self) -> &mut super::ABOOHDMJFBF::ABOOHDMJFBF {
        if let ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::JBLGFIEHAAI(_)) = self.NJACEEMDDKI {
        } else {
            self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::JBLGFIEHAAI(super::ABOOHDMJFBF::ABOOHDMJFBF::new()));
        }
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::JBLGFIEHAAI(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_JBLGFIEHAAI(&mut self) -> super::ABOOHDMJFBF::ABOOHDMJFBF {
        if self.has_JBLGFIEHAAI() {
            match self.NJACEEMDDKI.take() {
                ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::JBLGFIEHAAI(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ABOOHDMJFBF::ABOOHDMJFBF::new()
        }
    }

    // .FOLIEMPMMGI LEMBCBBIGFC = 14;

    pub fn LEMBCBBIGFC(&self) -> &super::FOLIEMPMMGI::FOLIEMPMMGI {
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::LEMBCBBIGFC(ref v)) => v,
            _ => <super::FOLIEMPMMGI::FOLIEMPMMGI as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LEMBCBBIGFC(&mut self) {
        self.NJACEEMDDKI = ::std::option::Option::None;
    }

    pub fn has_LEMBCBBIGFC(&self) -> bool {
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::LEMBCBBIGFC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LEMBCBBIGFC(&mut self, v: super::FOLIEMPMMGI::FOLIEMPMMGI) {
        self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::LEMBCBBIGFC(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LEMBCBBIGFC(&mut self) -> &mut super::FOLIEMPMMGI::FOLIEMPMMGI {
        if let ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::LEMBCBBIGFC(_)) = self.NJACEEMDDKI {
        } else {
            self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::LEMBCBBIGFC(super::FOLIEMPMMGI::FOLIEMPMMGI::new()));
        }
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::LEMBCBBIGFC(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LEMBCBBIGFC(&mut self) -> super::FOLIEMPMMGI::FOLIEMPMMGI {
        if self.has_LEMBCBBIGFC() {
            match self.NJACEEMDDKI.take() {
                ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::LEMBCBBIGFC(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FOLIEMPMMGI::FOLIEMPMMGI::new()
        }
    }

    // .PPBNJKAPJOJ ADIIKJNMLPH = 6;

    pub fn ADIIKJNMLPH(&self) -> &super::PPBNJKAPJOJ::PPBNJKAPJOJ {
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ADIIKJNMLPH(ref v)) => v,
            _ => <super::PPBNJKAPJOJ::PPBNJKAPJOJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_ADIIKJNMLPH(&mut self) {
        self.NJACEEMDDKI = ::std::option::Option::None;
    }

    pub fn has_ADIIKJNMLPH(&self) -> bool {
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ADIIKJNMLPH(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ADIIKJNMLPH(&mut self, v: super::PPBNJKAPJOJ::PPBNJKAPJOJ) {
        self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ADIIKJNMLPH(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ADIIKJNMLPH(&mut self) -> &mut super::PPBNJKAPJOJ::PPBNJKAPJOJ {
        if let ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ADIIKJNMLPH(_)) = self.NJACEEMDDKI {
        } else {
            self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ADIIKJNMLPH(super::PPBNJKAPJOJ::PPBNJKAPJOJ::new()));
        }
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ADIIKJNMLPH(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ADIIKJNMLPH(&mut self) -> super::PPBNJKAPJOJ::PPBNJKAPJOJ {
        if self.has_ADIIKJNMLPH() {
            match self.NJACEEMDDKI.take() {
                ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ADIIKJNMLPH(v)) => v,
                _ => panic!(),
            }
        } else {
            super::PPBNJKAPJOJ::PPBNJKAPJOJ::new()
        }
    }

    // .HCFPLCGOGEC GMCBLIEBGGG = 11;

    pub fn GMCBLIEBGGG(&self) -> &super::HCFPLCGOGEC::HCFPLCGOGEC {
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::GMCBLIEBGGG(ref v)) => v,
            _ => <super::HCFPLCGOGEC::HCFPLCGOGEC as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_GMCBLIEBGGG(&mut self) {
        self.NJACEEMDDKI = ::std::option::Option::None;
    }

    pub fn has_GMCBLIEBGGG(&self) -> bool {
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::GMCBLIEBGGG(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_GMCBLIEBGGG(&mut self, v: super::HCFPLCGOGEC::HCFPLCGOGEC) {
        self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::GMCBLIEBGGG(v))
    }

    // Mutable pointer to the field.
    pub fn mut_GMCBLIEBGGG(&mut self) -> &mut super::HCFPLCGOGEC::HCFPLCGOGEC {
        if let ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::GMCBLIEBGGG(_)) = self.NJACEEMDDKI {
        } else {
            self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::GMCBLIEBGGG(super::HCFPLCGOGEC::HCFPLCGOGEC::new()));
        }
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::GMCBLIEBGGG(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_GMCBLIEBGGG(&mut self) -> super::HCFPLCGOGEC::HCFPLCGOGEC {
        if self.has_GMCBLIEBGGG() {
            match self.NJACEEMDDKI.take() {
                ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::GMCBLIEBGGG(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HCFPLCGOGEC::HCFPLCGOGEC::new()
        }
    }

    // .IBMKIMJPOJP IGGNGJGPCLK = 5;

    pub fn IGGNGJGPCLK(&self) -> &super::IBMKIMJPOJP::IBMKIMJPOJP {
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::IGGNGJGPCLK(ref v)) => v,
            _ => <super::IBMKIMJPOJP::IBMKIMJPOJP as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_IGGNGJGPCLK(&mut self) {
        self.NJACEEMDDKI = ::std::option::Option::None;
    }

    pub fn has_IGGNGJGPCLK(&self) -> bool {
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::IGGNGJGPCLK(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_IGGNGJGPCLK(&mut self, v: super::IBMKIMJPOJP::IBMKIMJPOJP) {
        self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::IGGNGJGPCLK(v))
    }

    // Mutable pointer to the field.
    pub fn mut_IGGNGJGPCLK(&mut self) -> &mut super::IBMKIMJPOJP::IBMKIMJPOJP {
        if let ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::IGGNGJGPCLK(_)) = self.NJACEEMDDKI {
        } else {
            self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::IGGNGJGPCLK(super::IBMKIMJPOJP::IBMKIMJPOJP::new()));
        }
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::IGGNGJGPCLK(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_IGGNGJGPCLK(&mut self) -> super::IBMKIMJPOJP::IBMKIMJPOJP {
        if self.has_IGGNGJGPCLK() {
            match self.NJACEEMDDKI.take() {
                ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::IGGNGJGPCLK(v)) => v,
                _ => panic!(),
            }
        } else {
            super::IBMKIMJPOJP::IBMKIMJPOJP::new()
        }
    }

    // .JLNEFOKFNAP ALNPLJOHLGB = 13;

    pub fn ALNPLJOHLGB(&self) -> &super::JLNEFOKFNAP::JLNEFOKFNAP {
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ALNPLJOHLGB(ref v)) => v,
            _ => <super::JLNEFOKFNAP::JLNEFOKFNAP as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_ALNPLJOHLGB(&mut self) {
        self.NJACEEMDDKI = ::std::option::Option::None;
    }

    pub fn has_ALNPLJOHLGB(&self) -> bool {
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ALNPLJOHLGB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ALNPLJOHLGB(&mut self, v: super::JLNEFOKFNAP::JLNEFOKFNAP) {
        self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ALNPLJOHLGB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ALNPLJOHLGB(&mut self) -> &mut super::JLNEFOKFNAP::JLNEFOKFNAP {
        if let ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ALNPLJOHLGB(_)) = self.NJACEEMDDKI {
        } else {
            self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ALNPLJOHLGB(super::JLNEFOKFNAP::JLNEFOKFNAP::new()));
        }
        match self.NJACEEMDDKI {
            ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ALNPLJOHLGB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ALNPLJOHLGB(&mut self) -> super::JLNEFOKFNAP::JLNEFOKFNAP {
        if self.has_ALNPLJOHLGB() {
            match self.NJACEEMDDKI.take() {
                ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ALNPLJOHLGB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JLNEFOKFNAP::JLNEFOKFNAP::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ABOOHDMJFBF::ABOOHDMJFBF>(
            "JBLGFIEHAAI",
            EGMKICMDEMB::has_JBLGFIEHAAI,
            EGMKICMDEMB::JBLGFIEHAAI,
            EGMKICMDEMB::mut_JBLGFIEHAAI,
            EGMKICMDEMB::set_JBLGFIEHAAI,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FOLIEMPMMGI::FOLIEMPMMGI>(
            "LEMBCBBIGFC",
            EGMKICMDEMB::has_LEMBCBBIGFC,
            EGMKICMDEMB::LEMBCBBIGFC,
            EGMKICMDEMB::mut_LEMBCBBIGFC,
            EGMKICMDEMB::set_LEMBCBBIGFC,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::PPBNJKAPJOJ::PPBNJKAPJOJ>(
            "ADIIKJNMLPH",
            EGMKICMDEMB::has_ADIIKJNMLPH,
            EGMKICMDEMB::ADIIKJNMLPH,
            EGMKICMDEMB::mut_ADIIKJNMLPH,
            EGMKICMDEMB::set_ADIIKJNMLPH,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HCFPLCGOGEC::HCFPLCGOGEC>(
            "GMCBLIEBGGG",
            EGMKICMDEMB::has_GMCBLIEBGGG,
            EGMKICMDEMB::GMCBLIEBGGG,
            EGMKICMDEMB::mut_GMCBLIEBGGG,
            EGMKICMDEMB::set_GMCBLIEBGGG,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::IBMKIMJPOJP::IBMKIMJPOJP>(
            "IGGNGJGPCLK",
            EGMKICMDEMB::has_IGGNGJGPCLK,
            EGMKICMDEMB::IGGNGJGPCLK,
            EGMKICMDEMB::mut_IGGNGJGPCLK,
            EGMKICMDEMB::set_IGGNGJGPCLK,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JLNEFOKFNAP::JLNEFOKFNAP>(
            "ALNPLJOHLGB",
            EGMKICMDEMB::has_ALNPLJOHLGB,
            EGMKICMDEMB::ALNPLJOHLGB,
            EGMKICMDEMB::mut_ALNPLJOHLGB,
            EGMKICMDEMB::set_ALNPLJOHLGB,
        ));
        oneofs.push(egmkicmdemb::NJACEEMDDKI::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EGMKICMDEMB>(
            "EGMKICMDEMB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EGMKICMDEMB {
    const NAME: &'static str = "EGMKICMDEMB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::JBLGFIEHAAI(is.read_message()?));
                },
                114 => {
                    self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::LEMBCBBIGFC(is.read_message()?));
                },
                50 => {
                    self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ADIIKJNMLPH(is.read_message()?));
                },
                90 => {
                    self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::GMCBLIEBGGG(is.read_message()?));
                },
                42 => {
                    self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::IGGNGJGPCLK(is.read_message()?));
                },
                106 => {
                    self.NJACEEMDDKI = ::std::option::Option::Some(egmkicmdemb::NJACEEMDDKI::ALNPLJOHLGB(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.NJACEEMDDKI {
            match v {
                &egmkicmdemb::NJACEEMDDKI::JBLGFIEHAAI(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &egmkicmdemb::NJACEEMDDKI::LEMBCBBIGFC(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &egmkicmdemb::NJACEEMDDKI::ADIIKJNMLPH(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &egmkicmdemb::NJACEEMDDKI::GMCBLIEBGGG(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &egmkicmdemb::NJACEEMDDKI::IGGNGJGPCLK(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &egmkicmdemb::NJACEEMDDKI::ALNPLJOHLGB(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.NJACEEMDDKI {
            match v {
                &egmkicmdemb::NJACEEMDDKI::JBLGFIEHAAI(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
                },
                &egmkicmdemb::NJACEEMDDKI::LEMBCBBIGFC(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
                },
                &egmkicmdemb::NJACEEMDDKI::ADIIKJNMLPH(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
                },
                &egmkicmdemb::NJACEEMDDKI::GMCBLIEBGGG(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
                },
                &egmkicmdemb::NJACEEMDDKI::IGGNGJGPCLK(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
                },
                &egmkicmdemb::NJACEEMDDKI::ALNPLJOHLGB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
                },
            };
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

    fn new() -> EGMKICMDEMB {
        EGMKICMDEMB::new()
    }

    fn clear(&mut self) {
        self.NJACEEMDDKI = ::std::option::Option::None;
        self.NJACEEMDDKI = ::std::option::Option::None;
        self.NJACEEMDDKI = ::std::option::Option::None;
        self.NJACEEMDDKI = ::std::option::Option::None;
        self.NJACEEMDDKI = ::std::option::Option::None;
        self.NJACEEMDDKI = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EGMKICMDEMB {
        static instance: EGMKICMDEMB = EGMKICMDEMB {
            NJACEEMDDKI: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EGMKICMDEMB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EGMKICMDEMB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EGMKICMDEMB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EGMKICMDEMB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `EGMKICMDEMB`
pub mod egmkicmdemb {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:EGMKICMDEMB.NJACEEMDDKI)
    pub enum NJACEEMDDKI {
        // @@protoc_insertion_point(oneof_field:EGMKICMDEMB.JBLGFIEHAAI)
        JBLGFIEHAAI(super::super::ABOOHDMJFBF::ABOOHDMJFBF),
        // @@protoc_insertion_point(oneof_field:EGMKICMDEMB.LEMBCBBIGFC)
        LEMBCBBIGFC(super::super::FOLIEMPMMGI::FOLIEMPMMGI),
        // @@protoc_insertion_point(oneof_field:EGMKICMDEMB.ADIIKJNMLPH)
        ADIIKJNMLPH(super::super::PPBNJKAPJOJ::PPBNJKAPJOJ),
        // @@protoc_insertion_point(oneof_field:EGMKICMDEMB.GMCBLIEBGGG)
        GMCBLIEBGGG(super::super::HCFPLCGOGEC::HCFPLCGOGEC),
        // @@protoc_insertion_point(oneof_field:EGMKICMDEMB.IGGNGJGPCLK)
        IGGNGJGPCLK(super::super::IBMKIMJPOJP::IBMKIMJPOJP),
        // @@protoc_insertion_point(oneof_field:EGMKICMDEMB.ALNPLJOHLGB)
        ALNPLJOHLGB(super::super::JLNEFOKFNAP::JLNEFOKFNAP),
    }

    impl ::protobuf::Oneof for NJACEEMDDKI {
    }

    impl ::protobuf::OneofFull for NJACEEMDDKI {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::EGMKICMDEMB as ::protobuf::MessageFull>::descriptor().oneof_by_name("NJACEEMDDKI").unwrap()).clone()
        }
    }

    impl NJACEEMDDKI {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<NJACEEMDDKI>("NJACEEMDDKI")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EGMKICMDEMB.proto\x1a\x11ABOOHDMJFBF.proto\x1a\x11FOLIEMPMMGI.prot\
    o\x1a\x11HCFPLCGOGEC.proto\x1a\x11IBMKIMJPOJP.proto\x1a\x11JLNEFOKFNAP.p\
    roto\x1a\x11PPBNJKAPJOJ.proto\"\xc8\x02\n\x0bEGMKICMDEMB\x120\n\x0bJBLGF\
    IEHAAI\x18\x0c\x20\x01(\x0b2\x0c.ABOOHDMJFBFH\0R\x0bJBLGFIEHAAI\x120\n\
    \x0bLEMBCBBIGFC\x18\x0e\x20\x01(\x0b2\x0c.FOLIEMPMMGIH\0R\x0bLEMBCBBIGFC\
    \x120\n\x0bADIIKJNMLPH\x18\x06\x20\x01(\x0b2\x0c.PPBNJKAPJOJH\0R\x0bADII\
    KJNMLPH\x120\n\x0bGMCBLIEBGGG\x18\x0b\x20\x01(\x0b2\x0c.HCFPLCGOGECH\0R\
    \x0bGMCBLIEBGGG\x120\n\x0bIGGNGJGPCLK\x18\x05\x20\x01(\x0b2\x0c.IBMKIMJP\
    OJPH\0R\x0bIGGNGJGPCLK\x120\n\x0bALNPLJOHLGB\x18\r\x20\x01(\x0b2\x0c.JLN\
    EFOKFNAPH\0R\x0bALNPLJOHLGBB\r\n\x0bNJACEEMDDKIb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(6);
            deps.push(super::ABOOHDMJFBF::file_descriptor().clone());
            deps.push(super::FOLIEMPMMGI::file_descriptor().clone());
            deps.push(super::HCFPLCGOGEC::file_descriptor().clone());
            deps.push(super::IBMKIMJPOJP::file_descriptor().clone());
            deps.push(super::JLNEFOKFNAP::file_descriptor().clone());
            deps.push(super::PPBNJKAPJOJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EGMKICMDEMB::generated_message_descriptor_data());
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
