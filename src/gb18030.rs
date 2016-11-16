// Copyright 2015-2016 Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use handles::*;
use data::*;
use variant::*;
use super::*;

#[inline(always)]
fn call_gb18030_range_decode(first_minus_offset: u8,
                             second_minus_offset: u8,
                             third_minus_offset: u8,
                             fourth_minus_offset: u8)
                             -> char {
    if fourth_minus_offset > (0x39 - 0x30) {
        return '\u{0}';
    }
    let pointer = (first_minus_offset as usize * (10 * 126 * 10)) +
                  (second_minus_offset as usize * (10 * 126)) +
                  (third_minus_offset as usize * 10) +
                  fourth_minus_offset as usize;
    gb18030_range_decode(pointer)
}

enum Gb18030Pending {
    None,
    One(u8),
    Two(u8, u8),
    Three(u8, u8, u8),
}

impl Gb18030Pending {
    fn is_none(&self) -> bool {
        match self {
            &Gb18030Pending::None => true,
            _ => false,
        }
    }

    fn count(&self) -> usize {
        match self {
            &Gb18030Pending::None => 0,
            &Gb18030Pending::One(_) => 1,
            &Gb18030Pending::Two(_, _) => 2,
            &Gb18030Pending::Three(_, _, _) => 3,
        }
    }
}

pub struct Gb18030Decoder {
    first: Option<u8>,
    second: Option<u8>,
    third: Option<u8>,
    pending: Gb18030Pending,
    pending_ascii: Option<u8>,
}

impl Gb18030Decoder {
    pub fn new() -> VariantDecoder {
        VariantDecoder::Gb18030(Gb18030Decoder {
            first: None,
            second: None,
            third: None,
            pending: Gb18030Pending::None,
            pending_ascii: None,
        })
    }

    fn extra_from_state(&self, byte_length: usize) -> usize {
        byte_length + self.pending.count() +
        match self.first {
            None => 0,
            Some(_) => 1,
        } +
        match self.second {
            None => 0,
            Some(_) => 1,
        } +
        match self.third {
            None => 0,
            Some(_) => 1,
        } +
        match self.pending_ascii {
            None => 0,
            Some(_) => 1,
        }
    }

    pub fn max_utf16_buffer_length(&self, byte_length: usize) -> usize {
        // ASCII: 1 to 1 (worst case)
        // gbk: 2 to 1
        // ranges: 4 to 1 or 4 to 2
        self.extra_from_state(byte_length) + 1
    }

    pub fn max_utf8_buffer_length_without_replacement(&self, byte_length: usize) -> usize {
        // ASCII: 1 to 1
        // gbk: 2 to 2 or 2 to 3
        // ranges: 4 to 2, 4 to 3 or 4 to 4
        // 0x80: 1 to 3 (worst case)
        (self.extra_from_state(byte_length) * 3) + 1
    }

    pub fn max_utf8_buffer_length(&self, byte_length: usize) -> usize {
        (self.extra_from_state(byte_length) * 3) + 1
    }

    pub fn decode_to_utf8_raw(&mut self,
                              src: &[u8],
                              dst: &mut [u8],
                              last: bool)
                              -> (DecoderResult, usize, usize) {
        let mut source = ByteSource::new(src);
        let mut dest = Utf8Destination::new(dst);
        {
            match self.pending_ascii {
                Some(ascii) => {
                    match dest.check_space_bmp() {
                        Space::Full(_) => {
                            return (DecoderResult::OutputFull, 0, 0);
                        }
                        Space::Available(pending_ascii_handle) => {
                            self.pending_ascii = None;
                            pending_ascii_handle.write_ascii(ascii);
                        }
                    }
                }
                None => {}
            }
        }
        while !self.pending.is_none() {
            match source.check_available() {
                Space::Full(src_consumed) => {
                    if last {
                        // Start non-boilerplate
                        let count = self.pending.count();
                        self.pending = Gb18030Pending::None;
                        return (DecoderResult::Malformed(count as u8, 0),
                                src_consumed,
                                dest.written());
                        // End non-boilerplate
                    }
                    return (DecoderResult::InputEmpty, src_consumed, dest.written());
                }
                Space::Available(source_handle) => {
                    match dest.check_space_astral() {
                        Space::Full(dst_written) => {
                            return (DecoderResult::OutputFull,
                                    source_handle.consumed(),
                                    dst_written);
                        }
                        Space::Available(handle) => {
                            let (byte, unread_handle) = source_handle.read();
                            match self.pending {
                                Gb18030Pending::One(first_minus_offset) => {
                                    self.pending = Gb18030Pending::None;
                                    let second = byte;
                                    let unread_handle_second = unread_handle;
                                    // Start non-boilerplate
                                    // If second is between 0x40 and 0x7E,
                                    // inclusive, subtract offset 0x40. Else if
                                    // second is between 0x80 and 0xFE, inclusive,
                                    // subtract offset 0x41. In both cases,
                                    // handle as a two-byte sequence.
                                    // Else if second is between 0x30 and 0x39,
                                    // inclusive, subtract offset 0x30 and
                                    // handle as a four-byte sequence.
                                    let second_minus_offset = second.wrapping_sub(0x30);
                                    // It's not optimal to do this check first,
                                    // but this results in more readable code.
                                    if second_minus_offset > (0x39 - 0x30) {
                                        // Start non-boilerplate
                                        // Two-byte (or error)
                                        let mut trail_minus_offset = second.wrapping_sub(0x40);
                                        if trail_minus_offset > (0x7E - 0x40) {
                                            let trail_minus_range_start = second.wrapping_sub(0x80);
                                            if trail_minus_range_start > (0xFE - 0x80) {
                                                if second < 0x80 {
                                                    return (DecoderResult::Malformed(1, 0),
                                                            unread_handle_second.unread(),
                                                            handle.written());
                                                }
                                                return (DecoderResult::Malformed(2, 0),
                                                        unread_handle_second.consumed(),
                                                        handle.written());
                                            }
                                            trail_minus_offset = second - 0x41;
                                        }
                                        let pointer = first_minus_offset as usize * 190usize +
                                                      trail_minus_offset as usize;
                                        let bmp = gb18030_decode(pointer);
                                        if bmp == 0 {
                                            if second < 0x80 {
                                                return (DecoderResult::Malformed(1, 0),
                                                        unread_handle_second.unread(),
                                                        handle.written());
                                            }
                                            return (DecoderResult::Malformed(2, 0),
                                                    unread_handle_second.consumed(),
                                                    handle.written());
                                        }
                                        handle.write_bmp_excl_ascii(bmp)
                                        // End non-boilerplate
                                    } else {
                                        // Four-byte!
                                        self.pending = Gb18030Pending::Two(first_minus_offset,
                                                                           second_minus_offset);
                                        handle.decommit()
                                    }
                                }
                                Gb18030Pending::Two(first_minus_offset, second_minus_offset) => {
                                    self.pending = Gb18030Pending::None;
                                    let third = byte;
                                    let unread_handle_third = unread_handle;
                                    // Start non-boilerplate
                                    // If third is between 0x81 and 0xFE, inclusive,
                                    // subtract offset 0x81.
                                    let third_minus_offset = third.wrapping_sub(0x81);
                                    if third_minus_offset > (0xFE - 0x81) {
                                        // We have an error. Let's inline what's going
                                        // to happen when `second` is
                                        // reprocessed. (`third` gets unread.)
                                        // `second` is guaranteed ASCII, so let's
                                        // put it in `pending_ascii`. Recompute
                                        // `second` from `second_minus_offset`.
                                        self.pending_ascii = Some(second_minus_offset + 0x30);
                                        // Now unread `third` and designate the previous
                                        // `first` as being in error.
                                        return (DecoderResult::Malformed(1, 1),
                                                unread_handle_third.unread(),
                                                handle.written());
                                    }
                                    // End non-boilerplate
                                    self.pending = Gb18030Pending::Three(first_minus_offset,
                                                                         second_minus_offset,
                                                                         third_minus_offset);
                                    handle.decommit()
                                }
                                Gb18030Pending::Three(first_minus_offset,
                                                      second_minus_offset,
                                                      third_minus_offset) => {
                                    self.pending = Gb18030Pending::None;
                                    let fourth = byte;
                                    let unread_handle_fourth = unread_handle;
                                    // Start non-boilerplate
                                    // If fourth is between 0x30 and 0x39, inclusive,
                                    // subtract offset 0x30.
                                    let fourth_minus_offset = fourth.wrapping_sub(0x30);
                                    let c = call_gb18030_range_decode(first_minus_offset,
                                                                      second_minus_offset,
                                                                      third_minus_offset,
                                                                      fourth_minus_offset);
                                    if c == '\u{0}' {
                                        // We have an error. Let's inline what's going
                                        // to happen when `second` and `third` are
                                        // reprocessed. (`fourth` gets unread.)
                                        // `second` is guaranteed ASCII, so let's
                                        // put it in `pending_ascii`. Recompute
                                        // `second` from `second_minus_offset` to
                                        // make this block reusable when `second`
                                        // is not in scope.
                                        self.pending_ascii = Some(second_minus_offset + 0x30);
                                        // `third` is guaranteed to be in the range
                                        // that makes it become the new `self.first`.
                                        self.pending = Gb18030Pending::One(third_minus_offset);
                                        // Now unread `fourth` and designate the previous
                                        // `first` as being in error.
                                        return (DecoderResult::Malformed(1, 2),
                                                unread_handle_fourth.unread(),
                                                handle.written());
                                    }
                                    handle.write_char_excl_ascii(c)
                                    // End non-boilerplate
                                }
                                Gb18030Pending::None => unreachable!("Checked in loop condition"),
                            };
                        }
                    }
                }
            }
        }
        'outermost: loop {
            match dest.copy_ascii_from_check_space_astral(&mut source) {
                CopyAsciiResult::Stop(ret) => return ret,
                CopyAsciiResult::GoOn((mut non_ascii, mut handle)) => {
                    'middle: loop {
                        let dest_again = {
                            let first_minus_offset = {
                                // Start non-boilerplate
                                // If first is between 0x81 and 0xFE, inclusive,
                                // subtract offset 0x81.
                                let non_ascii_minus_offset = non_ascii.wrapping_sub(0x81);
                                if non_ascii_minus_offset > (0xFE - 0x81) {
                                    if non_ascii == 0x80 {
                                        handle.write_upper_bmp(0x20ACu16);
                                        continue 'outermost;
                                    }
                                    return (DecoderResult::Malformed(1, 0),
                                            source.consumed(),
                                            handle.written());
                                }
                                non_ascii_minus_offset
                                // End non-boilerplate
                            };
                            match source.check_available() {
                                Space::Full(src_consumed_trail) => {
                                    if last {
                                        return (DecoderResult::Malformed(1, 0),
                                                src_consumed_trail,
                                                handle.written());
                                    }
                                    self.pending = Gb18030Pending::One(first_minus_offset);
                                    return (DecoderResult::InputEmpty,
                                            src_consumed_trail,
                                            handle.written());
                                }
                                Space::Available(source_handle_trail) => {
                                    let (second, unread_handle_second) = source_handle_trail.read();
                                    // Start non-boilerplate
                                    // If second is between 0x40 and 0x7E,
                                    // inclusive, subtract offset 0x40. Else if
                                    // second is between 0x80 and 0xFE, inclusive,
                                    // subtract offset 0x41. In both cases,
                                    // handle as a two-byte sequence.
                                    // Else if second is between 0x30 and 0x39,
                                    // inclusive, subtract offset 0x30 and
                                    // handle as a four-byte sequence.
                                    let second_minus_offset = second.wrapping_sub(0x30);
                                    // It's not optimal to do this check first,
                                    // but this results in more readable code.
                                    if second_minus_offset > (0x39 - 0x30) {
                                        // Start non-boilerplate
                                        // Two-byte (or error)
                                        let mut trail_minus_offset = second.wrapping_sub(0x40);
                                        if trail_minus_offset > (0x7E - 0x40) {
                                            let trail_minus_range_start = second.wrapping_sub(0x80);
                                            if trail_minus_range_start > (0xFE - 0x80) {
                                                if second < 0x80 {
                                                    return (DecoderResult::Malformed(1, 0),
                                                            unread_handle_second.unread(),
                                                            handle.written());
                                                }
                                                return (DecoderResult::Malformed(2, 0),
                                                        unread_handle_second.consumed(),
                                                        handle.written());
                                            }
                                            trail_minus_offset = second - 0x41;
                                        }
                                        let pointer = first_minus_offset as usize * 190usize +
                                                      trail_minus_offset as usize;
                                        let bmp = gb18030_decode(pointer);
                                        if bmp == 0 {
                                            if second < 0x80 {
                                                return (DecoderResult::Malformed(1, 0),
                                                        unread_handle_second.unread(),
                                                        handle.written());
                                            }
                                            return (DecoderResult::Malformed(2, 0),
                                                    unread_handle_second.consumed(),
                                                    handle.written());
                                        }
                                        handle.write_bmp_excl_ascii(bmp)
                                    } else {
                                        // Four-byte!
                                        // End non-boilerplate
                                        match unread_handle_second.decommit().check_available() {
                                            Space::Full(src_consumed_third) => {
                                                if last {
                                                    return (DecoderResult::Malformed(2, 0),
                                                            src_consumed_third,
                                                            handle.written());
                                                }
                                                self.pending =
                                                    Gb18030Pending::Two(first_minus_offset,
                                                                        second_minus_offset);
                                                return (DecoderResult::InputEmpty,
                                                        src_consumed_third,
                                                        handle.written());
                                            }
                                            Space::Available(source_handle_third) => {
                                                let (third, unread_handle_third) =
                                                    source_handle_third.read();
                                                // Start non-boilerplate
                                                // If third is between 0x81 and 0xFE, inclusive,
                                                // subtract offset 0x81.
                                                let third_minus_offset = third.wrapping_sub(0x81);
                                                if third_minus_offset > (0xFE - 0x81) {
                                                    // We have an error. Let's inline what's going
                                                    // to happen when `second` is
                                                    // reprocessed. (`third` gets unread.)
                                                    debug_assert!(second >= 0x30 && second <= 0x39);
                                                    // `second` is guaranteed ASCII, so let's
                                                    // put it in `pending_ascii`
                                                    self.pending_ascii = Some(second);
                                                    // Now unread `third` and designate the previous
                                                    // `first` as being in error.
                                                    return (DecoderResult::Malformed(1, 1),
                                                            unread_handle_third.unread(),
                                                            handle.written());
                                                }
                                                // End non-boilerplate
                                                match unread_handle_third.decommit()
                                                                         .check_available() {
                                                    Space::Full(src_consumed_fourth) => {
                                                        if last {
                                                            return (DecoderResult::Malformed(3, 0),
                                                                    src_consumed_fourth,
                                                                    handle.written());
                                                        }
                                                        self.pending = Gb18030Pending::Three(first_minus_offset, second_minus_offset, third_minus_offset);
                                                        return (DecoderResult::InputEmpty,
                                                                src_consumed_fourth,
                                                                handle.written());
                                                    }
                                                    Space::Available(source_handle_fourth) => {
                                                        let (fourth, unread_handle_fourth) =
                                                            source_handle_fourth.read();
                                                        // Start non-boilerplate
                                                        // If fourth is between 0x30 and 0x39, inclusive,
                                                        // subtract offset 0x30.
                                                        let fourth_minus_offset =
                                                            fourth.wrapping_sub(0x30);
                                                        let c = call_gb18030_range_decode(first_minus_offset, second_minus_offset, third_minus_offset, fourth_minus_offset);
                                                        if c == '\u{0}' {
                                                            // We have an error. Let's inline what's going
                                                            // to happen when `second` and `third` are
                                                            // reprocessed. (`fourth` gets unread.)
                                                            // `second` is guaranteed ASCII, so let's
                                                            // put it in `pending_ascii`. Recompute
                                                            // `second` from `second_minus_offset` to
                                                            // make this block reusable when `second`
                                                            // is not in scope.
                                                            self.pending_ascii =
                                                                Some(second_minus_offset + 0x30);
                                                            debug_assert!(third >= 0x81 &&
                                                                          third <= 0xFE);
                                                            // `third` is guaranteed to be in the range
                                                            // that makes it become the new `self.first`.
                                                            self.pending = Gb18030Pending::One(third_minus_offset);
                                                            // Now unread `fourth` and designate the previous
                                                            // `first` as being in error.
                                                            return (DecoderResult::Malformed(1, 2),
                                                                    unread_handle_fourth.unread(),
                                                                    handle.written());
                                                        }
                                                        handle.write_char_excl_ascii(c)
                                                        // End non-boilerplate
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // End non-boilerplate
                                }
                            }
                        };
                        match source.check_available() {
                            Space::Full(src_consumed) => {
                                return (DecoderResult::InputEmpty,
                                        src_consumed,
                                        dest_again.written());
                            }
                            Space::Available(source_handle) => {
                                match dest_again.check_space_astral() {
                                    Space::Full(dst_written) => {
                                        return (DecoderResult::OutputFull,
                                                source_handle.consumed(),
                                                dst_written);
                                    }
                                    Space::Available(destination_handle) => {
                                        let (b, _) = source_handle.read();
                                        'innermost: loop {
                                            if b > 127 {
                                                non_ascii = b;
                                                handle = destination_handle;
                                                continue 'middle;
                                            }
                                            // Testing on Haswell says that we should write the
                                            // byte unconditionally instead of trying to unread it
                                            // to make it part of the next SIMD stride.
                                            destination_handle.write_ascii(b);
                                            // We've got markup or ASCII text
                                            continue 'outermost;
                                        }
                                    }
                                }
                            }
                        }
                        unreachable!("Should always continue earlier.");
                    }
                }
            }
            unreachable!("Should always continue earlier.");
        }
    }

    decoder_function!({
                          if self.pending_ascii.is_some() {
                              match dest.check_space_bmp() {
                                  Space::Full(_) => {
                                      return (DecoderResult::OutputFull, 0, 0);
                                  }
                                  Space::Available(destination_handle) => {
                                      destination_handle.write_ascii(self.pending_ascii.unwrap());
                                      self.pending_ascii = None;
                                  }
                              }
                          }
                      },
                      {
                          if self.third.is_some() {
                              self.first = None;
                              self.second = None;
                              self.third = None;
                              return (DecoderResult::Malformed(3, 0), src_consumed, dest.written());
                          }
                          if self.second.is_some() {
                              self.first = None;
                              self.second = None;
                              self.third = None;
                              return (DecoderResult::Malformed(2, 0), src_consumed, dest.written());
                          }
                          if self.first.is_some() {
                              self.first = None;
                              self.second = None;
                              self.third = None;
                              return (DecoderResult::Malformed(1, 0), src_consumed, dest.written());
                          }
                      },
                      {
                          if self.first.is_none() {
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
                                  self.first = Some(b);
                                  continue;
                              }
                              return (DecoderResult::Malformed(1, 0),
                                      unread_handle.consumed(),
                                      destination_handle.written());
                          }
                          if self.third.is_some() {
                              let first = self.first.unwrap();
                              let second = self.second.unwrap();
                              let third = self.third.unwrap();
                              self.first = None;
                              self.second = None;
                              self.third = None;
                              if b >= 0x30 && b <= 0x39 {
                                  let pointer = ((first as usize - 0x81) * (10 * 126 * 10)) +
                                                ((second as usize - 0x30) * (10 * 126)) +
                                                ((third as usize - 0x81) * 10) +
                                                (b as usize - 0x30);
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
                              self.pending_ascii = Some(second);
                              debug_assert!(third >= 0x81 && third <= 0xFE);
                              // `third` is guaranteed to be in the range
                              // that makes it become the new `self.first`.
                              self.first = Some(third);
                              // Now unread `b` and designate the previous
                              // `first` as being in error.
                              return (DecoderResult::Malformed(1, 2),
                                      unread_handle.unread(),
                                      destination_handle.written());
                          }
                          if self.second.is_some() {
                              if b >= 0x81 && b <= 0xFE {
                                  self.third = Some(b);
                                  continue;
                              }
                              let second = self.second.unwrap();
                              self.second = None;
                              self.first = None;
                              // We have an error. Let's inline what's going
                              // to happen when `second` is
                              // reprocessed. (`b` gets unread.)
                              debug_assert!(second >= 0x30 && second <= 0x39);
                              // `second` is guaranteed ASCII, so let's
                              // put it in `pending_ascii`
                              self.pending_ascii = Some(second);
                              // Now unread `b` and designate the previous
                              // `first` as being in error.
                              return (DecoderResult::Malformed(1, 1),
                                      unread_handle.unread(),
                                      destination_handle.written());
                          }
                          // self.first != 0
                          if b >= 0x30 && b <= 0x39 {
                              self.second = Some(b);
                              continue;
                          }
                          let lead = self.first.unwrap();
                          self.first = None;
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
                      check_space_astral,
                      decode_to_utf16_raw,
                      u16,
                      Utf16Destination);
}

pub struct Gb18030Encoder {
    extended: bool,
}

impl Gb18030Encoder {
    pub fn new(encoding: &'static Encoding, extended_range: bool) -> Encoder {
        Encoder::new(encoding,
                     VariantEncoder::Gb18030(Gb18030Encoder { extended: extended_range }))
    }

    pub fn max_buffer_length_from_utf16_without_replacement(&self, u16_length: usize) -> usize {
        if self.extended {
            u16_length * 4
        } else {
            u16_length * 2
        }
    }

    pub fn max_buffer_length_from_utf8_without_replacement(&self, byte_length: usize) -> usize {
        if self.extended {
            // 1 to 1
            // 2 to 2
            // 3 to 2
            // 2 to 4 (worst)
            // 3 to 4
            // 4 to 4
            byte_length * 2
        } else {
            // 1 to 1
            // 2 to 2
            // 3 to 2
            byte_length
        }
    }

    ascii_compatible_encoder_functions!({
                                            if bmp == 0xE5E5 {
                                                return (EncoderResult::unmappable_from_bmp(bmp),
                                                        source.consumed(),
                                                        handle.written());
                                            }
                                            if bmp == 0x20AC && !self.extended {
                                                handle.write_one(0x80u8)
                                            } else {
                                                let pointer = gb18030_encode(bmp);
                                                if pointer != usize::max_value() {
                                                    let lead = (pointer / 190) + 0x81;
                                                    let trail = pointer % 190;
                                                    let offset = if trail < 0x3F {
                                                        0x40
                                                    } else {
                                                        0x41
                                                    };
                                                    handle.write_two(lead as u8,
                                                                     (trail + offset) as u8)
                                                } else {
                                                    if !self.extended {
                                                        return (EncoderResult::unmappable_from_bmp(bmp),
                                                            source.consumed(),
                                                            handle.written());
                                                    }
                                                    let range_pointer = gb18030_range_encode(bmp);
                                                    let first = range_pointer / (10 * 126 * 10);
                                                    let rem_first = range_pointer % (10 * 126 * 10);
                                                    let second = rem_first / (10 * 126);
                                                    let rem_second = rem_first % (10 * 126);
                                                    let third = rem_second / 10;
                                                    let fourth = rem_second % 10;
                                                    handle.write_four((first + 0x81) as u8,
                                                                      (second + 0x30) as u8,
                                                                      (third + 0x81) as u8,
                                                                      (fourth + 0x30) as u8)
                                                }
                                            }
                                        },
                                        {
                                            if !self.extended {
                                                return (EncoderResult::Unmappable(astral),
                                                        source.consumed(),
                                                        handle.written());
                                            }
                                            let range_pointer = astral as usize +
                                                                (189000usize - 0x10000usize);
                                            let first = range_pointer / (10 * 126 * 10);
                                            let rem_first = range_pointer % (10 * 126 * 10);
                                            let second = rem_first / (10 * 126);
                                            let rem_second = rem_first % (10 * 126);
                                            let third = rem_second / 10;
                                            let fourth = rem_second % 10;
                                            handle.write_four((first + 0x81) as u8,
                                                              (second + 0x30) as u8,
                                                              (third + 0x81) as u8,
                                                              (fourth + 0x30) as u8)
                                        },
                                        bmp,
                                        astral,
                                        self,
                                        source,
                                        handle,
                                        copy_ascii_to_check_space_four,
                                        check_space_four,
                                        false);
}

// Any copyright to the test code below this comment is dedicated to the
// Public Domain. http://creativecommons.org/publicdomain/zero/1.0/

#[cfg(test)]
mod tests {
    use super::super::testing::*;
    use super::super::*;

    fn decode_gb18030(bytes: &[u8], expect: &str) {
        decode(GB18030, bytes, expect);
    }

    fn encode_gb18030(string: &str, expect: &[u8]) {
        encode(GB18030, string, expect);
    }

    fn encode_gbk(string: &str, expect: &[u8]) {
        encode(GBK, string, expect);
    }

    #[test]
    fn test_gb18030_decode() {
        // Empty
        decode_gb18030(b"", &"");

        // ASCII
        decode_gb18030(b"\x61\x62", "\u{0061}\u{0062}");

        // euro
        decode_gb18030(b"\x80", "\u{20AC}");
        decode_gb18030(b"\xA2\xE3", "\u{20AC}");

        // two bytes
        decode_gb18030(b"\x81\x40", "\u{4E02}");
        decode_gb18030(b"\x81\x7E", "\u{4E8A}");
        decode_gb18030(b"\x81\x7F", "\u{FFFD}\u{007F}");
        decode_gb18030(b"\x81\x80", "\u{4E90}");
        decode_gb18030(b"\x81\xFE", "\u{4FA2}");
        decode_gb18030(b"\xFE\x40", "\u{FA0C}");
        decode_gb18030(b"\xFE\x7E", "\u{E843}");
        decode_gb18030(b"\xFE\x7F", "\u{FFFD}\u{007F}");
        decode_gb18030(b"\xFE\x80", "\u{4723}");
        decode_gb18030(b"\xFE\xFE", "\u{E4C5}");

        // The difference from the original GB18030
        decode_gb18030(b"\xA3\xA0", "\u{3000}");
        decode_gb18030(b"\xA1\xA1", "\u{3000}");

        // 0xFF
        decode_gb18030(b"\xFF\x40", "\u{FFFD}\u{0040}");

        // Four bytes
        decode_gb18030(b"\x81\x30\x81\x30", "\u{0080}");
        decode_gb18030(b"\x81\x35\xF4\x37", "\u{E7C7}");
        decode_gb18030(b"\x81\x37\xA3\x30", "\u{2603}");
        decode_gb18030(b"\x94\x39\xDA\x33", "\u{1F4A9}");
        decode_gb18030(b"\xE3\x32\x9A\x35", "\u{10FFFF}");
        decode_gb18030(b"\xE3\x32\x9A\x36\x81\x30", "\u{FFFD}\u{0032}\u{309B8}");
        decode_gb18030(b"\xE3\x32\x9A\x36\x81\x40",
                       "\u{FFFD}\u{0032}\u{FFFD}\u{0036}\u{4E02}");
        decode_gb18030(b"\xE3\x32\x9A", "\u{FFFD}"); // not \u{FFFD}\u{0032}\u{FFFD} !

    }

    #[test]
    fn test_gb18030_encode() {
        // Empty
        encode_gb18030("", b"");

        // ASCII
        encode_gb18030("\u{0061}\u{0062}", b"\x61\x62");

        // euro
        encode_gb18030("\u{20AC}", b"\xA2\xE3");

        // two bytes
        encode_gb18030("\u{4E02}", b"\x81\x40");
        encode_gb18030("\u{4E8A}", b"\x81\x7E");
        encode_gb18030("\u{4E90}", b"\x81\x80");
        encode_gb18030("\u{4FA2}", b"\x81\xFE");
        encode_gb18030("\u{FA0C}", b"\xFE\x40");
        encode_gb18030("\u{E843}", b"\xFE\x7E");
        encode_gb18030("\u{4723}", b"\xFE\x80");
        encode_gb18030("\u{E4C5}", b"\xFE\xFE");

        // The difference from the original GB18030
        encode_gb18030("\u{E5E5}", b"&#58853;");
        encode_gb18030("\u{3000}", b"\xA1\xA1");

        // Four bytes
        encode_gb18030("\u{0080}", b"\x81\x30\x81\x30");
        encode_gb18030("\u{E7C7}", b"\x81\x35\xF4\x37");
        encode_gb18030("\u{2603}", b"\x81\x37\xA3\x30");
        encode_gb18030("\u{1F4A9}", b"\x94\x39\xDA\x33");
        encode_gb18030("\u{10FFFF}", b"\xE3\x32\x9A\x35");
    }

    #[test]
    fn test_gbk_encode() {
        // Empty
        encode_gbk("", b"");

        // ASCII
        encode_gbk("\u{0061}\u{0062}", b"\x61\x62");

        // euro
        encode_gbk("\u{20AC}", b"\x80");

        // two bytes
        encode_gbk("\u{4E02}", b"\x81\x40");
        encode_gbk("\u{4E8A}", b"\x81\x7E");
        encode_gbk("\u{4E90}", b"\x81\x80");
        encode_gbk("\u{4FA2}", b"\x81\xFE");
        encode_gbk("\u{FA0C}", b"\xFE\x40");
        encode_gbk("\u{E843}", b"\xFE\x7E");
        encode_gbk("\u{4723}", b"\xFE\x80");
        encode_gbk("\u{E4C5}", b"\xFE\xFE");

        // The difference from the original gb18030
        encode_gbk("\u{E5E5}", b"&#58853;");
        encode_gbk("\u{3000}", b"\xA1\xA1");

        // Four bytes
        encode_gbk("\u{0080}", b"&#128;");
        encode_gbk("\u{E7C7}", b"&#59335;");
        encode_gbk("\u{2603}", b"&#9731;");
        encode_gbk("\u{1F4A9}", b"&#128169;");
        encode_gbk("\u{10FFFF}", b"&#1114111;");
    }
}
