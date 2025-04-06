use super::sizes::CalculationsType;

/// A numerical struct restricting the value it holds to have a limited amount of bits
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ConstrainedNum<const BITS: u32>(CalculationsType);