fn main() {
    println!("example 1");
    let a = String::from("Hello world");
    let len = _first_word_1(&a);
    let word = &a[..len];

    println!("len is {}, word = {}", len, word); // 输出是 5 hello

    let a = String::from("中 文");
    let len = _first_word_1(&a);
    let word = &a[..len];

    println!("len is {}, word = {}", len, word); // 输出是 3 中

    // println!("{}", &a[..2]); // 运行时错误，中用3个字节存，但是你切的是2个字节，中都没切完

    // -----------------------------

    println!("\nexample 2");
    let mut s = String::from("hello world");
    let word = _first_word_2(&s);

    println!("first word is {}", word); // 正常运行

    s.clear();

    // println!("first word is {}", word); // 编译错误
    // 原因 在 println 这一行，同时存在可变和不可变引用，s.clear() 引用可变的，println 引用的是不可变的

    // -----------------------------
    println!("\nexample 3");
    let my_string = String::from("hello world");

    let word = _first_word_3(&my_string[0..6]);
    println!("{word}");
    let word = _first_word_3(&my_string[..]);
    println!("{word}");
    let word = _first_word_3(&my_string);
    println!("{word}");

    let my_string_literal = "Hello World";

    let word = _first_word_3(&my_string_literal[0..6]);
    println!("{word}");
    let word = _first_word_3(&my_string_literal[..]);
    println!("{word}");
    let word = _first_word_3(&my_string_literal);
    println!("{word}");

    // -----------------------------
    println!("\nexample 4");
    _array_slice_example();
}

fn _first_word_1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn _first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn _first_word_3(s: &str) -> &str {
    // 有经验的老手会这样写，这里会发生隐式类型转换
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn _array_slice_example() {
    let a = [1, 2, 3, 4, 5];

    let slices = &a[1..3];
    assert_eq!(slices, &[2, 3])
}
