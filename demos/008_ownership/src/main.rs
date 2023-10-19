fn main() {
    _deep_clone_string();
}


fn _scope() {
    // {                      // s is not valid here, it’s not yet declared
    //     let s = "hello";   // s is valid from this point forward

    //     // do stuff with s
    // }   // this scope is now over, and s is no longer valid
}

fn _string() {
    // let mut s = String::from("hello");

    // s.push_str(", world!");

    // println!("{}", s);
}

fn _wrong_string_2() {
    // let s1 = String::from("hello");

    // let s2 = s1;

    // println!("{}, world", s1); // borrowed here after move <-- which is wrong
}

fn _deep_clone_string() {
    let s1 = String::from("hello");

    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}