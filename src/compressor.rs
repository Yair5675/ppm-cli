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
use crate::interval::Interval;
use crate::models::Model;

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
}
