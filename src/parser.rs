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

use crate::sim::Symbol;

/// A trait for pre-processing raw byte values into compressible Symbols.
pub trait Parser {
    /// Compresses a single byte into at least one Symbol.<br>
    fn parse_byte(&self, byte: u8) -> Vec<Symbol>;
}

impl<P: Parser + ?Sized> Parser for Box<P> {
    fn parse_byte(&self, byte: u8) -> Vec<Symbol> {
        (**self).parse_byte(byte)
    }
}

/// Regular parser - parses bytes directly into a `Symbol::Byte`
pub struct ByteParser;
impl Parser for ByteParser {
    fn parse_byte(&self, byte: u8) -> Vec<Symbol> {
        vec![Symbol::Byte(byte)]
    }
}

/// Parser for binary symbols - each byte is parsed into 8 symbols, where each symbol is either
/// `Symbol::Byte(0)` or `Symbol::Byte(1)` (depending on the corresponding bit value).<br>
/// Bits are parsed in big-endian.
pub struct BitParser;
impl Parser for BitParser {
    fn parse_byte(&self, byte: u8) -> Vec<Symbol> {
        let mut symbols = Vec::with_capacity(8);
        let mut mask: u8 = 0b10000000;

        for _ in 0..8 {
            let symbol = if byte & mask != 0 {
                Symbol::Byte(1)
            } else {
                Symbol::Byte(0)
            };
            symbols.push(symbol);
            mask >>= 1;
        }

        symbols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_parser_all_bytes() {
        let parser = ByteParser;
        for byte in 0..=255 {
            let result = parser.parse_byte(byte);
            let expected = vec![Symbol::Byte(byte)];
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_bit_parser_all_zero() {
        let parser = BitParser;
        let result = parser.parse_byte(0u8);
        let expected = vec![Symbol::Byte(0); 8];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bit_parser_all_one() {
        let parser = BitParser;
        let result = parser.parse_byte(0b11111111);
        let expected = vec![Symbol::Byte(1); 8];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bit_parser_alternating_bits() {
        let parser = BitParser;
        let result = parser.parse_byte(0b10101010);
        let expected = vec![
            Symbol::Byte(1),
            Symbol::Byte(0),
            Symbol::Byte(1),
            Symbol::Byte(0),
            Symbol::Byte(1),
            Symbol::Byte(0),
            Symbol::Byte(1),
            Symbol::Byte(0),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bit_parser_reverse_alternating_bits() {
        let parser = BitParser;
        let result = parser.parse_byte(0b01010101);
        let expected = vec![
            Symbol::Byte(0),
            Symbol::Byte(1),
            Symbol::Byte(0),
            Symbol::Byte(1),
            Symbol::Byte(0),
            Symbol::Byte(1),
            Symbol::Byte(0),
            Symbol::Byte(1),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bit_parser_random_bits() {
        let parser = BitParser;
        let result = parser.parse_byte(0b11001001);
        let expected = vec![
            Symbol::Byte(1),
            Symbol::Byte(1),
            Symbol::Byte(0),
            Symbol::Byte(0),
            Symbol::Byte(1),
            Symbol::Byte(0),
            Symbol::Byte(0),
            Symbol::Byte(1),
        ];
        assert_eq!(result, expected);
    }
}
