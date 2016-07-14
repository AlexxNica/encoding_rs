// Copyright 2015-2016 Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use variant::*;
use super::*;

pub struct ReplacementDecoder {
    emitted: bool,
}

impl ReplacementDecoder {
    pub fn new() -> VariantDecoder {
        VariantDecoder::Replacement(ReplacementDecoder { emitted: false })
    }

    pub fn max_utf16_buffer_length(&self, _u16_length: usize) -> usize {
        1
    }

    pub fn max_utf8_buffer_length_without_replacement(&self, _byte_length: usize) -> usize {
        1 // really zero, but that might surprise callers
    }

    pub fn max_utf8_buffer_length(&self, _byte_length: usize) -> usize {
        3
    }

    fn decode(&mut self, src: &[u8]) -> (DecoderResult, usize, usize) {
        // Don't err if the input stream is empty. See
        // https://github.com/whatwg/encoding/issues/33
        if self.emitted || src.is_empty() {
            (DecoderResult::InputEmpty, src.len(), 0)
        } else {
            // We don't need to check if output has enough space, because
            // everything is weird anyway if the caller of the `Decoder` API
            // passes an output buffer that violates the minimum size rules.
            self.emitted = true;
            (DecoderResult::Malformed(1, 0), 1, 0)
        }
    }

    pub fn decode_to_utf16_raw(&mut self,
                               src: &[u8],
                               _dst: &mut [u16],
                               _last: bool)
                               -> (DecoderResult, usize, usize) {
        self.decode(src)
    }

    pub fn decode_to_utf8_raw(&mut self,
                              src: &[u8],
                              _dst: &mut [u8],
                              _last: bool)
                              -> (DecoderResult, usize, usize) {
        self.decode(src)
    }
}

// Any copyright to the test code below this comment is dedicated to the
// Public Domain. http://creativecommons.org/publicdomain/zero/1.0/

#[cfg(test)]
mod tests {
    use super::super::testing::*;
    use super::super::*;

    fn decode_replacement(bytes: &[u8], expect: &str) {
        decode(REPLACEMENT, bytes, expect);
    }

    fn encode_replacement(string: &str, expect: &[u8]) {
        encode(REPLACEMENT, string, expect);
    }

    #[test]
    fn test_replacement_decode() {
        decode_replacement(b"", "");
        decode_replacement(b"A", "\u{FFFD}");
        decode_replacement(b"AB", "\u{FFFD}");
    }

    #[test]
    fn test_replacement_encode() {
        assert_eq!(REPLACEMENT.new_encoder().encoding(), UTF_8);
        encode_replacement("\u{1F4A9}\u{2603}", "\u{1F4A9}\u{2603}".as_bytes());
    }
}
