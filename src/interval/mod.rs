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

mod bits_system;

pub use self::bits_system::BitsSystem;
use crate::frequencies::Cfi;
use crate::number_types::{CalculationsType, ConstrainedNum, INTERVAL_BITS};
use anyhow::{anyhow, Result};
use log::{debug, error};
use std::fmt::{Display, Formatter};

/// Boundary of an interval, an integer representation of a fractional value between 0 and 1.
pub type IntervalBoundary = ConstrainedNum<INTERVAL_BITS>;

/// An interval containing values between 0 and 1 using the integer representation for fractional
/// values.
pub struct Interval {
    /// Lower boundary of the interval
    low: IntervalBoundary,
    /// Upper boundary of the interval
    high: IntervalBoundary,
    /// The BitsSystem of the interval
    system: BitsSystem<INTERVAL_BITS>,
}

impl Interval {
    /// Forms a new Interval that represents the mathematical interval [0, 1).
    pub fn full_interval() -> Self {
        let system: BitsSystem<INTERVAL_BITS> =
            BitsSystem::new().expect("For some reason INTERVAL_BITS was set to less than 2 ಠ_ಠ");

        Self {
            low: IntervalBoundary::zero(),
            high: IntervalBoundary::max(),
            system,
        }
    }

    /// Updates the model's boundaries based on a Cumulative-Frequency-Interval.
    pub fn update(&mut self, cfi: Cfi) {
        debug!("Interval: Updating with CFI {:?}", cfi);
        // Compute the width of the interval:
        let width: CalculationsType = *self.high - *self.low + 1;

        // ASSUMPTION - cfi.start < cfi.end <= cfi.total
        // In that case, updating boundaries will never cause overflow, since it will compute a
        // value smaller than or equal to the current high (at most), which must be valid.
        // Moreover, it will not break the invariance low < high since cfi.start < cfi.end.
        unsafe {
            let new_low = IntervalBoundary::new_unchecked(
                *self.low + (width * *cfi.start).div_euclid(*cfi.total),
            );
            let new_high = IntervalBoundary::new_unchecked(
                *self.low + (width * *cfi.end).div_euclid(*cfi.total) - 1,
            );
            (self.low, self.high) = (new_low, new_high);
            debug!("Interval: Post-update interval: {}", self)
        }
    }

    pub fn get_state(&self) -> IntervalState {
        match () {
            // Check convergence:
            _ if self.low >= self.system.half() => IntervalState::Converging(true),
            _ if self.high < self.system.half() => IntervalState::Converging(false),

            // Check near-convergence:
            _ if self.low >= self.system.one_fourth()
                && self.high < self.system.three_fourths() =>
            {
                IntervalState::NearConvergence
            }

            // Default:
            _ => IntervalState::NoConvergence,
        }
    }

    pub fn low(&self) -> IntervalBoundary {
        self.low
    }

    pub fn high(&self) -> IntervalBoundary {
        self.high
    }

    pub fn set_boundaries(
        &mut self,
        new_low: IntervalBoundary,
        new_high: IntervalBoundary,
    ) -> Result<()> {
        Self::validate_boundaries_invariant(&new_low, &new_high)?;
        (self.low, self.high) = (new_low, new_high);
        debug!("Interval: Interval with new boundaries: {}", self);
        Ok(())
    }

    pub fn set_low(&mut self, new_low: IntervalBoundary) -> Result<()> {
        Self::validate_boundaries_invariant(&new_low, &self.high)?;
        self.low = new_low;
        Ok(())
    }

    pub fn set_high(&mut self, new_high: IntervalBoundary) -> Result<()> {
        Self::validate_boundaries_invariant(&self.low, &new_high)?;
        self.high = new_high;
        Ok(())
    }

    pub fn system(&self) -> &BitsSystem<INTERVAL_BITS> {
        &self.system
    }

    /// Validates that setting the interval's boundaries to the proposed ones will not break the
    /// boundaries invariant `low < high`.
    fn validate_boundaries_invariant(
        new_low: &IntervalBoundary,
        new_high: &IntervalBoundary,
    ) -> Result<()> {
        let (low, high) = (**new_low, **new_high);
        if low < high {
            Ok(())
        } else {
            let msg = format!(
                "Updating boundaries would break the invariance low < high (new low: {:0bits$b} >= new high {:0bits$b}",
                low, high, bits = INTERVAL_BITS as usize
            );
            error!("{}", msg);
            Err(anyhow!(msg))
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:0bits$b}, {:0bits$b})",
            *self.low,
            *self.high,
            bits = INTERVAL_BITS as usize
        )
    }
}

/// The state of an interval, based on its boundaries
pub enum IntervalState {
    /// The interval's lower and upper boundaries both have the same Most-Significant Bit, which
    /// means the interval converges to a value.<br>
    /// The value held by the variant is **true** if the MSB is 1, and **false** if it's 0.
    Converging(bool),

    /// The interval nearly converges - a special case that happens when low = 01XX...X and high =
    /// 10YY...Y.
    NearConvergence,

    /// The interval's boundaries do not converge or nearly converge, which is the default state.
    NoConvergence,
}
