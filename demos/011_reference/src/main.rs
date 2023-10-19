fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hahaha");

    _can_modify_mut_reference(&mut s2);

    println!("s2 is {s2}");

    _multi_mut_references();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// fn _cannot_modify_borrow_value(s: &String) {
// s.push_str("~~") // compile error
// }

fn _can_modify_mut_reference(s: &mut String) {
    s.push_str("~~~")
}

fn _data_race_with_two_mut_reference() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; // 这是不对的

    // println!("{}, {}", r1, r2);
}

fn _multi_mut_references() {
    let mut s = String::from("hello");

    {
        // 作用域开始
        let r1 = &mut s;

        r1.push_str("-1");

        println!("scope 1 {}", s);
    } // 作用域结束

    let r2 = &mut s;
    r2.push_str("-2");

    println!("scope 2 {}", s);
}

// fn _mut_and_immut_reference_wrong() {
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM

//     println!("{}, {}, and {}", r1, r2, r3);

// }

fn _mut_and_immut_reference_correct() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// fn _dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away. // 离开这个函数的作用域，s 被释放销毁，但是你返回了一个指针指向它的地址？错误！
//   // Danger!

fn _dangle_reference_example() {
    // let reference_to_nothing = _dangle();
    _no_dangle();
}

fn _no_dangle() -> String {
    let s = String::from("hello");

    s
}
