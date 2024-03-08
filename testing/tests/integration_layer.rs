mod common;

#[test]
fn test_add_v2() {
    common::setup();
    assert_eq!(testing::add(3, 2), 5);
}
