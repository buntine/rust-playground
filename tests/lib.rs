extern crate playground;

#[test]
fn euler1() {
    assert_eq!(23, playground::euler1(10));
    assert_eq!(78, playground::euler1(20));
    assert_eq!(233_168, playground::euler1(1000));
}
