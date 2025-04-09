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
