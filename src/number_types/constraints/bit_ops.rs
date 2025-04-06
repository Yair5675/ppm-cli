use super::{CalculationsType, ConstrainedNum};
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};

impl<const BITS: u32, T: Into<CalculationsType>> BitAnd<T> for ConstrainedNum<BITS> {
    type Output = Self;

    fn bitand(mut self, rhs: T) -> Self::Output {
        // Bitand never adds bits, so it is safe to use:
        self.0 &= rhs.into();
        self
    }
}

impl<const BITS: u32, T: Into<CalculationsType>> BitOr<T> for ConstrainedNum<BITS> {
    type Output = Self;
    fn bitor(mut self, rhs: T) -> Self::Output {
        // Bitor can potentially make us exceed bits if rhs uses more bits than allowed, so we need
        // to mask the result:
        self.0 = (self.0 | rhs.into()) & *Self::max();
        self
    }
}

impl<const BITS: u32, T: Into<CalculationsType>> BitXor<T> for ConstrainedNum<BITS> {
    type Output = Self;

    fn bitxor(mut self, rhs: T) -> Self::Output {
        // Bitxor can potentially make us exceed bits if rhs uses more bits than allowed, so we need
        // to mask the result:
        self.0 = (self.0 ^ rhs.into()) & *Self::max();
        self
    }
}

impl<const BITS: u32> Not for ConstrainedNum<BITS> {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        // Not can increase bits so mask the result:
        self.0 = (!self.0) & *Self::max();
        self
    }
}

impl<const BITS: u32, T: Into<CalculationsType>> Shr<T> for ConstrainedNum<BITS> {
    type Output = Self;

    fn shr(mut self, rhs: T) -> Self::Output {
        // Shr never increases bits, only decreasing them, so don't mask:
        self.0 >>= rhs.into();
        self
    }
}

impl<const BITS: u32, T: Into<CalculationsType>> Shl<T> for ConstrainedNum<BITS> {
    type Output = Self;

    fn shl(mut self, rhs: T) -> Self::Output {
        // Shl could potentially increase bits, so mask the result:
        self.0 = (self.0 << rhs.into()) & *Self::max();
        self
    }
}
