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

use super::static_table::StaticFrequencyTable;
use super::{Cfi, Frequency, FrequencyTable};

#[test]
fn test_static_frequency_table_creation() {
    let freqs = vec![
        Frequency::new(2).unwrap(),
        Frequency::new(3).unwrap(),
        Frequency::new(5).unwrap(),
    ];
    let table = StaticFrequencyTable::new(&freqs).unwrap();

    assert_eq!(table.get_total(), Frequency::new(10).unwrap());

    // Cumulative frequencies: [0, 2, 5, 10]
    assert_eq!(
        table.get_cfi(0),
        Some(Cfi {
            start: Frequency::new(0).unwrap(),
            end: Frequency::new(2).unwrap(),
            total: Frequency::new(10).unwrap()
        })
    );
    assert_eq!(
        table.get_cfi(1),
        Some(Cfi {
            start: Frequency::new(2).unwrap(),
            end: Frequency::new(5).unwrap(),
            total: Frequency::new(10).unwrap()
        })
    );
    assert_eq!(
        table.get_cfi(2),
        Some(Cfi {
            start: Frequency::new(5).unwrap(),
            end: Frequency::new(10).unwrap(),
            total: Frequency::new(10).unwrap()
        })
    );
    assert_eq!(table.get_cfi(3), None);
}

#[test]
fn test_static_frequency_table_get_index() {
    let freqs = vec![
        Frequency::new(1).unwrap(),
        Frequency::new(2).unwrap(),
        Frequency::new(3).unwrap(),
    ];
    let table = StaticFrequencyTable::new(&freqs).unwrap();

    // Cumulative: [0, 1, 3, 6]
    assert_eq!(table.get_index(Frequency::new(0).unwrap()), Some(0));
    assert_eq!(table.get_index(Frequency::new(1).unwrap()), Some(1));
    assert_eq!(table.get_index(Frequency::new(2).unwrap()), Some(1));
    assert_eq!(table.get_index(Frequency::new(3).unwrap()), Some(2));
    assert_eq!(table.get_index(Frequency::new(5).unwrap()), Some(2));
    assert_eq!(table.get_index(Frequency::new(6).unwrap()), None);
}

#[test]
fn test_static_frequency_table_overflow() {
    // This should fail if it overflows
    let max = Frequency::max();
    let result = StaticFrequencyTable::new(&[max, Frequency::one()]);
    assert!(result.is_err());
}
