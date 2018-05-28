extern crate twofer;
use twofer::twofer;

#[test]
fn empty_string() {
    assert_eq!(twofer(""), "One for you, one for me.");
}

#[test]
fn alice() {
    assert_eq!(twofer("Alice"), "One for Alice, one for me.");
}

#[test]
fn bob() {
    assert_eq!(twofer("Bob"), "One for Bob, one for me.");
}
