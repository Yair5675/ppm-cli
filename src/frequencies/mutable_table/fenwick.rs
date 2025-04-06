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
            data: vec![0; size + 1].into_boxed_slice(),
        }
    }

    /// Computes the cumulative sum of all values up to (but not including) the given index.<br>
    /// This function's time complexity is **O(log n)**.
    pub fn get_sum(&self, mut index: usize) -> CalculationsType {
        let mut sum = 0;
        while 0 < index && index < self.data.len() {
            sum += self.data[index];
            index -= lsb(index);
        }
        sum
    }

    /// Adds a certain amount to an index in the tree in **O(log n)** time complexity.
    pub fn add(&mut self, mut index: usize, amount: CalculationsType) {
        // Shift the index by one since the fenwick tree is 1-based:
        index += 1;
        while index < self.data.len() {
            self.data[index] += amount;
            index += lsb(index);
        }
    }
}
