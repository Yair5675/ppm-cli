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

use crate::number_types::ConstrainedNum;
use log::info;
use thiserror::Error;

/// In Arithmetic Coding, we use an integer representation of fractional values to achieve
/// pseudo-infinite precision with finite bits.
///
/// An integer's bits are treated like bits after a decimal point.<br>
/// For example: 0101<sub>2</sub> (5<sub>10</sub>) would be treated as if it were 0.0101<sub>2</sub>
/// (0.3125<sub>10</sub>).
///
/// BitsSystem is a struct holding special constants in this integer representation that are vital
/// to Arithmetic Coding. The generic constant BITS is the number of bits used in the system.
pub struct BitsSystem<const BITS: u32> {
    /// Largest possible value in the integer representation, 0.11..1:
    max: ConstrainedNum<BITS>,
    /// Half in the integer representation, 0.10..0:
    half: ConstrainedNum<BITS>,
    /// One fourth in the integer representation, 0.010..0:
    one_fourth: ConstrainedNum<BITS>,
    /// Three fourths in the integer representation, 0.110..0:
    three_fourths: ConstrainedNum<BITS>,
}

impl<const BITS: u32> BitsSystem<BITS> {
    /// Creates a new bits system. Will fail if _BITS_ is less than 2.
    pub fn new() -> Result<Self, NotEnoughBitsForSystemError> {
        // Check the BITS:
        if BITS < 2 {
            return Err(NotEnoughBitsForSystemError { bits: BITS });
        }
        // Create all constants, ConstraintNum will take care of everything
        let max = ConstrainedNum::max();
        let half = max >> 1u8;
        let one_fourth = half >> 1u8;
        let three_fourths = half | one_fourth;

        info!("Creating a Bits System of {} bits", BITS);

        Ok(Self {
            max,
            half,
            one_fourth,
            three_fourths,
        })
    }

    pub fn max(&self) -> ConstrainedNum<BITS> {
        self.max
    }

    pub fn half(&self) -> ConstrainedNum<BITS> {
        self.half
    }

    pub fn one_fourth(&self) -> ConstrainedNum<BITS> {
        self.one_fourth
    }

    pub fn three_fourths(&self) -> ConstrainedNum<BITS> {
        self.three_fourths
    }
}

#[derive(Debug, Error)]
#[error("Every Bits System must have at least 2 bits ({bits} were given)")]
pub struct NotEnoughBitsForSystemError {
    bits: u32,
}
