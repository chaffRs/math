use super::*;

#[test]
fn abs_left_greater_than_right() {
    assert_eq!(abs(10, 3), 7);
}

#[test]
fn abs_left_less_than_right() {
    assert_eq!(abs(3, 10), 7);
}

#[test]
fn abs_equal_operands_is_zero() {
    assert_eq!(abs(5, 5), 0);
    assert_eq!(abs(0, 0), 0);
}

#[test]
fn abs_with_zero() {
    assert_eq!(abs(7, 0), 7);
    assert_eq!(abs(0, 7), 7);
}

#[test]
fn abs_is_commutative() {
    assert_eq!(abs(42, 100), abs(100, 42));
    assert_eq!(abs(0, u64::MAX), abs(u64::MAX, 0));
}

#[test]
fn abs_u64_bounds() {
    assert_eq!(abs(u64::MAX, 0), u64::MAX);
    assert_eq!(abs(u64::MAX, u64::MAX), 0);
    assert_eq!(abs(u64::MAX, 1), u64::MAX - 1);
}
