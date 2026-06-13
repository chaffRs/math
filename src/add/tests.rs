use super::*;

#[test]
fn adds_two_numbers() {
    assert_eq!(add(2, 2), 4);
}

#[test]
fn adds_with_zero() {
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(7, 0), 7);
    assert_eq!(add(0, 7), 7);
}
