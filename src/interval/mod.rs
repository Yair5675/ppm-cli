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

mod bits_system;

use crate::number_types::{ConstrainedNum, INTERVAL_BITS};
pub use self::bits_system::BitsSystem;

/// Boundary of an interval, an integer representation of a fractional value between 0 and 1.
pub type IntervalBoundary = ConstrainedNum<INTERVAL_BITS>;

/// An interval containing values between 0 and 1 using the integer representation for fractional
/// values.
pub struct Interval {
    /// Lower boundary of the interval
    low: IntervalBoundary,
    /// Upper boundary of the interval
    high: IntervalBoundary,
    /// The BitsSystem of the interval
    system: BitsSystem<INTERVAL_BITS>,
}
