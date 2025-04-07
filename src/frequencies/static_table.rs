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

use super::{Cfi, Frequency, FrequencyTable};
use anyhow::{Context, Result};

/// A frequency table whose values cannot be updated after initialization
pub struct StaticFrequencyTable {
    /// The cumulative frequencies, stored in a box for memory optimization reasons
    cum_freqs: Box<[Frequency]>,
}

impl StaticFrequencyTable {
    /// Creates a static frequency table from the frequencies provided here.<br>
    /// The new table's length will be the length of the provided slice.
    ///
    /// The frequencies provided here should not be cumulative, and the function will fail if at
    /// any point the sum of the slice's frequencies exceeds the allowed bits.
    pub fn new(frequencies: &[Frequency]) -> Result<Self> {
        // Initialize the cumulative frequencies vector with 0 as the first CFI's start value:
        let mut accum = 0;
        let mut cum_freqs = Vec::with_capacity(frequencies.len() + 1);
        cum_freqs.push(Frequency::zero());

        for (idx, frequency) in frequencies.iter().enumerate() {
            // Calculate cumulative and catch any overflow:
            accum += **frequency;
            cum_freqs.push(Frequency::new(accum).context(format!(
                "Failed to create static table, index {idx} caused an overflow"
            ))?);
        }

        Ok(Self {
            cum_freqs: cum_freqs.into_boxed_slice(),
        })
    }
}

impl FrequencyTable for StaticFrequencyTable {
    fn get_cfi(&self, index: usize) -> Option<Cfi> {
        self.cum_freqs
            // Get start and end of the CFI:
            .get(index)
            .zip(self.cum_freqs.get(index + 1))
            // Map to CFI, check if start is equal to end:
            .and_then(|(&start, &end)| {
                if start == end {
                    None
                } else {
                    Some(Cfi {
                        start,
                        end,
                        total: self.get_total(),
                    })
                }
            })
    }

    fn get_index(&self, cumulative_frequency: Frequency) -> Option<usize> {
        // Use binary search since all frequencies are non-negative and therefor all cumulative
        // frequencies are sorted:
        let (mut left, mut right) = (0, self.cum_freqs.len() - 2);

        while left <= right {
            let middle = (left + right) >> 1;

            // Check lower bound:
            if cumulative_frequency < self.cum_freqs[middle] {
                right = middle - 1;
            }
            // Check upper bound:
            else if cumulative_frequency >= self.cum_freqs[middle + 1] {
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
        // Cumulative sum of all frequencies is always the last index in the box:
        self.cum_freqs[self.cum_freqs.len() - 1]
    }
}
