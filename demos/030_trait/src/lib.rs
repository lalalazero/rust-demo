use std::fmt::Display;

// pub mod aggregator { // 不声明这个 mod， use 的时候少写一层
pub trait Summary {
    fn summarize_author(&self) -> &String;
    fn summarize(&self) -> String {
        let s1 = String::from("Read more...");
        let s2 = self.summarize_author();

        format!("{}, {}", s1, s2)
    }
}
pub trait Debug {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // 采用默认实现
    // fn summarize(&self) -> String {
    //     format!("{}, by {} {}", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> &String {
        &self.author
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> &String {
        &self.username
    }
}

pub fn notify(item: &impl Summary) {
    // 接受任何一个实现了 Summary trait 接口的类型作为参数
    println!("Breaking news {}", item.summarize());
}

pub fn notify_equivalency<T: Summary>(item: &T) {
    // impl xxxTrait 是这个的语法糖，两者等价
    println!("Breaking news {}", item.summarize());
}

pub fn notifyAll(item1: &impl Summary, item2: &impl Summary) {} // item1 和 item2 可以是不同类型，只要它们都实现了 summary trait
pub fn notifyAll_2<T: Summary>(item1: &T, item2: &T) {} // item1 和 item2 需要是同样的类型，并且实现了 summary trait

pub fn notify_multi(item: &(impl Summary + Display)) {} // item 需要都实现 summary 和 display trait
pub fn notify_multi_2<T: Summary + Display>(item: &T) {} // 两个 trait bounds，分别是 summary 和 display

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    1
}
fn some_function_2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

fn return_summarizable() -> impl Summary { // 表示这个返回值的类型是实现了 Summary trait 的类型
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 返回类型不是单一类型时，无法用 impl xxxtrait 声明类型
// fn returns_summarizable_wrong_case(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> { // 只有实现了 Display 和 PartialOrd trait 的 T 才能调用 cmp_display 方法
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest number is x = {}", self.x);
        }else{
            println!("the largest number is y = {}", self.y);
        }
    }
}

// } // mod 边界
