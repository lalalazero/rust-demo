fn main() {
    let my_name = "rust";
    greet(my_name);

    _chinese_text_wrong_example(); // 编译没问题，运行报错，因为中字占3个字节，但是切的是2个字节，导致中都切不完全
}

fn greet(name: &str) { // 这里不能用 String 因为字符串字面量和 String 不是一个东西
    println!("Hello, {}", name);
}

fn _chinese_text_wrong_example() {
    let s = "中国";
    let a = &s[0..2];
    println!("{}", a);
}
