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

use crate::number_types::CalculationsType;

/// Computes the least significant set bit of a number
fn lsb(n: usize) -> usize {
    let n = n as isize;
    (n & -n) as usize
}

/// A data structure that allows efficient calculation of cumulative summation AND mutation of
/// values
pub struct FenwickTree {
    // Values of the tree, allow for quick computation of cumulative sum AND mutation of values.
    // It uses Box since we never append/remove elements, only mutate them:
    data: Box<[CalculationsType]>,
}

impl FenwickTree {
    /// Creates a new, empty FenwickTree with the given size
    pub fn new(size: usize) -> Self {
        // Fenwick trees index calculations depend on the indices starting at 1, so add an extra
        // element to ensure this:
        Self {
            data: vec![0; size + 1].into_boxed_slice(),
        }
    }

    /// Computes the cumulative sum of all values up to (but not including) the given index.<br>
    /// This function's time complexity is **O(log n)**.
    pub fn get_sum(&self, mut index: usize) -> CalculationsType {
        let mut sum = 0;
        while 0 < index && index < self.data.len() {
            sum += self.data[index];
            index -= lsb(index);
        }
        sum
    }

    /// Returns the length of the tree, i.e: how many elements it contains
    pub fn len(&self) -> usize {
        self.data.len() - 1
    }

    /// Adds a certain amount to an index in the tree in **O(log n)** time complexity.
    pub fn add(&mut self, mut index: usize, amount: CalculationsType) {
        // Shift the index by one since the fenwick tree is 1-based:
        index += 1;
        while index < self.data.len() {
            self.data[index] += amount;
            index += lsb(index);
        }
    }
}

impl<const N: usize> From<&[CalculationsType; N]> for FenwickTree {
    fn from(values: &[CalculationsType; N]) -> Self {
        FenwickTree::from(&values[..])
    }
}

impl From<&Vec<CalculationsType>> for FenwickTree {
    fn from(values: &Vec<CalculationsType>) -> Self {
        FenwickTree::from(&values[..])
    }
}

impl From<&[CalculationsType]> for FenwickTree {
    /// Constructs a FenwickTree containing the given values.
    ///
    /// This function is more efficient than adding them manually to an empty tree, as this function
    /// optimizes the operation and reduces the time complexity from **O(n log n)** to **O(n)**.
    fn from(values: &[CalculationsType]) -> Self {
        // Initialize data to be all zeroes. Fenwick trees are 1-based, so we add 1 to the length:
        let mut data = vec![0; values.len() + 1];

        for i in 1..data.len() {
            // Copy from values:
            data[i] += values[i - 1];

            // Find the parent index, and add to it as well:
            let parent_idx = i + lsb(i);
            if parent_idx < data.len() {
                let add_to_parent = data[i];
                data[parent_idx] += add_to_parent;
            }
        }

        Self {
            data: data.into_boxed_slice(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_fenwick_tree() {
        let values = [1, 2, 3, 4, 5];
        let tree = FenwickTree::from(&values[..]);

        // Test if the cumulative sums are correct:
        assert_eq!(tree.get_sum(0), 0); // Sum up to index 0 (should be 0)
        assert_eq!(tree.get_sum(1), 1); // Sum up to index 1 (should be 1)
        assert_eq!(tree.get_sum(2), 3); // Sum up to index 2 (should be 1 + 2)
        assert_eq!(tree.get_sum(3), 6); // Sum up to index 3 (should be 1 + 2 + 3)
        assert_eq!(tree.get_sum(4), 10); // Sum up to index 4 (should be 1 + 2 + 3 + 4)
        assert_eq!(tree.get_sum(5), 15); // Sum up to index 5 (should be 1 + 2 + 3 + 4 + 5)
    }

    #[test]
    fn test_empty_tree() {
        let tree = FenwickTree::new(5);

        // For an empty tree, all sums should be zero
        assert_eq!(tree.get_sum(0), 0);
        assert_eq!(tree.get_sum(1), 0);
        assert_eq!(tree.get_sum(2), 0);
        assert_eq!(tree.get_sum(3), 0);
        assert_eq!(tree.get_sum(4), 0);
        assert_eq!(tree.get_sum(5), 0);
    }

    #[test]
    fn test_add() {
        let mut tree = FenwickTree::from(&[1, 2, 3, 4, 5]);

        // New tree after addition - [1, 2, 6, 4, 5]:
        tree.add(2, 3);

        assert_eq!(tree.get_sum(1), 1); // 1
        assert_eq!(tree.get_sum(2), 3); // 1 + 2 = 3
        assert_eq!(tree.get_sum(3), 9); // 1 + 2 + 6 = 9
        assert_eq!(tree.get_sum(4), 13); // 1 + 2 + 6 + 4 = 13
        assert_eq!(tree.get_sum(5), 18); // 1 + 2 + 6 + 4 + 5 = 18
    }

    #[test]
    fn test_multiple_adds() {
        let mut tree = FenwickTree::from(&[1, 2, 3, 4, 5]);

        // New tree after both additions = [1, 7, 3, 14, 5]
        tree.add(3, 10);
        tree.add(1, 5);

        assert_eq!(tree.get_sum(1), 1); // 1
        assert_eq!(tree.get_sum(2), 8); // 1 + 7 = 8
        assert_eq!(tree.get_sum(3), 11); // 1 + 7 + 3 = 11
        assert_eq!(tree.get_sum(4), 25); // 1 + 7 + 3 + 14 = 25
        assert_eq!(tree.get_sum(5), 30); // 1 + 7 + 3 + 14 + 5 = 30
    }

    #[test]
    fn test_edge_case_empty_values() {
        let empty: Vec<CalculationsType> = Vec::new();
        let tree = FenwickTree::from(&empty); // Create an empty tree
        assert_eq!(tree.get_sum(0), 0); // There should be no sum for any index
    }

    #[test]
    fn test_large_input() {
        let values: Vec<CalculationsType> = (1..=10000).collect();
        let tree = FenwickTree::from(&values);

        // Test if the sum of the first 10000 values is correct
        assert_eq!(tree.get_sum(10_000), 50005000); // Sum of first 10000 natural numbers: n*(n+1)/2
    }
}
