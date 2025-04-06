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

type C3 = ConstrainedNum<3>; // Max value: 0b111 = 7
type C4 = ConstrainedNum<4>; // Max value: 0b1111 = 15

#[test]
fn bitand_with_primitive() {
    let a = C4::new(0b1101).unwrap();
    let result = a & 0b0110u8;
    assert_eq!(result.0, 0b0100); // 0b1101 & 0b0110 = 0b0100
}

#[test]
fn bitor_with_primitive_masks_correctly() {
    let a = C3::new(0b001).unwrap(); // 1
    let result = a | 0b1111u8; // 0b1111 = 15, masked down to 0b111 = 7
    assert_eq!(result.0, 0b111); // Masked to 3 bits
}

#[test]
fn bitxor_with_primitive_masks_correctly() {
    let a = C3::new(0b101).unwrap(); // 5
    let result = a ^ 0b1111u8; // 5 ^ 15 = 10, 0b1010 & 0b111 = 0b010
    assert_eq!(result.0, 0b010); // Masked to 3 bits
}

#[test]
fn bitnot_masks_correctly() {
    let a = C3::new(0b001).unwrap(); // !0b001 = 0b...11111110 → mask = 0b111 → 0b110
    let result = !a;
    assert_eq!(result.0, 0b110); // Masked to 3 bits
}

#[test]
fn bitand_self() {
    let a = C4::new(0b1101).unwrap();
    let b = C4::new(0b0110).unwrap();
    assert_eq!((a & b).0, 0b0100); // 0b1101 & 0b0110 = 0b0100
}

#[test]
fn bitor_self() {
    let a = C4::new(0b0101).unwrap();
    let b = C4::new(0b0011).unwrap();
    assert_eq!((a | b).0, 0b0111); // 0b0101 | 0b0011 = 0b0111
}

#[test]
fn bitxor_self() {
    let a = C4::new(0b1111).unwrap();
    let b = C4::new(0b0101).unwrap();
    assert_eq!((a ^ b).0, 0b1010); // 0b1111 ^ 0b0101 = 0b1010
}

#[test]
fn shl_with_primitive_masks_correctly() {
    let a = C3::new(0b010).unwrap(); // 0b010 << 2 = 0b1000 → 0b1000 & 0b111 = 0b000
    let result = a << 2u8;
    assert_eq!(result.0, 0b000); // Masked to 3 bits
}

#[test]
fn shr_with_primitive() {
    let a = C4::new(0b1000).unwrap(); // 0b1000 >> 3 = 0b0001
    let result = a >> 3u8;
    assert_eq!(result.0, 0b0001); // Shift right without exceeding bits
}

#[test]
fn bitor_self_does_not_exceed_constraints() {
    let a = C3::new(0b010).unwrap();
    let b = C3::new(0b111).unwrap();
    let result = a | b;
    assert_eq!(result.0, 0b111); // Still fits within 3 bits
}

#[test]
fn bitxor_self_result_fits_constraints() {
    let a = C3::new(0b111).unwrap(); // 7
    let b = C3::new(0b101).unwrap(); // 5
    let result = a ^ b; // 7 ^ 5 = 2, fits in 3 bits
    assert_eq!(result.0, 0b010); // Result is within 3 bits
}

#[test]
fn bitand_with_max_value() {
    let a = C3::new(0b111).unwrap();
    let result = a & 0b101u8;
    assert_eq!(result.0, 0b101); // 0b111 & 0b101 = 0b101
}

#[test]
fn shl_zero_no_change() {
    let a = C4::new(0b1001).unwrap();
    assert_eq!((a << 0u8).0, 0b1001); // Shifting by 0 should not change the value
}

#[test]
fn shr_zero_no_change() {
    let a = C4::new(0b1001).unwrap();
    assert_eq!((a >> 0u8).0, 0b1001); // Shifting by 0 should not change the value
}

#[test]
fn shl_with_large_value_masks_correctly() {
    let a = C3::new(0b010).unwrap(); // 0b010 << 4 = 0b10000 → 0b10000 & 0b111 = 0b000
    let result = a << 4u8;
    assert_eq!(result.0, 0b000); // Masked to 3 bits
}
