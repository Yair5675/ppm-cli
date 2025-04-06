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
        let mut accum = Frequency::zero();
        let mut current_idx = 1; // Keep first index 0
        let mut fenwick = FenwickTree::new(frequencies.len() + 1);

        for frequency in frequencies.iter() {
            accum = Frequency::new(*accum + **frequency).context(format!(
                "Failed to create mutable table, index {} caused an overflow",
                current_idx - 1
            ))?;

            fenwick.add(current_idx, *accum);
            current_idx += 1;
        }

        Ok(Self {
            fenwick,
            total: accum,
        })
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
