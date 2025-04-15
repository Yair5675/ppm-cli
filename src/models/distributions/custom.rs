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
use crate::sim::{Symbol, SymbolIndexMapping};
use anyhow::{anyhow, Result};
use log::{error, warn};

/// A probability model with a custom distribution for indices.
pub struct CustomDistributionModel<SIM: SymbolIndexMapping> {
    /// The table holding all frequencies
    table: StaticFrequencyTable,
    /// A mapping between symbols and indices in the table
    sim: SIM,
}

impl<SIM: SymbolIndexMapping> CustomDistributionModel<SIM> {
    /// Creates a model with a custom distribution for indices.
    ///
    /// ## Parameters
    /// * sim: A mapping between symbols and indices in _frequencies_.
    /// * frequencies: A slice of symbol frequencies. The mapping between the symbols and the
    ///   frequencies is determined by _sim_.
    /// ## Potential Failures
    /// If the sum of the frequencies exceeds Frequency::max(), an error will be returned.
    /// If the length of _frequencies_ does not equal `sim.supported_symbols_count()`, an error will
    /// be returned.
    pub fn new(sim: SIM, frequencies: &[Frequency]) -> Result<Self> {
        let supported_symbols = sim.supported_symbols_count();
        if supported_symbols != frequencies.len() {
            let msg = format!(
                "Given SIM supports a different amount of symbols than provided in frequencies\
                     (supported = {}, frequencies length = {}",
                supported_symbols,
                frequencies.len()
            );
            error!("{}", msg);
            Err(anyhow!(msg))
        } else {
            Ok(Self {
                sim,
                table: StaticFrequencyTable::new(frequencies)?,
            })
        }
    }
}

impl<SIM: SymbolIndexMapping> Model for CustomDistributionModel<SIM> {
    fn get_cfi(&self, symbol: Symbol) -> Result<ModelCfi, ModelCfiError> {
        let index = self.sim.get_index(&symbol).ok_or_else(|| {
            error!(
                "Custom Distribution Model: Unsupported symbol \"{}\" given",
                symbol
            );
            ModelCfiError::UnsupportedSymbol(symbol)
        })?;

        self.table
            .get_cfi(index)
            .map(|cfi| {
                if symbol.is_escape() {
                    ModelCfi::EscapeCfi(cfi)
                } else {
                    ModelCfi::IndexCfi(cfi)
                }
            })
            .ok_or_else(|| {
                warn!(
                    "Custom Distribution Model: Empty CFI assigned to queried symbol {}",
                    symbol
                );
                ModelCfiError::EmptyCfi { symbol }
            })
    }

    fn get_symbol(&self, cumulative_frequency: Frequency) -> Option<Symbol> {
        self.table
            .get_index(cumulative_frequency)
            .and_then(|index| self.sim.get_symbol(index))
    }

    fn get_total(&self) -> Frequency {
        self.table.get_total()
    }
}
