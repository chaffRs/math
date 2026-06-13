use chaff_math::abs_diff;

#[test]
fn abs_diff_via_public_api() {
    assert_eq!(abs_diff(10, 3), 7);
    assert_eq!(abs_diff(3, 10), 7);
    assert_eq!(abs_diff(5, 5), 0);
}
