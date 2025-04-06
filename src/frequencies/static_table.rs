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

use super::Frequency;
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
