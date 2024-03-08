mod common;

#[test]
fn test_add() {
    common::setup();
    assert_eq!(testing::add(3, 2), 5);
}
