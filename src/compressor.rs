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

use crate::bit_buffer::BitBuffer;
use crate::interval::{Interval, IntervalState};
use crate::models::{Model, ModelCfi};
use crate::number_types::INTERVAL_BITS;
use crate::sim::Symbol;
use anyhow::Result;

pub struct Compressor<'a, M: Model> {
    /// Number of bits that were put aside in case of near-convergence, their value is unknown until
    /// a converging bit 'b' is found, and is equal to !b, repeated N times.
    outstanding_bits: usize,

    /// The buffer holding the output bits:
    output: BitBuffer,

    /// The interval that the compressor uses to represent the data it compresses.
    interval: Interval,

    /// The model in charge of calculating the probabilities of symbols appearing in the data. It
    /// can dramatically increase compression rate.
    model: &'a mut M,
}

impl<'a, M: Model> Compressor<'a, M> {
    /// Creates a new compressor object from a statistical model.
    ///
    /// Note that if the model implements the `update` and `flush` functions, it is the
    /// **responsibility of the CALLER** to make sure the state of the model is not affected by
    /// previous operations (i.e: call the `flush` function if needed).
    pub fn new(model: &'a mut M) -> Self {
        Self {
            outstanding_bits: 0,
            output: BitBuffer::new(),
            interval: Interval::full_interval(),
            model,
        }
    }

    /// When the interval's boundaries finally converge on a bit, the values of all remaining
    /// outstanding bits are also known (it's the inverse of the given bit).
    ///
    /// This helper function outputs all outstanding bits to the bitbuffer, followed by the given
    /// bit. It is also responsible for setting `self.outstanding_bits` to 0.
    fn output_with_outstanding(&mut self, bit: bool) {
        self.output.append(bit);

        self.output.append_repeated(!bit, self.outstanding_bits);
        self.outstanding_bits = 0;
    }

    /// Processes the state of the saved interval until it is in a no-convergence state.
    fn process_interval_state(&mut self) -> Result<()> {
        // Process the state until the interval is non-converging:
        loop {
            let (low, high) = match self.interval.get_state() {
                IntervalState::Converging(bit) => {
                    self.output_with_outstanding(bit);

                    // Get rid of the converging bit in the boundaries, shift 1 in for high:
                    let low = self.interval.low() << 1u8;
                    let high = (self.interval.high() << 1u8) | 1u8;
                    (low, high)
                }
                IntervalState::NearConvergence => {
                    // Increase the outstanding bits counter, shift out the second MSBs, and shift
                    // in a 1 bit for high:
                    self.outstanding_bits += 1;

                    let half = self.interval.system().half();
                    let low = (self.interval.low() << 1u8) ^ half;
                    let high = (self.interval.high() << 1u8) | (*half + 1);

                    (low, high)
                }
                IntervalState::NoConvergence => break Ok(()),
            };
            self.interval.set_boundaries(low, high)?;
        }
    }

    /// Compresses the given symbol using arithmetic coding. This function **CANNOT** be used with
    /// an escape CFI, as they are only supposed to be used during decompression.
    ///
    /// Since arithmetic coding may compress symbols into a number of bits indivisible by 8, the
    /// function returns an iterator over any COMPLETE bytes of the compression (which may be empty
    /// if the compression used less than 8 bits).<br>
    /// To retrieve the leftover bits and finish compression, call the `finalize` function
    pub fn load_symbol(&mut self, symbol: Symbol) -> Result<impl Iterator<Item = u8>> {
        let cfi = self.model.get_cfi(symbol)?;
        self.model.update(symbol, &cfi)?;

        match cfi {
            ModelCfi::IndexCfi(cfi) => {
                self.interval.update(cfi);
                self.process_interval_state()?;
            }
            // If it's an escape CFI, repeatedly load the symbol:
            ModelCfi::EscapeCfi(cfi) => {
                self.interval.update(cfi);
                self.process_interval_state()?;
                return self.load_symbol(symbol);
            }
        }
        Ok(self.output.get_complete_bytes())
    }

    /// Ends the compression of any bits left over from previous operations, outputting them as an
    /// iterator of bytes.
    pub fn finalize(mut self) -> impl Iterator<Item = u8> {
        // When all symbols are loaded, the possible interval boundaries are:
        // - [01yyy, 11xxx)
        // - [00yyy, 11xxx)
        // - [00yyy, 10xxx)
        // So we must insert '01' if low is '00', and '10' if low is '01'. Along with those, any
        // pending near-convergence bits must be inserted as well. A simple way of doing it is just
        // adding 1 to the near-convergence counter and insert the value of low's second MSB:
        self.outstanding_bits += 1;
        self.output_with_outstanding(*((self.interval.low() >> (INTERVAL_BITS - 2)) & 1u8) == 1);

        self.output
            .get_complete_bytes()
            .chain(self.output.get_leftover_bits())
    }
}
