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

use std::num::NonZero;
use thiserror::Error;

/// A probability model that assigns each index an equal probability
pub struct UniformDistributionModel {
    /// Number of symbols the model supports
    num_symbols: NonZero<usize>,
    /// The index assigned to the escape symbol
    escape_idx: Option<usize>,
}

impl UniformDistributionModel {
    /// Initializes a UniformDistributionModel without an escape index (see `new_with_escape` if you
    /// want an escape index).
    /// 
    /// ## Parameters:
    /// * num_symbols - Number of symbols in the model, cannot be zero.
    pub fn new(num_symbols: NonZero<usize>) -> Self {
        Self {
            num_symbols, escape_idx: None
        }
    }
    
    /// Initializes a UniformDistributionModel with an assigned escape index.
    /// 
    /// ## Parameters:
    /// * num_symbols - Number of symbols in the model **including the escape symbol**, cannot be 0.
    /// * escape_idx - The index chosen for the escape symbol.
    /// 
    /// ## Possible Failures:
    /// The function will return Err(EscapeIndexTooLarge) if escape_idx >= num_symbols
    pub fn new_with_escape(num_symbols: NonZero<usize>, escape_idx: usize) -> Result<Self, EscapeIndexTooLarge> {
        if escape_idx >= num_symbols.get() {
            Err(EscapeIndexTooLarge(num_symbols.get(), escape_idx))
        } else {
            Ok(Self {
                num_symbols, escape_idx: Some(escape_idx)
            })
        }
    }
}

#[derive(Debug, Error)]
#[error("The number of symbols in the model is {0}, yet the index chosen for the escape symbol is {1}")]
pub struct EscapeIndexTooLarge(usize, usize);
