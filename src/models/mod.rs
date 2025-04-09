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

pub mod distributions;

use crate::frequencies::{Cfi, Frequency};
use crate::sim::Symbol;
use anyhow::Result;
use thiserror::Error;

/// Outputs of a probability model, wrapping CFIs to provide information for model-updating.
pub enum ModelCfi {
    /// Normal CFI, represents a regular symbol/index
    IndexCfi(Cfi),

    /// Either a CFI of an escape symbol, OR a CFI given by the model to alert the decompression
    /// of something. If received during the compression of a non-escape symbol, the compressor
    /// needs to re-compress the symbol until the model outputs either a IndexCFI or
    /// UnsupportedIndex
    EscapeCfi(Cfi),
}

/// Errors that might occur when getting a CFI from a model:
#[derive(Debug, Error)]
pub enum ModelCfiError {
    #[error("The model does not support the symbol \"{0}\", yet it was queried")]
    UnsupportedSymbol(Symbol),
    #[error("The CFI of the symbol \"{symbol:}\" is empty")]
    EmptyCfi { symbol: Symbol },
}

/// A trait defining the behavior of a probability model
pub trait Model {
    /// Computes a Cumulative-Frequency-Interval for a given symbol.
    ///
    /// ## Parameters:
    /// * _symbol_: The symbol whose CFI will be returned.<br>
    ///   Depending on the implementation, the model may return the CFI directly or emit
    ///   an escape CFI. If the model emits an escape CFI for a non-escape symbol, it is the
    ///   responsibility of the caller to repeatedly call the `get_cfi` + `update` methods
    ///   until either an actual CFI or an error is returned.
    /// ## Returns:
    /// A CFI assigned to that symbol in the model, or an escape CFI leading to that CFI.
    ///
    /// ## Possible Failures:
    /// Each model should return `ModelCfiError::UnsupportedIndex` if _symbol_ is not a part of
    /// their allowed symbols.
    /// Additionally, each model should return a `ModelCfiError::EmptyCfi` if the CFI assigned to
    /// the given symbol is empty (i.e: its start value equals its end value)
    fn get_cfi(&self, symbol: Symbol) -> Result<ModelCfi, ModelCfiError>;

    /// Given a cumulative frequency value, the function returns the symbol whose CFI in the model
    /// contains the given value.
    /// If no adequate CFI is found, None is returned.
    /// # Parameters:
    /// * cumulative_frequency - A cumulative frequency value that lies inside a CFI in the model.
    fn get_symbol(&self, cumulative_frequency: Frequency) -> Option<Symbol>;

    /// Returns the total cumulative frequencies in the table currently used by the model.
    fn get_total(&self) -> Frequency;

    /// Resets the state of the model. Must be called between independent uses of the model (for
    /// example, decompression after compression) to avoid unexpected behaviour.
    fn flush(&mut self) {}

    /// Updates the model based on some ModelCFI. This function should be called right after calling
    /// the `model.get_cfi(symbol)` function, using its output as the current function's
    /// _model_result_ parameter.
    ///
    /// ## Parameters
    /// * _symbol_ - The symbol given to the model's `get_cfi` function.
    /// * _model_result_: &ModelCFI - The result of calling `get_cfi` with _symbol_.
    ///
    /// ## Returns
    /// Nothing if the update went smoothly, otherwise propagates any update error.
    #[allow(unused_variables)]
    fn update(&mut self, symbol: Symbol, model_result: &ModelCfi) -> Result<()> {
        Ok(())
    }
}
