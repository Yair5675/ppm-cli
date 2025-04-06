#[cfg(test)]
mod unit_tests;
mod bit_ops;

use super::sizes::CalculationsType;
use std::ops::Deref;
use thiserror::Error;

/// Returns the number of bits used by a number
const fn get_used_bits_num(n: CalculationsType) -> u32 {
    CalculationsType::BITS - n.leading_zeros()
}

/// A numerical struct restricting the value it holds to have a limited amount of bits
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ConstrainedNum<const BITS: u32>(CalculationsType);

impl<const BITS: u32> ConstrainedNum<BITS> {
    /// Creates a new ConstrainedNum.
    ///
    /// ## Rules:
    /// The BITS assigned to it must be between 1 and CalculationsType::BITS (inclusively), and the
    /// given value cannot use more bits than BITS.<br>
    /// If one of those rules is broken, an appropriate error is returned.
    pub fn new(value: CalculationsType) -> Result<Self, BitsConstraintError<BITS>> {
        // Check BITS:
        if BITS == 0 {
            return Err(BitsConstraintError::ZeroBitsGiven);
        } else if BITS > CalculationsType::BITS {
            return Err(BitsConstraintError::BitsConstantTooLarge);
        }

        // Check value:
        let used_bits = get_used_bits_num(value);
        if used_bits > BITS {
            Err(BitsConstraintError::ValueUsesTooManyBits { value, used_bits })
        } else {
            Ok(Self(value))
        }
    }
    
    /// Creates a ConstrainedNum holding the value 0.<br>
    /// This operation is always safe since 0 uses no bits.
    pub fn zero() -> Self {
        Self(0)
    }
    
    /// Creates a ConstrainedNum holding the value 1.<br>
    /// This operation is always safe since BITS must be greater than or equal to 1, therefor 
    /// always allowing it to hold the value 1.
    pub fn one() -> Self {
        Self(1)
    }
    
    /// Returns the maximum value allowed using BITS bits.
    pub const fn max() -> Self {
        if BITS == CalculationsType::BITS {
            Self(CalculationsType::MAX)
        } else {
            Self((1 << BITS) - 1)
        }
    }
}

// Allow direct access to the numerical type:
impl<const BITS: u32> Deref for ConstrainedNum<BITS> {
    type Target = CalculationsType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const BITS: u32> From<ConstrainedNum<BITS>> for CalculationsType {
    fn from(value: ConstrainedNum<BITS>) -> Self {
        value.0
    }
}

impl<const BITS: u32> From<bool> for ConstrainedNum<BITS> {
    fn from(value: bool) -> Self {
        if value {
            Self::one()
        } else {
            Self::zero()
        }
    }
}

#[derive(Debug, Error)]
pub enum BitsConstraintError<const BITS: u32> {
    /// Bits constraint must have at least 1 bit
    #[error("BITS was set to 0, which is invalid")]
    ZeroBitsGiven,

    /// Generic constant BITS is larger than CalculationsType's bits
    #[error(
        "BITS is too large ({} is the maximum, {} was given)",
        CalculationsType::BITS,
        BITS
    )]
    BitsConstantTooLarge,

    /// Value given to ConstrainedNum uses more bits than the given generic constant BITS
    #[error("Value \"{}\" uses more bits than allowed ({} allowed, {} used)", .value, BITS, .used_bits)]
    ValueUsesTooManyBits {
        value: CalculationsType,
        used_bits: u32,
    },
}
