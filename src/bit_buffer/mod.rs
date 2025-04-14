// PPM-CLI: A Command-Line Interface for compressing data using Arithmetic Coding + Prediction by
// Partial Matching
// Copyright (C) 2025  Yair Ziv
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub mod bit_iter;
#[cfg(test)]
mod unit_tests;

use log::{debug, info};
use std::collections::LinkedList;

/// A buffer dedicated to bit storage
#[derive(Debug)]
pub struct BitBuffer {
    full_bytes: LinkedList<u8>,
    // Bits will be added to this byte, from its MSB to the LSB to preserve insertion order
    current_byte: u8,
    current_idx: usize,
}

impl BitBuffer {
    /// Initializes an empty BitBuffer.
    pub fn new() -> Self {
        info!("Created new BitBuffer");
        Self {
            full_bytes: LinkedList::new(),
            current_byte: 0,
            current_idx: 0,
        }
    }

    /// Inserts a single bit to the end of the buffer.
    pub fn append(&mut self, bit: bool) {
        debug!("Appending bit to buffer: {}", if bit { 1 } else { 0 });

        if bit {
            self.current_byte |= 1 << (7 - self.current_idx);
        }
        self.current_idx += 1;

        // If the current byte is full, save it:
        if self.current_idx >= 8 {
            self.save_current_byte();
        }
    }

    /// Inserts a single bit to the end of the buffer multiple times. This method is more efficient
    /// than calling `append` in a loop.
    ///
    /// Note that specifying 0 repetitions is allowed, and won't change the buffer.
    pub fn append_repeated(&mut self, bit: bool, mut repetitions: usize) {
        debug!(
            "Appending bits to buffer: {} {} time(s)",
            if bit { 1 } else { 0 },
            repetitions
        );
        let bit_repeated = if bit { u8::MAX } else { 0 };

        while self.current_idx + repetitions >= 8 {
            // Add to the current byte, then save it:
            self.current_byte |= bit_repeated >> self.current_idx;
            repetitions -= 8 - self.current_idx;
            self.save_current_byte();
        }

        // Insert leftover bits to current_byte if needed, update current_idx:
        if repetitions > 0 && bit {
            self.current_byte |= u8::MAX << (8 - repetitions);
        }
        self.current_idx += repetitions;
    }

    /// Saves the current byte into the `full_bytes` list, and resets both `current_idx` and
    /// `current_idx`.
    fn save_current_byte(&mut self) {
        debug!("Saving byte into BitBuffer: {:08b}", self.current_byte);
        self.full_bytes.push_back(self.current_byte);
        self.current_byte = 0;
        self.current_idx = 0;
    }

    /// Extracts full bytes from the buffer and returns them as an iterator. If there aren't enough
    /// bits in the buffer to form a single byte, the iterator will be empty.<br>
    /// To remove ambiguity: **The bytes will not remain in the buffer after calling this
    /// function**.
    pub fn get_complete_bytes(&mut self) -> impl Iterator<Item = u8> {
        debug!(
            "Removing {} complete bytes from buffer",
            self.full_bytes.len()
        );
        std::mem::take(&mut self.full_bytes).into_iter()
    }

    /// Returns the number of **bits** in the buffer
    pub fn len(&self) -> usize {
        8 * self.full_bytes.len() + self.current_idx
    }

    /// If the number of bits in the buffer isn't divisible by 8, there will exist 'leftover' bits,
    /// which cannot be turned into a byte without padding.
    ///
    /// The function will return those leftover bits, padded with zeroes. Those zero bits will be
    /// added to the right of the leftover bits (i.e: the least significant bit of the returned byte
    /// is guaranteed to be a padding zero bit).
    /// If no leftover bits exist, the function returns None.
    ///
    /// Note that this operation does **not** remove those leftover bits from the buffer.
    pub fn get_leftover_bits(&self) -> Option<u8> {
        debug!(
            "Leftover bits were requested. Do they exist: {}",
            self.current_idx > 0
        );
        if self.current_idx > 0 {
            Some(self.current_byte)
        } else {
            None
        }
    }
}

impl From<&[u8]> for BitBuffer {
    fn from(value: &[u8]) -> Self {
        debug!("Creating BitBuffer from slice of {} bytes", value.len());
        // Since whose are all full bytes, add them directly to the full_bytes list:
        Self {
            full_bytes: LinkedList::from_iter(value.iter().copied()),
            current_byte: 0,
            current_idx: 0,
        }
    }
}

impl From<Vec<u8>> for BitBuffer {
    fn from(value: Vec<u8>) -> Self {
        debug!("Creating BitBuffer from Vec of {} bytes", value.len());
        // Since whose are all full bytes, add them directly to the full_bytes list:
        Self {
            full_bytes: LinkedList::from_iter(value),
            current_byte: 0,
            current_idx: 0,
        }
    }
}
