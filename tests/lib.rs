extern crate playground;

#[test]
fn euler1() {
    assert_eq!(23, playground::euler1(10));
    assert_eq!(78, playground::euler1(20));
    assert_eq!(233_168, playground::euler1(1_000));
}

#[test]
fn euler2_imperative() {
    assert_eq!(44, playground::euler2_imperative(100));
    assert_eq!(4613732, playground::euler2_imperative(4_000_000));
}
