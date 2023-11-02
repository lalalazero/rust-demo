#[cfg(test)]
mod tests {
    use super::*; // tests 是一个单独的 module，因此要引用外部定义的 Rectangle struct 和相关方法

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }


    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_2()-> Result<(), String>{
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equals four"))
        }

    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        // panic!("Make this test fail!");
    }

    #[test]
    #[ignore]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value as `{}`", result);
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn greater_than_100(){
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_2() {
        Guess::new2(200);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    #[test]
    fn test_private_add() {
        assert_eq!(4, private_add(2, 2));
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 3
}

pub fn greeting(name: &str) -> String {
    format!("Hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    pub fn new2(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }

        Guess { value }
    }
}

fn private_add(a: i32, b: i32) -> i32 {
    a + b
}