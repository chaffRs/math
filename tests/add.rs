use chaff_math::add;

#[test]
fn add_via_public_api() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(0, 0), 0);
}
