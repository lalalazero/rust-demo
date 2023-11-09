use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    closure_example_6();
    closure_example_8();
}

fn closure_example_6() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r: &Rectangle| r.width); // sort_by_key 接受一个实现了 FnMut 的 closure
    println!("{:#?}", list);
}

fn closure_example_7() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    list.sort_by_key(|r| {
        // sort_operations.push(value); // 报错，因为这里发生了 move，把 value 的所有权交给了 sort_operations
                                        // move 只会发生一次，下次再 move 的时候 value 已经不存在了，因此报错
        r.width // sort_by_key 接受一个实现了 FnMut 的 closure
    });
    println!("{:#?}", list);
}

fn closure_example_8() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut count = 0;

    list.sort_by_key(|r| {
        count += 1; // 这里是 count 的一个借用，而不是 move
        r.width // sort_by_key 接受一个实现了 FnMut 的 closure
    });
    println!("{:#?}, sorted in {count} operations.", list);
}

fn closure_example_5() {
    let list = vec![1, 2, 3];

    println!("Before dining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap(); // move 会强制转移所有权

    // println!("After calling closure: {:?}", list); // 报错，list 的所有权已经被转移了
}

fn closure_example_4() {
    // closure 是如何取值的之—— 可变引用

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {:?}", list); // 同时存在 可变和不可变引用，报错
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn closure_example_3() {
    // closure 是如何取值的之—— 不可变引用

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn closure_example_2() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // 报错
}

fn closure_example_1() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}
