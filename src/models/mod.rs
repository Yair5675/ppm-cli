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

use crate::frequencies::Cfi;

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
