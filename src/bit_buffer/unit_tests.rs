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

use super::bit_iter::BitIterator;
use super::BitBuffer;

#[test]
fn empty_upon_initializing() {
    let buffer = BitBuffer::new();
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 0);
    assert!(buffer.full_bytes.is_empty())
}

#[test]
fn test_less_than_byte_appends() {
    let mut buffer = BitBuffer::new();

    buffer.append(false);
    assert_eq!(buffer.current_byte, 0u8);
    assert_eq!(buffer.current_idx, 1);
    assert!(buffer.full_bytes.is_empty());

    buffer.append(true);
    assert_eq!(buffer.current_byte, 0b01000000u8);
    assert_eq!(buffer.current_idx, 2);
    assert!(buffer.full_bytes.is_empty());
}

#[test]
fn test_exactly_one_byte_appends() {
    let mut buffer = BitBuffer::new();
    buffer.append(true);
    buffer.append(false);
    buffer.append(true);
    buffer.append(true);
    buffer.append(false);
    buffer.append(true);
    buffer.append(true);
    buffer.append(true);

    assert_eq!(buffer.full_bytes.len(), 1);
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 0);

    let first_byte = buffer.full_bytes.front().unwrap();
    assert_eq!(first_byte, &0b10110111u8);
}

#[test]
fn test_over_one_byte_appends() {
    let mut buffer = BitBuffer::new();
    buffer.append(true);
    buffer.append(false);
    buffer.append(true);
    buffer.append(true);
    buffer.append(false);
    buffer.append(true);
    buffer.append(true);
    buffer.append(true);

    buffer.append(false);
    buffer.append(true);

    assert_eq!(buffer.full_bytes.len(), 1);
    assert_eq!(buffer.current_byte, 0b01000000);
    assert_eq!(buffer.current_idx, 2);

    let first_byte = buffer.full_bytes.front().unwrap();
    assert_eq!(first_byte, &0b10110111u8);
}

#[test]
fn test_less_than_byte_appends_repeated() {
    let mut buffer = BitBuffer::new();
    buffer.append_repeated(true, 5);
    assert_eq!(buffer.current_byte, 0b11111000u8);
    assert_eq!(buffer.current_idx, 5);
    assert!(buffer.full_bytes.is_empty());

    buffer = BitBuffer::new();
    buffer.append_repeated(false, 4);
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 4);
    assert!(buffer.full_bytes.is_empty());
}

#[test]
fn test_exactly_one_byte_appends_repeated() {
    let mut buffer = BitBuffer::new();
    buffer.append_repeated(true, 8);

    assert_eq!(buffer.full_bytes.len(), 1);
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 0);
    let byte = buffer.full_bytes.front().unwrap();
    assert_eq!(byte, &u8::MAX);

    buffer = BitBuffer::new();
    buffer.append_repeated(false, 8);

    assert_eq!(buffer.full_bytes.len(), 1);
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 0);
    let byte = buffer.full_bytes.front().unwrap();
    assert_eq!(byte, &0);
}

#[test]
fn test_over_one_byte_appends_repeated() {
    let mut buffer = BitBuffer::new();
    buffer.append_repeated(true, 18);

    assert_eq!(buffer.full_bytes.len(), 2);
    assert_eq!(buffer.current_byte, 0b11000000u8);
    assert_eq!(buffer.current_idx, 2);

    let (front, back) = (
        buffer.full_bytes.front().unwrap(),
        buffer.full_bytes.back().unwrap(),
    );
    assert_eq!(front, &u8::MAX);
    assert_eq!(back, &u8::MAX);

    buffer = BitBuffer::new();
    buffer.append_repeated(false, 19);

    assert_eq!(buffer.full_bytes.len(), 2);
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 3);

    let (front, back) = (
        buffer.full_bytes.front().unwrap(),
        buffer.full_bytes.back().unwrap(),
    );
    assert_eq!(front, &0);
    assert_eq!(back, &0);
}

#[test]
fn test_len_empty() {
    let buffer = BitBuffer::new();
    assert_eq!(buffer.len(), 0);
}

#[test]
fn test_len_less_than_byte() {
    let mut buffer = BitBuffer::new();
    buffer.append_repeated(false, 5);
    assert_eq!(buffer.len(), 5);
}

#[test]
fn test_len_multiple_bytes() {
    let buffer = BitBuffer::from(vec![100, 11, 23, 45, 68, 19]);
    assert_eq!(buffer.len(), 8 * 6);
}

#[test]
fn test_full_bytes_new_buffer() {
    let mut buffer = BitBuffer::new();
    let bytes: Vec<u8> = buffer.get_complete_bytes().collect();
    assert_eq!(bytes, Vec::new());
    assert!(buffer.full_bytes.is_empty());
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 0);
}

#[test]
fn test_full_bytes_not_enough_bits() {
    let mut buffer = BitBuffer::new();
    buffer.append_repeated(true, 6);
    let bytes: Vec<u8> = buffer.get_complete_bytes().collect();
    assert_eq!(bytes, Vec::new());
    assert!(buffer.full_bytes.is_empty());
    assert_eq!(buffer.current_byte, 0b11111100);
    assert_eq!(buffer.current_idx, 6);

    buffer = BitBuffer::new();
    buffer.append_repeated(false, 7);
    let bytes: Vec<u8> = buffer.get_complete_bytes().collect();
    assert_eq!(bytes, Vec::new());
    assert!(buffer.full_bytes.is_empty());
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 7);
}

#[test]
fn test_full_bytes_exactly_one_byte() {
    let mut buffer = BitBuffer::new();
    buffer.append_repeated(true, 8);

    let bytes: Vec<u8> = buffer.get_complete_bytes().collect();
    assert_eq!(vec![u8::MAX], bytes);
    assert!(buffer.full_bytes.is_empty());
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 0);

    buffer = BitBuffer::new();
    buffer.append_repeated(false, 8);

    let bytes: Vec<u8> = buffer.get_complete_bytes().collect();
    assert_eq!(vec![0], bytes);
    assert!(buffer.full_bytes.is_empty());
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 0);
}

#[test]
fn test_full_bytes_multiple_bytes_no_remainder() {
    let mut buffer = BitBuffer::new();
    buffer.append_repeated(true, 16);

    let bytes: Vec<u8> = buffer.get_complete_bytes().collect();
    assert_eq!(vec![u8::MAX, u8::MAX], bytes);
    assert!(buffer.full_bytes.is_empty());
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 0);

    buffer = BitBuffer::new();
    buffer.append_repeated(false, 24);

    let bytes: Vec<u8> = buffer.get_complete_bytes().collect();
    assert_eq!(vec![0, 0, 0], bytes);
    assert!(buffer.full_bytes.is_empty());
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 0);
}

#[test]
fn test_full_bytes_multiple_bytes_with_remainder() {
    let mut buffer = BitBuffer::new();
    buffer.append_repeated(true, 20);

    let bytes: Vec<u8> = buffer.get_complete_bytes().collect();
    assert_eq!(vec![u8::MAX, u8::MAX], bytes);
    assert!(buffer.full_bytes.is_empty());
    assert_eq!(buffer.current_byte, 0b11110000);
    assert_eq!(buffer.current_idx, 4);

    buffer = BitBuffer::new();
    buffer.append_repeated(false, 27);

    let bytes: Vec<u8> = buffer.get_complete_bytes().collect();
    assert_eq!(vec![0, 0, 0], bytes);
    assert!(buffer.full_bytes.is_empty());
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 3);
}

#[test]
fn test_from_slice() {
    // Test converting a slice into a BitBuffer
    let data: &[u8] = &[0b10101010, 0b11001100, 0b11110000];
    let mut buffer: BitBuffer = data.into();

    // The buffer should have exactly 3 bytes
    assert_eq!(buffer.full_bytes.len(), 3);
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 0);

    // Check the contents of the bytes in the buffer
    let bytes: Vec<u8> = buffer.get_complete_bytes().collect();
    assert_eq!(bytes, vec![0b10101010, 0b11001100, 0b11110000]);
    assert!(buffer.full_bytes.is_empty());
}

#[test]
fn test_from_vec() {
    // Test converting a Vec<u8> into a BitBuffer
    let data: Vec<u8> = vec![0b10101010, 0b11001100, 0b11110000];
    let mut buffer: BitBuffer = data.into();

    // The buffer should have exactly 3 bytes
    assert_eq!(buffer.full_bytes.len(), 3);
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 0);

    // Check the contents of the bytes in the buffer
    let bytes: Vec<u8> = buffer.get_complete_bytes().collect();
    assert_eq!(bytes, vec![0b10101010, 0b11001100, 0b11110000]);
    assert!(buffer.full_bytes.is_empty());
}

#[test]
fn test_from_empty_slice() {
    // Test converting an empty slice into a BitBuffer
    let data: &[u8] = &[];
    let buffer: BitBuffer = data.into();

    // The buffer should have no bytes
    assert_eq!(buffer.full_bytes.len(), 0);
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 0);
}

#[test]
fn test_from_empty_vec() {
    // Test converting an empty Vec<u8> into a BitBuffer
    let data: Vec<u8> = Vec::new();
    let buffer: BitBuffer = data.into();

    // The buffer should have no bytes
    assert_eq!(buffer.full_bytes.len(), 0);
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 0);
}

#[test]
fn test_from_single_byte() {
    // Test converting a single byte slice into a BitBuffer
    let data: &[u8] = &[0b10101010];
    let mut buffer: BitBuffer = data.into();

    // The buffer should have exactly 1 byte
    assert_eq!(buffer.full_bytes.len(), 1);
    assert_eq!(buffer.current_byte, 0);
    assert_eq!(buffer.current_idx, 0);

    // Check the contents of the bytes in the buffer
    let bytes: Vec<u8> = buffer.get_complete_bytes().collect();
    assert_eq!(bytes, vec![0b10101010]);
    assert!(buffer.full_bytes.is_empty());
}

#[test]
fn test_leftover_empty() {
    let buffer = BitBuffer::new();
    assert!(buffer.get_leftover_bits().is_none());
}

#[test]
fn test_leftover_less_than_byte() {
    let mut buffer = BitBuffer::new();
    buffer.append(true);
    buffer.append(false);
    buffer.append(true);

    let leftover = buffer.get_leftover_bits();
    assert!(leftover.is_some());
    assert_eq!(leftover.unwrap(), 0b10100000);
}

#[test]
fn test_leftover_exactly_one_byte() {
    let buffer = BitBuffer::from(vec![0b10011010u8]);

    let leftover = buffer.get_leftover_bits();
    assert!(leftover.is_none());
}

#[test]
fn test_leftover_byte_with_remainder() {
    let mut buffer = BitBuffer::from(vec![0b10011010u8]);
    buffer.append(false);

    let leftover = buffer.get_leftover_bits();
    assert!(leftover.is_some());
    assert_eq!(leftover.unwrap(), 0);
}

#[test]
fn test_leftover_multiple_bytes_no_remainder() {
    let buffer = BitBuffer::from(vec![15, 120u8, 11, 33]);

    let leftover = buffer.get_leftover_bits();
    assert!(leftover.is_none());
}

#[test]
fn test_bit_iterator_empty() {
    let buffer = BitBuffer::new();
    let bit_iterator: BitIterator = buffer.into();

    let bits: Vec<bool> = bit_iterator.collect();
    assert!(bits.is_empty());
}

#[test]
fn test_bit_iterator_less_than_byte() {
    let mut buffer = BitBuffer::new();
    buffer.append(true);
    buffer.append(false);
    buffer.append(true);

    let bit_iterator: BitIterator = buffer.into();

    let expected_bits = vec![true, false, true];
    let bits: Vec<bool> = bit_iterator.collect();
    assert_eq!(bits, expected_bits);
}

#[test]
fn test_bit_iterator_exactly_one_byte() {
    let mut buffer = BitBuffer::new();
    buffer.append(true);
    buffer.append(false);
    buffer.append(true);
    buffer.append(true);
    buffer.append(false);
    buffer.append(true);
    buffer.append(false);
    buffer.append(true);

    let bit_iterator: BitIterator = buffer.into();

    let expected_bits = vec![true, false, true, true, false, true, false, true];
    let bits: Vec<bool> = bit_iterator.collect();
    assert_eq!(bits, expected_bits);
}

#[test]
fn test_bit_iterator_multiple_bytes() {
    let mut buffer = BitBuffer::new();
    buffer.append_repeated(true, 8);
    buffer.append_repeated(true, 8);
    buffer.append_repeated(false, 8);
    buffer.append_repeated(true, 8);

    let bit_iterator: BitIterator = buffer.into();

    let expected_bits = vec![
        true, true, true, true, true, true, true, true, // First byte
        true, true, true, true, true, true, true, true, // Second byte
        false, false, false, false, false, false, false, false, // Third byte
        true, true, true, true, true, true, true, true, // Fourth byte
    ];

    let bits: Vec<bool> = bit_iterator.collect();
    assert_eq!(bits.len(), 8 * 4);
    assert_eq!(bits, expected_bits);
}

#[test]
fn test_bit_iterator_from_slice() {
    let byte_slice = vec![0b10101010u8, 0b11001100u8]; // 10101010 11001100
    let bit_iterator: BitIterator = BitIterator::from(byte_slice);

    let expected_bits = vec![
        true, false, true, false, true, false, true, false, // First byte
        true, true, false, false, true, true, false, false, // Second byte
    ];

    let bits: Vec<bool> = bit_iterator.collect();
    assert_eq!(bits.len(), 8 * 2);
    assert_eq!(bits, expected_bits);
}
