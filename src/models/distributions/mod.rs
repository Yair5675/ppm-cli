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

use super::{Model, ModelCFI};
use crate::frequencies::{Cfi, Frequency};
use crate::number_types::{BitsConstraintError, CalculationsType, FREQUENCY_BITS};
use anyhow::Result;
use std::num::NonZero;
use thiserror::Error;

/// A probability model that assigns each index an equal probability
pub struct UniformDistributionModel {
    /// Number of symbols the model supports, saved as a frequency to avoid converting it to one
    num_symbols: Frequency,
    /// The index assigned to the escape symbol
    escape_idx: Option<usize>,
}

impl UniformDistributionModel {
    /// Initializes a UniformDistributionModel without an escape index (see `new_with_escape` if you
    /// want an escape index).
    ///
    /// ## Parameters:
    /// * num_symbols - Number of symbols in the model, cannot be zero.
    ///
    /// ## Possible Failures:
    /// If num_symbols is larger than the number of allowed frequencies (i.e: Frequency::max()) an
    /// error will be returned.
    pub fn new(num_symbols: NonZero<usize>) -> Result<Self, UniformModelInitError> {
        Ok(Self {
            num_symbols: Frequency::new(num_symbols.get() as CalculationsType)?,
            escape_idx: None,
        })
    }

    /// Initializes a UniformDistributionModel with an assigned escape index.
    ///
    /// ## Parameters:
    /// * num_symbols - Number of symbols in the model **including the escape symbol**, cannot be 0.
    /// * escape_idx - The index chosen for the escape symbol.
    ///
    /// ## Possible Failures:
    /// If num_symbols is larger than the number of allowed frequencies (i.e: Frequency::max()) an
    /// error will be returned.<br>
    /// If the escape index is larger than or equal to the number of symbols, an error will be
    /// returned.
    pub fn new_with_escape(
        num_symbols: NonZero<usize>,
        escape_idx: usize,
    ) -> Result<Self, UniformModelInitError> {
        if escape_idx >= num_symbols.get() {
            Err(UniformModelInitError::EscapeIndexTooLarge(
                num_symbols.get(),
                escape_idx,
            ))
        } else {
            Ok(Self {
                num_symbols: Frequency::new(num_symbols.get() as CalculationsType)?,
                escape_idx: Some(escape_idx),
            })
        }
    }
}

#[derive(Debug, Error)]
pub enum UniformModelInitError {
    #[error("The number of symbols in the model exceeds Frequency::max()")]
    TooManySymbolsError(#[from] BitsConstraintError<FREQUENCY_BITS>),

    #[error("The number of symbols in the model is {0}, yet the index chosen for the escape symbol is {1}")]
    EscapeIndexTooLarge(usize, usize),
}

impl Model for UniformDistributionModel {
    fn get_cfi(&self, index: usize) -> Result<ModelCFI> {
        // Check index:
        let index = index as CalculationsType;
        if index >= *self.num_symbols {
            return Ok(ModelCFI::UnsupportedIndex);
        }
        // Since each index is assigned a probability of 1, its CFI can be easily computed:
        let cfi = Cfi {
            start: Frequency::new(index)?,
            end: Frequency::new(index + 1)?,
            total: self.get_total(),
        };

        Ok(match self.escape_idx {
            Some(escape_idx) if escape_idx == index as usize => ModelCFI::EscapeCfi(cfi),
            _ => ModelCFI::IndexCfi(cfi),
        })
    }

    fn get_symbol(&self, cumulative_frequency: Frequency) -> Option<usize> {
        // Since each index gets an equal probability, the cumulative frequency is equal to the
        // index itself:
        if cumulative_frequency >= self.num_symbols {
            None
        } else {
            Some(*cumulative_frequency as usize)
        }
    }

    fn get_total(&self) -> Frequency {
        self.num_symbols
    }
}
