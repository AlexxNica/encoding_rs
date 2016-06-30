// Copyright 2015-2016 Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use handles::*;
use data::*;
use variant::*;
use super::*;

pub struct EucJpDecoder {
    lead: u8,
    jis0212: bool,
}

impl EucJpDecoder {
    pub fn new() -> VariantDecoder {
        VariantDecoder::EucJp(EucJpDecoder {
            lead: 0,
            jis0212: false,
        })
    }

    fn plus_one_if_lead(&self, byte_length: usize) -> usize {
        byte_length +
        if self.lead == 0 {
            0
        } else {
            1
        }
    }

    pub fn max_utf16_buffer_length(&self, byte_length: usize) -> usize {
        self.plus_one_if_lead(byte_length)
    }

    pub fn max_utf8_buffer_length(&self, byte_length: usize) -> usize {
        // worst case: 2 to 3
        let len = self.plus_one_if_lead(byte_length);
        len + (len + 1) / 2
    }

    pub fn max_utf8_buffer_length_with_replacement(&self, byte_length: usize) -> usize {
        self.plus_one_if_lead(byte_length) * 3
    }

    decoder_functions!({},
                       {
                           if self.lead != 0 {
                               self.lead = 0;
                               return (DecoderResult::Malformed(1, 0),
                                       src_consumed,
                                       dest.written());
                           }
                       },
                       {
                           if self.lead == 0 {
                               if b <= 0x7f {
                                   // TODO optimize ASCII run
                                   destination_handle.write_ascii(b);
                                   continue;
                               }
                               if (b >= 0xA1 && b <= 0xFE) || b == 0x8E || b == 0x8F {
                                   self.lead = b;
                                   continue;
                               }
                               return (DecoderResult::Malformed(1, 0),
                                       unread_handle.consumed(),
                                       destination_handle.written());
                           }
                           let lead = self.lead as usize;
                           self.lead = 0;
                           // Comparison to 0xA1 could be hoisted, but the
                           // form below matches the spec better.
                           if lead == 0x8E && (b >= 0xA1 && b <= 0xDF) {
                               destination_handle.write_upper_bmp(0xFF61 - 0xA1 + b as u16);
                               continue;
                           }
                           if lead == 0x8F && (b >= 0xA1 && b <= 0xFE) {
                               self.lead = b;
                               self.jis0212 = true;
                               continue;
                           }
                           if (b >= 0xA1 && b <= 0xFE) && (lead >= 0xA1 && lead <= 0xFE) {
                               let pointer = (lead as usize - 0xA1) * 94usize + (b as usize - 0xA1);
                               let bmp = if self.jis0212 {
                                   self.jis0212 = false;
                                   jis0212_decode(pointer)
                               } else {
                                   jis0208_decode(pointer)
                               };
                               if bmp != 0 {
                                   destination_handle.write_bmp_excl_ascii(bmp);
                                   continue;
                               }
                           }
                           if b < 0xA1 || b == 0xFF {
                               return (DecoderResult::Malformed(1, 0),
                                       unread_handle.unread(),
                                       destination_handle.written());
                           }
                           return (DecoderResult::Malformed(2, 0),
                                   unread_handle.consumed(),
                                   destination_handle.written());
                       },
                       self,
                       src_consumed,
                       dest,
                       b,
                       destination_handle,
                       unread_handle,
                       check_space_bmp);
}

pub struct EucJpEncoder;

impl EucJpEncoder {
    pub fn new(encoding: &'static Encoding) -> Encoder {
        Encoder::new(encoding, VariantEncoder::EucJp(EucJpEncoder))
    }

    pub fn max_buffer_length_from_utf16(&self, u16_length: usize) -> usize {
        u16_length * 2
    }

    pub fn max_buffer_length_from_utf8(&self, byte_length: usize) -> usize {
        byte_length
    }

    encoder_functions!({},
                       {
                           if c <= '\u{7F}' {
                               // TODO optimize ASCII run
                               destination_handle.write_one(c as u8);
                               continue;
                           }
                           if c == '\u{A5}' {
                               destination_handle.write_one(0x5Cu8);
                               continue;
                           }
                           if c == '\u{203E}' {
                               destination_handle.write_one(0x7Eu8);
                               continue;
                           }
                           if c >= '\u{FF61}' && c <= '\u{FF9F}' {
                               destination_handle.write_two(0x8Eu8,
                                                            (c as usize - 0xFF61 + 0xA1) as u8);
                               continue;
                           }
                           if c == '\u{2212}' {
                               destination_handle.write_two(0xA1u8, 0xDDu8);
                               continue;
                           }
                           let pointer = jis0208_encode(c);
                           if pointer == usize::max_value() {
                               return (EncoderResult::Unmappable(c),
                                       unread_handle.consumed(),
                                       destination_handle.written());
                           }
                           let lead = (pointer / 94) + 0xA1;
                           let trail = (pointer % 94) + 0xA1;
                           destination_handle.write_two(lead as u8, trail as u8);
                           continue;
                       },
                       self,
                       src_consumed,
                       source,
                       dest,
                       c,
                       destination_handle,
                       unread_handle,
                       check_space_two);
}

#[cfg(test)]
mod tests {
    use super::super::testing::*;
    use super::super::*;

    fn decode_euc_jp(bytes: &[u8], expect: &str) {
        decode(EUC_JP, bytes, expect);
    }

    fn encode_euc_jp(string: &str, expect: &[u8]) {
        encode(EUC_JP, string, expect);
    }

    #[test]
    fn test_euc_jp_decode() {
        // ASCII
        decode_euc_jp(b"\x61\x62", "\u{0061}\u{0062}");

        // Half-width
        decode_euc_jp(b"\x8E\xA1", "\u{FF61}");
        decode_euc_jp(b"\x8E\xDF", "\u{FF9F}");
        decode_euc_jp(b"\x8E\xA0", "\u{FFFD}\u{FFFD}");
        decode_euc_jp(b"\x8E\xE0", "\u{FFFD}");
        decode_euc_jp(b"\x8E\xFF", "\u{FFFD}\u{FFFD}");

        // JIS 0212
        decode_euc_jp(b"\x8F\xA1\xA1", "\u{FFFD}");
        decode_euc_jp(b"\x8F\xA2\xAF", "\u{02D8}");
        decode_euc_jp(b"\x8F\xA2\xFF", "\u{FFFD}\u{FFFD}");

        // JIS 0208
        decode_euc_jp(b"\xA1\xA1", "\u{3000}");
        decode_euc_jp(b"\xA1\xA0", "\u{FFFD}\u{FFFD}");
        decode_euc_jp(b"\xFC\xFE", "\u{FF02}");
        decode_euc_jp(b"\xFE\xFE", "\u{FFFD}");

        // Bad leads
        decode_euc_jp(b"\xFF\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\xA0\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\x80\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\x81\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\x82\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\x83\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\x84\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\x85\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\x86\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\x87\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\x88\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\x89\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\x8A\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\x8B\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\x8C\xA1\xA1", "\u{FFFD}\u{3000}");
        decode_euc_jp(b"\x8D\xA1\xA1", "\u{FFFD}\u{3000}");

        // Bad ASCII trail
        decode_euc_jp(b"\xA1\x40", "\u{FFFD}\u{0040}");        
    }

    #[test]
    fn test_euc_jp_encode() {
        // ASCII
        encode_euc_jp("\u{0061}\u{0062}", b"\x61\x62");

        // Exceptional code points
        encode_euc_jp("\u{00A5}", b"\x5C");
        encode_euc_jp("\u{203E}", b"\x7E");
        encode_euc_jp("\u{2212}", b"\xA1\xDD");

        // Half-width
        encode_euc_jp("\u{FF61}", b"\x8E\xA1");
        encode_euc_jp("\u{FF9F}", b"\x8E\xDF");

        // JIS 0212
        encode_euc_jp("\u{02D8}", b"&#728;");

        // JIS 0208
        encode_euc_jp("\u{3000}", b"\xA1\xA1");
        encode_euc_jp("\u{FF02}", b"\xFC\xFE");
    }

}
