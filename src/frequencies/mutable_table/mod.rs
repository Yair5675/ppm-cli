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

mod fenwick;

use self::fenwick::FenwickTree;
use super::{Cfi, Frequency, FrequencyTable};

use crate::number_types::CalculationsType;
use anyhow::{Context, Result};

/// A frequency table which can be mutated
pub struct MutableFrequencyTable {
    /// The frequencies, stored in a fenwick tree for efficient querying and mutating (O(log n))
    fenwick: FenwickTree,

    /// The total cumulative frequency. It can be computed from the fenwick tree, but saving it is
    /// easy and makes its query more efficient
    total: Frequency,
}

impl MutableFrequencyTable {
    /// Creates a mutable frequency table from the frequencies provided here.<br>
    /// The new table's length will be the length of the provided slice.
    ///
    /// The frequencies provided here should not be cumulative, and the function will fail if at
    /// any point the sum of the slice's frequencies exceeds the allowed bits.
    pub fn new(frequencies: &[Frequency]) -> Result<Self> {
        let fenwick = FenwickTree::from(
            &frequencies
                .iter()
                .map(|f| **f)
                .collect::<Vec<CalculationsType>>(),
        );
        let total = Frequency::new(fenwick.get_sum(fenwick.len()))
            .context("Failed to create mutable table, overflow occurred for total")?;

        Ok(Self {
            fenwick,
            total,
        })
    }

    /// Adds a certain amount to the frequency at the given index in the table.
    ///
    /// If the result of that addition exceeds the bits allowed for a frequency, it is not saved in
    /// the table.
    pub fn add_frequency(&mut self, index: usize, amount: Frequency) {
        // Since `total` is the largest, if adding to it fails adding to anything else will too:
        if let Ok(new_total) = Frequency::new(*self.total + *amount) {
            self.total = new_total;
            self.fenwick.add(index, *amount);
        }
    }
}

impl FrequencyTable for MutableFrequencyTable {
    fn get_cfi(&self, index: usize) -> Option<Cfi> {
        if index < self.fenwick.len() - 1 {
            Some(Cfi {
                // Invariants ensure unwrapping frequencies is safe:
                start: Frequency::new(self.fenwick.get_sum(index))
                    .expect("MutableFrequencyTable invariant violated"),
                end: Frequency::new(self.fenwick.get_sum(index + 1))
                    .expect("MutableFrequencyTable invariant violated"),
                total: self.total,
            })
        } else {
            None
        }
    }

    fn get_index(&self, cumulative_frequency: Frequency) -> Option<usize> {
        // Implement binary search (get_sum doesn't include the index so only decrement 1 from len):
        let (mut left, mut right) = (0, self.fenwick.len() - 1);
        let cumulative_frequency = *cumulative_frequency;

        while left <= right {
            let middle = (left + right) >> 1;

            // Check lower bound:
            if cumulative_frequency < self.fenwick.get_sum(middle) {
                right = middle - 1;
            }
            // Check upper bound:
            else if cumulative_frequency >= self.fenwick.get_sum(middle + 1) {
                left = middle + 1;
            }
            // Spot on!
            else {
                return Some(middle);
            }
        }

        None
    }

    fn get_total(&self) -> Frequency {
        self.total
    }
}
