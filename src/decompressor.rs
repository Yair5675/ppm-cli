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

use crate::interval::{Interval, IntervalState};
use crate::models::Model;
use crate::number_types::{ConstrainedNum, INTERVAL_BITS};
use anyhow::Result;

pub struct Decompressor<'a, M: Model, I: Iterator<Item = bool>> {
    /// Iterator over compressed bits
    bits_iter: I,

    /// Current interval in the decompression stage
    interval: Interval,

    /// Current value from the input, used to locate the next symbol's interval within the current
    /// interval
    value: ConstrainedNum<INTERVAL_BITS>,

    /// Probability model, must be the same as the compressor's model for the decompression to work
    model: &'a mut M,
}

impl<'a, M: Model, I: Iterator<Item = bool>> Decompressor<'a, M, I> {
    /// Creates a new decompressor object from a statistical model and a bits iterator.
    ///
    /// Note that if the model implements the `update` and `flush` functions, it is the
    /// **responsibility of the CALLER** to make sure the state of the model is not affected by
    /// previous operations (i.e: call the `flush` function if needed).
    pub fn new(model: &'a mut M, compressed_bits: I) -> Self {
        let mut this = Self {
            bits_iter: compressed_bits,
            interval: Interval::full_interval(),
            value: ConstrainedNum::zero(),
            model,
        };

        // Load bits into value:
        this.load_bits_to_value(INTERVAL_BITS);
        this
    }

    /// Processes the state of the interval until it is non-converging
    pub fn process_interval_state(&mut self) -> Result<()> {
        loop {
            // Simply copy the compression stage:
            let (low, high) = match self.interval.get_state() {
                // Remove MSB:
                IntervalState::Converging(_) => {
                    self.load_bits_to_value(1);
                    let low = self.interval.low() << 1u8;
                    let high = (self.interval.high() << 1u8) | 1u8;
                    
                    (low, high)
                }
                // Remove second MSB:
                IntervalState::NearConvergence => {
                    let half = self.interval.system().half();
                    let low = (self.interval.low() << 1u8) ^ half;
                    let high = (self.interval.high() << 1u8) | (*half + 1);

                    // Since value < high, it must start with 01 like low:
                    self.value = ((self.value << 1u8) ^ half) | self.get_next_bit();

                    (low, high)
                }

                IntervalState::NoConvergence => break Ok(()),
            };
            self.interval
                .set_low(low)
                .and_then(|_| self.interval.set_high(high))?
        }
    }

    /// Retrieve the next bit from the iterator as a ConstrainedNum, or returns 0 if `bits_iter` is
    /// empty.
    fn get_next_bit(&mut self) -> ConstrainedNum<INTERVAL_BITS> {
        match self.bits_iter.next() {
            None => ConstrainedNum::zero(),
            Some(b) => b.into(),
        }
    }

    /// Shifts bits from `bits_iter` into `value`. If `bits_iter` is empty, zero bits will be
    /// inserted into `value`.
    fn load_bits_to_value(&mut self, bits_num: u32) {
        for _ in 0..bits_num {
            self.value = (self.value << 1u8) | self.get_next_bit();
        }
    }
}
