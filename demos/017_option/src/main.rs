fn main() {
    println!("Hello, Option");

    let some_number = Some(5); // some_number 的类型是 Option<i32>
    let some_char = Some('e'); // some_char 的类型是 Option<char>

    let absent_number: Option<i32> = None; // 显示声明类型，因此编译器无法从 None 反推 absent_number 的类型
}
