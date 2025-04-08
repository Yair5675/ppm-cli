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

pub mod symbol;

use symbol::Symbol;

pub trait SymbolIndexMapping {
    /// Computes a unique index for _symbol_. If _symbol_ is not supported by the mapping, None is
    /// returned.<br>
    /// It must be guaranteed that if an index is returned, it lies in the range
    /// [0, `Self::supported_symbols_count()`).
    fn get_index(symbol: &Symbol) -> Option<usize>;

    /// Returns the symbol to which _index_ is assigned to. If no symbol is mapped to _index_, None
    /// is returned.
    fn get_symbol(index: usize) -> Option<Symbol>;

    /// Returns the number of symbols the mapping supports.
    fn supported_symbols_count() -> usize;
}
