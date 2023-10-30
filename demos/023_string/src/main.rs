fn main() {
    basic();
    update_string();
    concat_string();
    concat_string_2();
    concat_string_3();
    index_string();
}

fn basic() {
    let mut s = String::new();

    let data = "initial contents"; // 注意是双引号，不是单引号

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // string 是 UTF-8 编码的，因此任何语言都支持
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn update_string() {
    let mut s =  String::from("foo");
    let s2 = "--haha";
    s.push_str("bar");
    s.push_str(s2);

    println!("s is {s}");
    println!("s2 is {s2}");

    let mut s3 = String::from("lo");
    s3.push('l'); // 注意这里是单引号
    println!("s3 is {s3}");
}

fn concat_string() {
    let s1 = String::from("hello");
    let s2 = String::from(",world");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // println!("s1 is {s1}"); // s1 被 move 之后自动 drop，这里引用不再有效
    println!("s2 is {s2}");
    println!("s3 is {s3}");
}

fn concat_string_2() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {s}");
}

fn concat_string_3() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {s}");
    println!("{s1} \n{s2} \n{s3}"); // 没有发生 move
}


fn index_string() {
    let s1 = String::from("hello");
    // let h = s1[0]; // 这样是不对的，rust 编译器不知道你的 string 是什么语言什么字符，utf-8 编码的字节长度是1-4个
                   // 因此这样引用有可能是非正常边界，编译器不允许这个操作

    let len = s1.len();
    println!("{len}"); // 5 bytes

    for b in s1.bytes() {
        print!("{b} "); // 1个代表一个对应的 unicode 字符，这个字符这里就是英文
    }
    println!("");
    for c in s1.chars() {
        print!("{c} "); // 一个英文“字”
    }

    println!("");

    let s2 = String::from("今天星期一");
    let len2 = s2.len(); // 15 bytes
    println!("{len2}");
    for b in s2.bytes() {
        print!("{b} "); // 3个一组代表一个对应的 unicode 字符，这个字符这里就是中文
    }
    println!("");
    for c in s2.chars() {
        print!("{c} "); // 一个中文“字”
    }

}