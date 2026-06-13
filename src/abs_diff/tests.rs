use super::*;

#[test]
fn abs_left_greater_than_right() {
    assert_eq!(abs_diff(10, 3), 7);
}

#[test]
fn abs_left_less_than_right() {
    assert_eq!(abs_diff(3, 10), 7);
}

#[test]
fn abs_equal_operands_is_zero() {
    assert_eq!(abs_diff(5, 5), 0);
    assert_eq!(abs_diff(0, 0), 0);
}

#[test]
fn abs_with_zero() {
    assert_eq!(abs_diff(7, 0), 7);
    assert_eq!(abs_diff(0, 7), 7);
}

#[test]
fn abs_is_commutative() {
    assert_eq!(abs_diff(42, 100), abs_diff(100, 42));
    assert_eq!(abs_diff(0, u64::MAX), abs_diff(u64::MAX, 0));
}

#[test]
fn abs_u64_bounds() {
    assert_eq!(abs_diff(u64::MAX, 0), u64::MAX);
    assert_eq!(abs_diff(u64::MAX, u64::MAX), 0);
    assert_eq!(abs_diff(u64::MAX, 1), u64::MAX - 1);
}

#[test]
fn abs_signed_integers() {
    assert_eq!(abs_diff(-5_i32, 3), 8);
    assert_eq!(abs_diff(3_i32, -5), 8);
    assert_eq!(abs_diff(-7_i32, -7), 0);
}

#[test]
fn abs_f64_basic() {
    // 1.5, 0.5, 1.0 are all exactly representable, so == is safe here.
    assert_eq!(abs_diff(1.5_f64, 0.5), 1.0);
    assert_eq!(abs_diff(0.5_f64, 1.5), 1.0);
    assert_eq!(abs_diff(2.0_f64, 2.0), 0.0);
}

#[test]
fn abs_f64_nan_is_nan() {
    assert!(abs_diff(f64::NAN, 1.0).is_nan());
    assert!(abs_diff(1.0, f64::NAN).is_nan());
}
