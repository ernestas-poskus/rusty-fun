pub fn square(a: i32) -> i32 {
    return a * a;
}

#[test]
// #[should_panic(expected = "assertion failed")]
fn it_works() {
    assert!(true);
    assert_eq!(4, square(2));
}
