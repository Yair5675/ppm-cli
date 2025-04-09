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

use std::fmt::{Display, Formatter};

/// The number of unique symbols (256 byte values + 1 EOF + 1 ESCAPE)
pub const UNIQUE_SYMBOLS_AMOUNT: usize = 258;

/// A symbol in the compression/decompression process, its possible values contain all byte values
/// plus additional metadata values
#[derive(Copy, Clone, Debug)]
pub enum Symbol {
    /// A byte value
    Byte(u8),
    /// An End-Of-File value
    Eof,
    /// An 'escape' value
    Esc,
}

impl Symbol {
    pub fn is_escape(&self) -> bool {
        matches!(self, Symbol::Esc)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Byte(b) => write!(f, "{}", b),
            Symbol::Eof => write!(f, "EOF"),
            Symbol::Esc => write!(f, "ESCAPE"),
        }
    }
}
