use thiserror::Error;
use super::sizes::CalculationsType;

/// A numerical struct restricting the value it holds to have a limited amount of bits
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ConstrainedNum<const BITS: u32>(CalculationsType);

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