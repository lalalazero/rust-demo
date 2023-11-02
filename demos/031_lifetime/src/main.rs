fn main() {
    longest_example();
    longest_example_2();
}

// 引用了一个超出有效范围内的值
// An attempt to use a reference whose value has gone out of scope
// fn dangling_reference_wrong_example() {
//     let r;                // ---------+-- 'a // r 的 lifetime 生命周期用 'a 来表示
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |  // x 的 lifetime 用 'b 表示
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {}", r); //          |
// }                         // ---------+


// 正确的生命周期
// fn correct_example() {
//     let x = 5;            // ----------+-- 'b
//                           //           |
//     let r = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {}", r); //   |       | // 'a 的 lifetime 比 'b 小，'b 活的更久，因此 r 有效的时候 x 一定有效，这样就是对的
//                           // --+       |
// }                         // ----------+


fn longest_example() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest_example_2() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("the longest string is {}", result);
    }
}

// fn longest_example_3() {
//     let string1 = String::from("long string is long");
//     let result;

//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }

//     println!("the longest string is {}", result); // 报错，为啥？ string2 的 lifetime 是 inner scope, 但是在 out scope 被引用到了
                                                   // 虽然从例子中看 runtime 运行的结果 string1 才是 result 的值
                                                   // 但是 rust 编译器认为这样是不对的
                                                   // 因为我们已经通过注解 'a 告诉了编译器 s1 和 s2 和 result 都应该是相同的 lifetime
// }

// fn longest_wrong(x: &str, y: &str) -> &str { // 这个方法返回的可能是 x, 也可能是 y
//                                                 这样 rust 没法判断引用 result 的地方到底应该追溯 x 还是 y 的 lifetime
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 这个 lifetime annotation 注解的意思是
// all the references in the signature must have the same lifetime 'a
// 所有的引用都有同样的生命周期 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else{
        y
    }
}

// 也可以返回值只和用到的参数 lifetime 保持一致，没有用到的不设置
fn longest_2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// 如果返回值是函数内创建的变量
fn longest_3<'a>(x: &str, y: &str) -> String {
    let result = String::from("a long string");

    result // 这样是对的，返回值的类型也要改成 String，这样其实也不涉及到外部的 lifetime 的追溯了，可以把 'a 去掉
    // result.as_str() // 这样是错的，对应的返回值的类型是 &str
                       // 这个返回是 result 的一个 reference，而 result 变量自身随着 longest_3 的执行完毕，lifetime 就结束了

}

// struct 中的 lifetime
// 这个标记的 lifetime 表示
// an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
// ImportantExcerpt 的实例不能比它的 part 属性引用的 str 活得更久
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn struct_lifetime() {
    let novel = String::from("call me Ishmael. Some years ago..");
    let first_sentence = novel.split('.').next().expect("Cound not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// 在 rust v1.0 之前，这个函数签名需要声明 lifetime
// lifetime elision rules 总结了一些特定的 pattern，无需开发者声明 lifetime，编译器也能正确计算 lifetime
fn first_word_before_v1<'a>(s: &'a str) -> &'a str { &s[..]}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}