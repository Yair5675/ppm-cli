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

mod symbol;

use crate::number_types::{ConstrainedNum, FREQUENCY_BITS};

/// Number type for all frequencies, used to limit a frequency's bits
pub type Frequency = ConstrainedNum<FREQUENCY_BITS>;

/// A struct describing the Cumulative Frequency Interval of a symbol
#[derive(Debug, Clone)]
pub struct Cfi {
    pub start: Frequency,
    pub end: Frequency,
    pub total: Frequency,
}
