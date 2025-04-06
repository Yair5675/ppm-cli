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

// This file describes the different types used to represent multiple numerical values in the
// project

/// Maximum number of bits that represent the frequency of a symbol in the data
pub const FREQUENCY_BITS: u32 = 31;

/// Maximum number of bits an interval value can have (must satisfy:
/// `INTERVAL_BITS >= 2 + FREQUENCY_BITS` to provide the decoder + model enough room to decode all
/// symbols, even rare ones)
pub const INTERVAL_BITS: u32 = 33;

/// The type assigned to perform all calculations, avoids both overflow and underflow (must satisfy:
/// `CALCULATIONS_TYPE_BITS >= INTERVAL_BITS + FREQUENCY_BITS`
pub type CalculationsType = u64;
