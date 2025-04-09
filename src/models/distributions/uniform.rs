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

use crate::frequencies::{Cfi, Frequency};
use crate::models::{Model, ModelCfi, ModelCfiError};
use crate::number_types::CalculationsType;
use crate::sim::{Symbol, SymbolIndexMapping};

/// A probability model that assigns each symbol an equal probability
pub struct UniformDistributionModel<SIM: SymbolIndexMapping>(SIM);

impl<SIM: SymbolIndexMapping> UniformDistributionModel<SIM> {
    /// Initializes a UniformDistributionModel with a given Symbol-Index Mapping.
    ///
    /// ## Parameters:
    /// * sim - A mapping between symbols and indices.
    pub fn new(sim: SIM) -> Self {
        Self(sim)
    }
}

impl<SIM: SymbolIndexMapping> Model for UniformDistributionModel<SIM> {
    fn get_cfi(&self, symbol: Symbol) -> Result<ModelCfi, ModelCfiError> {
        // Get index:
        let index = self
            .0
            .get_index(&symbol)
            .ok_or(ModelCfiError::UnsupportedSymbol(symbol))?;

        // Since each index is assigned a probability of 1, its CFI can be easily computed:
        let cfi = {
            let index = index as CalculationsType;
            Cfi {
                // A SIM can have a maximum of UNIQUE_SYMBOLS_AMOUNT which is far less than
                // Frequency::max()
                start: Frequency::new(index)
                    .expect("SIM invariant broke, index too large to become frequency"),
                end: Frequency::new(index + 1)
                    .expect("SIM invariant broke, index + 1 too large to become frequency"),
                total: self.get_total(),
            }
        };

        if cfi.start == cfi.end {
            Err(ModelCfiError::EmptyCfi { symbol })
        } else if symbol.is_escape() {
            Ok(ModelCfi::EscapeCfi(cfi))
        } else {
            Ok(ModelCfi::IndexCfi(cfi))
        }
    }

    fn get_symbol(&self, cumulative_frequency: Frequency) -> Option<Symbol> {
        // Since each index gets an equal probability, the cumulative frequency is equal to the
        // index itself:
        self.0.get_symbol(*cumulative_frequency as usize)
    }

    fn get_total(&self) -> Frequency {
        // A SIM can have a maximum of UNIQUE_SYMBOLS_AMOUNT which is far less than
        // Frequency::max()
        Frequency::new(self.0.supported_symbols_count() as CalculationsType)
            .expect("SIM invariant broke, supported symbols count too large to become frequency")
    }
}
