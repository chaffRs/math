use chaff_math::abs;

#[test]
fn abs_via_public_api() {
    assert_eq!(abs(10, 3), 7);
    assert_eq!(abs(3, 10), 7);
    assert_eq!(abs(5, 5), 0);
}
