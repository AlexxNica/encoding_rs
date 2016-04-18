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

pub struct Gb18030Decoder {
    first: u8,
    second: u8,
    third: u8,
    pending_ascii: u8,
}

impl Gb18030Decoder {
    pub fn new() -> VariantDecoder {
        VariantDecoder::Gb18030(Gb18030Decoder {
            first: 0,
            second: 0,
            third: 0,
            pending_ascii: 0,
        })
    }

    pub fn max_utf16_buffer_length(&self, u16_length: usize) -> usize {
        u16_length
    }

    pub fn max_utf8_buffer_length(&self, byte_length: usize) -> usize {
        byte_length * 3
    }

    pub fn max_utf8_buffer_length_with_replacement(&self, byte_length: usize) -> usize {
        byte_length * 3
    }

    decoder_functions!({
                           if self.pending_ascii != 0 {
                               match dest.check_space_bmp() {
                                   Space::Full(_) => {
                                       return (DecoderResult::OutputFull, 0, 0);
                                   }
                                   Space::Available(destination_handle) => {
                                       destination_handle.write_ascii(self.pending_ascii);
                                       self.pending_ascii = 0;
                                   }
                               }
                           }
                       },
                       {
                           if self.third != 0 {
                               self.first = 0;
                               self.second = 0;
                               self.third = 0;
                               return (DecoderResult::Malformed(3, 0),
                                       src_consumed,
                                       dest.written());
                           }
                           if self.second != 0 {
                               self.first = 0;
                               self.second = 0;
                               self.third = 0;
                               return (DecoderResult::Malformed(2, 0),
                                       src_consumed,
                                       dest.written());
                           }
                           if self.first != 0 {
                               self.first = 0;
                               self.second = 0;
                               self.third = 0;
                               return (DecoderResult::Malformed(1, 0),
                                       src_consumed,
                                       dest.written());
                           }
                       },
                       {
                           if self.first == 0 {
                               debug_assert_eq!(self.second, 0);
                               debug_assert_eq!(self.third, 0);
                               if b <= 0x7f {
                                   // TODO optimize ASCII run
                                   destination_handle.write_ascii(b);
                                   continue;
                               }
                               if b == 0x80 {
                                   destination_handle.write_upper_bmp(0x20ACu16);
                                   continue;
                               }
                               if b >= 0x81 && b <= 0xFE {
                                   self.first = b;
                                   continue;
                               }
                               return (DecoderResult::Malformed(1, 0),
                                       unread_handle.consumed(),
                                       destination_handle.written());
                           }
                           if self.third != 0 {
                               let first = self.first;
                               let second = self.second;
                               let third = self.third;
                               self.first = 0;
                               self.second = 0;
                               self.third = 0;
                               if b >= 0x30 && b <= 0x39 {
                                   let pointer = (((first as usize - 0x81) * 10 + second as usize -
                                                   0x30) *
                                                  126 +
                                                  third as usize -
                                                  0x81) *
                                                 10 +
                                                 b as usize -
                                                 0x30;
                                   let c = gb18030_range_decode(pointer);
                                   if c != '\u{0}' {
                                       destination_handle.write_char_excl_ascii(c);
                                       continue;
                                   }
                               }
                               // We have an error. Let's inline what's going
                               // to happen when `second` and `third` are
                               // reprocessed. (`b` gets unread.)
                               debug_assert!(second >= 0x30 && second <= 0x39);
                               // `second` is guaranteed ASCII, so let's
                               // put it in `pending_ascii`
                               self.pending_ascii = second;
                               debug_assert!(third >= 0x81 && third <= 0xFE);
                               // `third` is guaranteed to be in the range
                               // that makes it become the new `self.first`.
                               self.first = third;
                               // Now unread `b` and designate the previous
                               // `first` as being in error.
                               return (DecoderResult::Malformed(1, 2),
                                       unread_handle.unread(),
                                       destination_handle.written());
                           }
                           if self.second != 0 {
                               debug_assert_eq!(self.third, 0);
                               if b >= 0x81 && b <= 0xFE {
                                   self.third = b;
                                   continue;
                               }
                               let second = self.second;
                               self.second = 0;
                               self.first = 0;
                               // We have an error. Let's inline what's going
                               // to happen when `second` is
                               // reprocessed. (`b` gets unread.)
                               debug_assert!(second >= 0x30 && second <= 0x39);
                               // `second` is guaranteed ASCII, so let's
                               // put it in `pending_ascii`
                               self.pending_ascii = second;
                               // Now unread `b` and designate the previous
                               // `first` as being in error.
                               return (DecoderResult::Malformed(1, 1),
                                       unread_handle.unread(),
                                       destination_handle.written());
                           }
                           // self.first != 0
                           debug_assert_eq!(self.second, 0);
                           debug_assert_eq!(self.third, 0);
                           if b >= 0x30 && b <= 0x39 {
                               self.second = b;
                               continue;
                           }
                           let lead = self.first;
                           self.first = 0;
                           let offset = if b < 0x7F {
                               0x40usize
                           } else {
                               0x41usize
                           };
                           if (b >= 0x40 && b <= 0x7E) || (b >= 0x80 && b <= 0xFE) {
                               let pointer = (lead as usize - 0x81) * 190usize +
                                             (b as usize - offset);
                               let bmp = gb18030_decode(pointer);
                               if bmp != 0 {
                                   destination_handle.write_bmp_excl_ascii(bmp);
                                   continue;
                               }
                           }
                           if b <= 0x7F {
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
                       check_space_astral);
}

pub struct Gb18030Encoder;

impl Gb18030Encoder {
    pub fn new(encoding: &'static Encoding, extended_range: bool) -> Encoder {
        Encoder::new(encoding, VariantEncoder::Gb18030(Gb18030Encoder))
    }

    pub fn max_buffer_length_from_utf16(&self, u16_length: usize) -> usize {
        0 // TODO
    }

    pub fn max_buffer_length_from_utf8(&self, byte_length: usize) -> usize {
        0 // TODO
    }

    pub fn max_buffer_length_from_utf16_with_replacement_if_no_unmappables(&self,
                                                                           u16_length: usize)
                                                                           -> usize {
        0 // TODO
    }

    pub fn max_buffer_length_from_utf8_with_replacement_if_no_unmappables(&self,
                                                                          byte_length: usize)
                                                                          -> usize {
        0 // TODO
    }

    pub fn encode_from_utf16(&mut self,
                             src: &[u16],
                             dst: &mut [u8],
                             last: bool)
                             -> (EncoderResult, usize, usize) {
        // XXX
        (EncoderResult::InputEmpty, 0, 0)
    }

    pub fn encode_from_utf8(&mut self,
                            src: &str,
                            dst: &mut [u8],
                            last: bool)
                            -> (EncoderResult, usize, usize) {
        // XXX
        (EncoderResult::InputEmpty, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

}
