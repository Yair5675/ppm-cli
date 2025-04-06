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

use crate::number_types::CalculationsType;

/// Computes the least significant set bit of a number
fn lsb(n: usize) -> usize {
    let n = n as isize;
    (n & -n) as usize
}

/// A data structure that allows efficient calculation of cumulative summation AND mutation of
/// values
pub struct FenwickTree {
    // Values of the tree, allow for quick computation of cumulative sum AND mutation of values.
    // It uses Box since we never append/remove elements, only mutate them:
    data: Box<[CalculationsType]>,
}

impl FenwickTree {
    /// Creates a new, empty FenwickTree with the given size
    pub fn new(size: usize) -> Self {
        // Fenwick trees index calculations depend on the indices starting at 1, so add an extra
        // element to ensure this:
        Self {
            data: vec![0; size + 1].into_boxed_slice()
        }
    }
}
