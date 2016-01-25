// Copyright 2015-2016 Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// THIS IS A GENERATED FILE. PLEASE DO NOT EDIT.
// Instead, please regenerate using generate-encoding-data.py

//! This module provides enums that wrap the various decoders and encoders.
//! The purpose is to make `Decoder` and `Encoder` `Sized` by writing the
//! dispatch explicitly for a finite set of specialized decoders and encoders.
//! Unfortunately, this means the compiler doesn't generate the dispatch code
//! and it has to be written here instead.
//!
//! The purpose of making `Decoder` and `Encoder` `Sized` is to allow stack
//! allocation in Rust code, including the convenience methods on `Encoding`.

use single_byte::*;
use utf_8::*;
use gb18030::*;
use big5::*;
use euc_jp::*;
use iso_2022_jp::*;
use shift_jis::*;
use euc_kr::*;
use replacement::*;
use x_user_defined::*;
use utf_16::*;
use super::*;

pub enum VariantDecoder {
    SingleByte(SingleByteDecoder),
    Utf8(Utf8Decoder),
    Gb18030(Gb18030Decoder),
    Big5(Big5Decoder),
    EucJp(EucJpDecoder),
    Iso2022Jp(Iso2022JpDecoder),
    ShiftJis(ShiftJisDecoder),
    EucKr(EucKrDecoder),
    Replacement(ReplacementDecoder),
    UserDefined(UserDefinedDecoder),
    Utf16(Utf16Decoder),
}

impl VariantDecoder {
    pub fn reset(&mut self) {
        match self {
            &mut VariantDecoder::SingleByte(ref mut v) => (),
            &mut VariantDecoder::Utf8(ref mut v) => v.reset(),
            &mut VariantDecoder::Gb18030(ref mut v) => v.reset(),
            &mut VariantDecoder::Big5(ref mut v) => v.reset(),
            &mut VariantDecoder::EucJp(ref mut v) => v.reset(),
            &mut VariantDecoder::Iso2022Jp(ref mut v) => v.reset(),
            &mut VariantDecoder::ShiftJis(ref mut v) => v.reset(),
            &mut VariantDecoder::EucKr(ref mut v) => v.reset(),
            &mut VariantDecoder::Replacement(ref mut v) => v.reset(),
            &mut VariantDecoder::UserDefined(ref mut v) => v.reset(),
            &mut VariantDecoder::Utf16(ref mut v) => v.reset(),
        }
    }

    pub fn max_utf16_buffer_length(&self, byte_length: usize) -> usize {
        match self {
            &VariantDecoder::SingleByte(ref v) => v.max_utf16_buffer_length(byte_length),
            &VariantDecoder::Utf8(ref v) => v.max_utf16_buffer_length(byte_length),
            &VariantDecoder::Gb18030(ref v) => v.max_utf16_buffer_length(byte_length),
            &VariantDecoder::Big5(ref v) => v.max_utf16_buffer_length(byte_length),
            &VariantDecoder::EucJp(ref v) => v.max_utf16_buffer_length(byte_length),
            &VariantDecoder::Iso2022Jp(ref v) => v.max_utf16_buffer_length(byte_length),
            &VariantDecoder::ShiftJis(ref v) => v.max_utf16_buffer_length(byte_length),
            &VariantDecoder::EucKr(ref v) => v.max_utf16_buffer_length(byte_length),
            &VariantDecoder::Replacement(ref v) => v.max_utf16_buffer_length(byte_length),
            &VariantDecoder::UserDefined(ref v) => v.max_utf16_buffer_length(byte_length),
            &VariantDecoder::Utf16(ref v) => v.max_utf16_buffer_length(byte_length),
        }
    }

    pub fn max_utf8_buffer_length(&self, byte_length: usize) -> usize {
        match self {
            &VariantDecoder::SingleByte(ref v) => v.max_utf8_buffer_length(byte_length),
            &VariantDecoder::Utf8(ref v) => v.max_utf8_buffer_length(byte_length),
            &VariantDecoder::Gb18030(ref v) => v.max_utf8_buffer_length(byte_length),
            &VariantDecoder::Big5(ref v) => v.max_utf8_buffer_length(byte_length),
            &VariantDecoder::EucJp(ref v) => v.max_utf8_buffer_length(byte_length),
            &VariantDecoder::Iso2022Jp(ref v) => v.max_utf8_buffer_length(byte_length),
            &VariantDecoder::ShiftJis(ref v) => v.max_utf8_buffer_length(byte_length),
            &VariantDecoder::EucKr(ref v) => v.max_utf8_buffer_length(byte_length),
            &VariantDecoder::Replacement(ref v) => v.max_utf8_buffer_length(byte_length),
            &VariantDecoder::UserDefined(ref v) => v.max_utf8_buffer_length(byte_length),
            &VariantDecoder::Utf16(ref v) => v.max_utf8_buffer_length(byte_length),
        }
    }

    pub fn max_utf8_buffer_length_with_replacement(&self, byte_length: usize) -> usize {
        match self {
            &VariantDecoder::SingleByte(ref v) => {
                v.max_utf8_buffer_length_with_replacement(byte_length)
            }
            &VariantDecoder::Utf8(ref v) => v.max_utf8_buffer_length_with_replacement(byte_length),
            &VariantDecoder::Gb18030(ref v) => {
                v.max_utf8_buffer_length_with_replacement(byte_length)
            }
            &VariantDecoder::Big5(ref v) => v.max_utf8_buffer_length_with_replacement(byte_length),
            &VariantDecoder::EucJp(ref v) => v.max_utf8_buffer_length_with_replacement(byte_length),
            &VariantDecoder::Iso2022Jp(ref v) => {
                v.max_utf8_buffer_length_with_replacement(byte_length)
            }
            &VariantDecoder::ShiftJis(ref v) => {
                v.max_utf8_buffer_length_with_replacement(byte_length)
            }
            &VariantDecoder::EucKr(ref v) => v.max_utf8_buffer_length_with_replacement(byte_length),
            &VariantDecoder::Replacement(ref v) => {
                v.max_utf8_buffer_length_with_replacement(byte_length)
            }
            &VariantDecoder::UserDefined(ref v) => {
                v.max_utf8_buffer_length_with_replacement(byte_length)
            }
            &VariantDecoder::Utf16(ref v) => v.max_utf8_buffer_length_with_replacement(byte_length),
        }
    }

    pub fn decode_to_utf16(&mut self,
                           src: &[u8],
                           dst: &mut [u16],
                           last: bool)
                           -> (DecoderResult, usize, usize) {
        match self {
            &mut VariantDecoder::SingleByte(ref mut v) => v.decode_to_utf16(src, dst, last),
            &mut VariantDecoder::Utf8(ref mut v) => v.decode_to_utf16(src, dst, last),
            &mut VariantDecoder::Gb18030(ref mut v) => v.decode_to_utf16(src, dst, last),
            &mut VariantDecoder::Big5(ref mut v) => v.decode_to_utf16(src, dst, last),
            &mut VariantDecoder::EucJp(ref mut v) => v.decode_to_utf16(src, dst, last),
            &mut VariantDecoder::Iso2022Jp(ref mut v) => v.decode_to_utf16(src, dst, last),
            &mut VariantDecoder::ShiftJis(ref mut v) => v.decode_to_utf16(src, dst, last),
            &mut VariantDecoder::EucKr(ref mut v) => v.decode_to_utf16(src, dst, last),
            &mut VariantDecoder::Replacement(ref mut v) => v.decode_to_utf16(src, dst, last),
            &mut VariantDecoder::UserDefined(ref mut v) => v.decode_to_utf16(src, dst, last),
            &mut VariantDecoder::Utf16(ref mut v) => v.decode_to_utf16(src, dst, last),
        }
    }

    pub fn decode_to_utf8(&mut self,
                          src: &[u8],
                          dst: &mut [u8],
                          last: bool)
                          -> (DecoderResult, usize, usize) {
        match self {
            &mut VariantDecoder::SingleByte(ref mut v) => v.decode_to_utf8(src, dst, last),
            &mut VariantDecoder::Utf8(ref mut v) => v.decode_to_utf8(src, dst, last),
            &mut VariantDecoder::Gb18030(ref mut v) => v.decode_to_utf8(src, dst, last),
            &mut VariantDecoder::Big5(ref mut v) => v.decode_to_utf8(src, dst, last),
            &mut VariantDecoder::EucJp(ref mut v) => v.decode_to_utf8(src, dst, last),
            &mut VariantDecoder::Iso2022Jp(ref mut v) => v.decode_to_utf8(src, dst, last),
            &mut VariantDecoder::ShiftJis(ref mut v) => v.decode_to_utf8(src, dst, last),
            &mut VariantDecoder::EucKr(ref mut v) => v.decode_to_utf8(src, dst, last),
            &mut VariantDecoder::Replacement(ref mut v) => v.decode_to_utf8(src, dst, last),
            &mut VariantDecoder::UserDefined(ref mut v) => v.decode_to_utf8(src, dst, last),
            &mut VariantDecoder::Utf16(ref mut v) => v.decode_to_utf8(src, dst, last),
        }
    }
}

pub enum VariantEncoder {
    SingleByte(SingleByteEncoder),
    Utf8(Utf8Encoder),
    Gb18030(Gb18030Encoder),
    Big5(Big5Encoder),
    EucJp(EucJpEncoder),
    Iso2022Jp(Iso2022JpEncoder),
    ShiftJis(ShiftJisEncoder),
    EucKr(EucKrEncoder),
    UserDefined(UserDefinedEncoder),
    Utf16(Utf16Encoder),
}

impl VariantEncoder {
    pub fn reset(&mut self) {}

    pub fn max_buffer_length_from_utf16(&self, u16_length: usize) -> usize {
        match self {
            &VariantEncoder::SingleByte(ref v) => v.max_buffer_length_from_utf16(u16_length),
            &VariantEncoder::Utf8(ref v) => v.max_buffer_length_from_utf16(u16_length),
            &VariantEncoder::Gb18030(ref v) => v.max_buffer_length_from_utf16(u16_length),
            &VariantEncoder::Big5(ref v) => v.max_buffer_length_from_utf16(u16_length),
            &VariantEncoder::EucJp(ref v) => v.max_buffer_length_from_utf16(u16_length),
            &VariantEncoder::Iso2022Jp(ref v) => v.max_buffer_length_from_utf16(u16_length),
            &VariantEncoder::ShiftJis(ref v) => v.max_buffer_length_from_utf16(u16_length),
            &VariantEncoder::EucKr(ref v) => v.max_buffer_length_from_utf16(u16_length),
            &VariantEncoder::UserDefined(ref v) => v.max_buffer_length_from_utf16(u16_length),
            &VariantEncoder::Utf16(ref v) => v.max_buffer_length_from_utf16(u16_length),
        }
    }

    pub fn max_buffer_length_from_utf8(&self, byte_length: usize) -> usize {
        match self {
            &VariantEncoder::SingleByte(ref v) => v.max_buffer_length_from_utf8(byte_length),
            &VariantEncoder::Utf8(ref v) => v.max_buffer_length_from_utf8(byte_length),
            &VariantEncoder::Gb18030(ref v) => v.max_buffer_length_from_utf8(byte_length),
            &VariantEncoder::Big5(ref v) => v.max_buffer_length_from_utf8(byte_length),
            &VariantEncoder::EucJp(ref v) => v.max_buffer_length_from_utf8(byte_length),
            &VariantEncoder::Iso2022Jp(ref v) => v.max_buffer_length_from_utf8(byte_length),
            &VariantEncoder::ShiftJis(ref v) => v.max_buffer_length_from_utf8(byte_length),
            &VariantEncoder::EucKr(ref v) => v.max_buffer_length_from_utf8(byte_length),
            &VariantEncoder::UserDefined(ref v) => v.max_buffer_length_from_utf8(byte_length),
            &VariantEncoder::Utf16(ref v) => v.max_buffer_length_from_utf8(byte_length),
        }
    }

    pub fn max_buffer_length_from_utf16_with_replacement_if_no_unmappables(&self,
                                                                           u16_length: usize)
                                                                           -> usize {
        match self {
            &VariantEncoder::SingleByte(ref v) => {
                v.max_buffer_length_from_utf16_with_replacement_if_no_unmappables(u16_length)
            }
            &VariantEncoder::Utf8(ref v) => {
                v.max_buffer_length_from_utf16_with_replacement_if_no_unmappables(u16_length)
            }
            &VariantEncoder::Gb18030(ref v) => {
                v.max_buffer_length_from_utf16_with_replacement_if_no_unmappables(u16_length)
            }
            &VariantEncoder::Big5(ref v) => {
                v.max_buffer_length_from_utf16_with_replacement_if_no_unmappables(u16_length)
            }
            &VariantEncoder::EucJp(ref v) => {
                v.max_buffer_length_from_utf16_with_replacement_if_no_unmappables(u16_length)
            }
            &VariantEncoder::Iso2022Jp(ref v) => {
                v.max_buffer_length_from_utf16_with_replacement_if_no_unmappables(u16_length)
            }
            &VariantEncoder::ShiftJis(ref v) => {
                v.max_buffer_length_from_utf16_with_replacement_if_no_unmappables(u16_length)
            }
            &VariantEncoder::EucKr(ref v) => {
                v.max_buffer_length_from_utf16_with_replacement_if_no_unmappables(u16_length)
            }
            &VariantEncoder::UserDefined(ref v) => {
                v.max_buffer_length_from_utf16_with_replacement_if_no_unmappables(u16_length)
            }
            &VariantEncoder::Utf16(ref v) => {
                v.max_buffer_length_from_utf16_with_replacement_if_no_unmappables(u16_length)
            }
        }
    }

    pub fn max_buffer_length_from_utf8_with_replacement_if_no_unmappables(&self,
                                                                          byte_length: usize)
                                                                          -> usize {
        match self {
            &VariantEncoder::SingleByte(ref v) => {
                v.max_buffer_length_from_utf8_with_replacement_if_no_unmappables(byte_length)
            }
            &VariantEncoder::Utf8(ref v) => {
                v.max_buffer_length_from_utf8_with_replacement_if_no_unmappables(byte_length)
            }
            &VariantEncoder::Gb18030(ref v) => {
                v.max_buffer_length_from_utf8_with_replacement_if_no_unmappables(byte_length)
            }
            &VariantEncoder::Big5(ref v) => {
                v.max_buffer_length_from_utf8_with_replacement_if_no_unmappables(byte_length)
            }
            &VariantEncoder::EucJp(ref v) => {
                v.max_buffer_length_from_utf8_with_replacement_if_no_unmappables(byte_length)
            }
            &VariantEncoder::Iso2022Jp(ref v) => {
                v.max_buffer_length_from_utf8_with_replacement_if_no_unmappables(byte_length)
            }
            &VariantEncoder::ShiftJis(ref v) => {
                v.max_buffer_length_from_utf8_with_replacement_if_no_unmappables(byte_length)
            }
            &VariantEncoder::EucKr(ref v) => {
                v.max_buffer_length_from_utf8_with_replacement_if_no_unmappables(byte_length)
            }
            &VariantEncoder::UserDefined(ref v) => {
                v.max_buffer_length_from_utf8_with_replacement_if_no_unmappables(byte_length)
            }
            &VariantEncoder::Utf16(ref v) => {
                v.max_buffer_length_from_utf8_with_replacement_if_no_unmappables(byte_length)
            }
        }
    }

    pub fn encode_from_utf16(&mut self,
                             src: &[u16],
                             dst: &mut [u8],
                             last: bool)
                             -> (EncoderResult, usize, usize) {
        match self {
            &mut VariantEncoder::SingleByte(ref mut v) => v.encode_from_utf16(src, dst, last),
            &mut VariantEncoder::Utf8(ref mut v) => v.encode_from_utf16(src, dst, last),
            &mut VariantEncoder::Gb18030(ref mut v) => v.encode_from_utf16(src, dst, last),
            &mut VariantEncoder::Big5(ref mut v) => v.encode_from_utf16(src, dst, last),
            &mut VariantEncoder::EucJp(ref mut v) => v.encode_from_utf16(src, dst, last),
            &mut VariantEncoder::Iso2022Jp(ref mut v) => v.encode_from_utf16(src, dst, last),
            &mut VariantEncoder::ShiftJis(ref mut v) => v.encode_from_utf16(src, dst, last),
            &mut VariantEncoder::EucKr(ref mut v) => v.encode_from_utf16(src, dst, last),
            &mut VariantEncoder::UserDefined(ref mut v) => v.encode_from_utf16(src, dst, last),
            &mut VariantEncoder::Utf16(ref mut v) => v.encode_from_utf16(src, dst, last),
        }
    }

    pub fn encode_from_utf8(&mut self,
                            src: &str,
                            dst: &mut [u8],
                            last: bool)
                            -> (EncoderResult, usize, usize) {
        match self {
            &mut VariantEncoder::SingleByte(ref mut v) => v.encode_from_utf8(src, dst, last),
            &mut VariantEncoder::Utf8(ref mut v) => v.encode_from_utf8(src, dst, last),
            &mut VariantEncoder::Gb18030(ref mut v) => v.encode_from_utf8(src, dst, last),
            &mut VariantEncoder::Big5(ref mut v) => v.encode_from_utf8(src, dst, last),
            &mut VariantEncoder::EucJp(ref mut v) => v.encode_from_utf8(src, dst, last),
            &mut VariantEncoder::Iso2022Jp(ref mut v) => v.encode_from_utf8(src, dst, last),
            &mut VariantEncoder::ShiftJis(ref mut v) => v.encode_from_utf8(src, dst, last),
            &mut VariantEncoder::EucKr(ref mut v) => v.encode_from_utf8(src, dst, last),
            &mut VariantEncoder::UserDefined(ref mut v) => v.encode_from_utf8(src, dst, last),
            &mut VariantEncoder::Utf16(ref mut v) => v.encode_from_utf8(src, dst, last),
        }
    }
}

pub enum VariantEncoding {
    SingleByte(&'static [u16; 128]),
    Utf8,
    Gbk,
    Gb18030,
    Big5,
    EucJp,
    Iso2022Jp,
    ShiftJis,
    EucKr,
    Replacement,
    Utf16Be,
    Utf16Le,
    UserDefined,
}

impl VariantEncoding {
    pub fn new_decoder(&self, encoding: &'static Encoding) -> Decoder {
        match self {
            &VariantEncoding::SingleByte(table) => SingleByteDecoder::new(encoding, table),
            &VariantEncoding::Utf8 => Utf8Decoder::new(encoding),
            &VariantEncoding::Gbk | &VariantEncoding::Gb18030 => Gb18030Decoder::new(encoding),
            &VariantEncoding::Big5 => Big5Decoder::new(encoding),
            &VariantEncoding::EucJp => EucJpDecoder::new(encoding),
            &VariantEncoding::Iso2022Jp => Iso2022JpDecoder::new(encoding),
            &VariantEncoding::ShiftJis => ShiftJisDecoder::new(encoding),
            &VariantEncoding::EucKr => EucKrDecoder::new(encoding),
            &VariantEncoding::Replacement => ReplacementDecoder::new(encoding),
            &VariantEncoding::UserDefined => UserDefinedDecoder::new(encoding),
            &VariantEncoding::Utf16Be => Utf16Decoder::new(encoding, true),
            &VariantEncoding::Utf16Le => Utf16Decoder::new(encoding, false),
        }
    }

    pub fn new_encoder(&self, encoding: &'static Encoding) -> Encoder {
        match self {
            &VariantEncoding::SingleByte(table) => SingleByteEncoder::new(encoding, table),
            &VariantEncoding::Utf8 => Utf8Encoder::new(encoding),
            &VariantEncoding::Gbk => Gb18030Encoder::new(encoding, false),
            &VariantEncoding::Gb18030 => Gb18030Encoder::new(encoding, true),
            &VariantEncoding::Big5 => Big5Encoder::new(encoding),
            &VariantEncoding::EucJp => EucJpEncoder::new(encoding),
            &VariantEncoding::Iso2022Jp => Iso2022JpEncoder::new(encoding),
            &VariantEncoding::ShiftJis => ShiftJisEncoder::new(encoding),
            &VariantEncoding::EucKr => EucKrEncoder::new(encoding),
            &VariantEncoding::Replacement => Utf8Encoder::new(UTF_8),
            &VariantEncoding::UserDefined => UserDefinedEncoder::new(encoding),
            &VariantEncoding::Utf16Be => Utf16Encoder::new(encoding, true),
            &VariantEncoding::Utf16Le => Utf16Encoder::new(encoding, false),
        }
    }
}
