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

use anyhow::Result;
use crate::frequencies::{Cfi, Frequency};

/// Outputs of a probability model, wrapping CFIs to provide information for model-updating.
pub enum ModelCFI {
    /// Normal CFI, represents a regular symbol/index
    IndexCfi(Cfi),

    /// Either a CFI of an escape symbol, OR a CFI given by the model to alert the decompression
    /// of something. If received during the compression of a non-escape symbol, the compressor
    /// needs to re-compress the symbol until the model outputs either a IndexCFI or 
    /// UnsupportedIndex
    EscapeCfi(Cfi),
    
    /// The result of an unsupported index/symbol given to the model
    UnsupportedIndex
}

/// A trait defining the behavior of a probability model
pub trait Model {
    /// Computes a Cumulative-Frequency-Interval for a given index.
    /// 
    /// ## Parameters:
    /// * _index_: The index whose CFI will be returned.<br>
    ///            Depending on the implementation, the model may return the CFI directly or emit
    ///            an escape CFI. If the model emits an escape CFI for a non-escape index, it is the
    ///            responsibility of the caller to repeatedly call the `get_cfi` + `update` methods
    ///            until either an actual CFI is returned OR an `UnsupportedIndex` variant is 
    ///            returned.
    /// ## Returns:
    /// A CFI assigned to that index in the model, or an escape CFI leading to that CFI.
    /// 
    /// ## Possible Failures:
    /// A `ModelCFI::UnsupportedIndex` may be returned if an unsupported index was provided. 
    /// Moreover, the specific implementation of the Model may fail for other reasons (hence the
    /// anyhow::Result wrapper around the ModelCfi).
    fn get_cfi(&self, index: usize) -> Result<ModelCFI>;

    /// Given a cumulative frequency value, the function returns the index whose CFI in the model
    /// contains the given value.
    /// If no adequate CFI is found, None is returned.
    /// # Parameters:
    /// * cumulative_frequency - A cumulative frequency value that lies inside a CFI in the model.
    fn get_symbol(&self, cumulative_frequency: Frequency) -> Option<usize>;
    
    /// Returns the total cumulative frequencies in the table currently used by the model.
    fn get_total(&self) -> Frequency;

    /// Resets the state of the model. Must be called between independent uses of the model (for
    /// example, decompression after compression) to avoid unexpected behaviour.
    fn flush(&mut self) {}

    /// Updates the model based on some ModelCFI. This function should be called right after calling
    /// the `model.get_cfi(index)` function, using its output as the current function's
    /// _model_result_ parameter.
    ///
    /// ## Parameters
    /// * _index_ - The index given to the model's `get_cfi` function.
    /// * _model_result_: &ModelCFI - The result of calling `get_cfi` with _symbol_.
    ///
    /// ## Returns
    /// Nothing if the update went smoothly, otherwise propagates any update error.
    #[allow(unused_variables)]
    fn update(&mut self, index: usize, model_result: &ModelCFI) -> Result<()> {
        Ok(())
    }
}
