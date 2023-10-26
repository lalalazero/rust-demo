mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        // 虽然 Breakfast 是 pub 但是没有太大的作用
        pub toast: String,
        seasonal_fruit: String, // 这个是 private 的
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // 绝对路径调用
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径调用
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye"); // 黑麦

    meal.toast = String::from("Wheat"); // 小麦
    println!("{} toast", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); // 编译报错，不允许修改 private 属性

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn test_before_use() {
    hosting::add_to_waitlist();
}

use crate::front_of_house::hosting;
pub fn test_after_user() {
    hosting::add_to_waitlist();
}

mod customer {
    // 相当于另一个封闭的模块
    use crate::front_of_house::hosting; // 不加这个找不到 hosting
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist(); // 不加上面第二行就会报错
    }
}
