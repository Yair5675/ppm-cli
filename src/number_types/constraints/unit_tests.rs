use super::{BitsConstraintError, ConstrainedNum};
use crate::number_types::sizes::CalculationsType;

#[test]
fn valid_value_within_bit_limit() {
    let val = 0b1010; // 4 bits
    let result = ConstrainedNum::<4>::new(val);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().0, val);
}

#[test]
fn valid_value_at_exact_bit_limit() {
    let val = 0b1111; // 4 bits
    let result = ConstrainedNum::<4>::new(val);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().0, val);
}

#[test]
fn value_uses_too_many_bits() {
    let val = 0b10000; // 5 bits
    let result = ConstrainedNum::<4>::new(val);
    assert!(matches!(
        result,
        Err(BitsConstraintError::ValueUsesTooManyBits { value, used_bits })
            if value == val && used_bits == 5
    ));
}

#[test]
fn zero_bits_is_invalid() {
    let result = ConstrainedNum::<0>::new(1);
    assert!(matches!(result, Err(BitsConstraintError::ZeroBitsGiven)));
}

#[test]
fn bits_exceeding_calculation_type() {
    let attempted = ConstrainedNum::<{ CalculationsType::BITS + 1 }>::new(1);
    assert!(matches!(
        attempted,
        Err(BitsConstraintError::BitsConstantTooLarge)
    ));
}

#[test]
fn minimal_valid_value() {
    let val = 1;
    let result = ConstrainedNum::<1>::new(val);
    assert!(result.is_ok());
}
