use test_example::{add_two};

mod common;

#[test]
fn it_adds_two() {
    common::setup(); // 初始化

    assert_eq!(5, add_two(2));
}