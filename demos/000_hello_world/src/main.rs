fn main() {
    let s = String::from("中");

    println!("{}", s.len());

    let first = &s[0..3];

    println!("{}", first);
}
