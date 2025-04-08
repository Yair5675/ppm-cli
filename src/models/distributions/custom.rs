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

use crate::frequencies::static_table::StaticFrequencyTable;
use crate::frequencies::{Frequency, FrequencyTable};
use crate::models::{Model, ModelCfi, ModelCfiError};
use anyhow::Result;

// TODO: Consider adding escape support later
/// A probability model with a custom distribution for indices. This model does **not** support
/// escape symbols currently.
pub struct CustomDistributionModel {
    /// The table holding all frequencies
    table: StaticFrequencyTable,
    /// Number of indices in the model:
    num_symbols: usize,
}

impl CustomDistributionModel {
    /// Creates a model with a custom distribution for indices. If the sum of the frequencies
    /// exceeds Frequency::max(), an error will be returned.
    pub fn new(frequencies: &[Frequency]) -> Result<Self> {
        Ok(Self {
            num_symbols: frequencies.len(),
            table: StaticFrequencyTable::new(frequencies)?,
        })
    }
}

impl Model for CustomDistributionModel {
    fn get_cfi(&self, index: usize) -> Result<ModelCfi, ModelCfiError> {
        if index >= self.num_symbols {
            Err(ModelCfiError::UnsupportedIndex(index))
        } else {
            self.table
                .get_cfi(index)
                .map(ModelCfi::IndexCfi)
                .ok_or(ModelCfiError::EmptyCfi { index })
        }
    }

    fn get_symbol(&self, cumulative_frequency: Frequency) -> Option<usize> {
        self.table.get_index(cumulative_frequency)
    }

    fn get_total(&self) -> Frequency {
        self.table.get_total()
    }
}
