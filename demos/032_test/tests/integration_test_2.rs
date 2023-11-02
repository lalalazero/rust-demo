use test_example;

#[test]
#[should_panic]
fn it_works() {
    test_example::Guess::new(-1);
}