use std::io;

fn main() {
    let y: i32 = loop {
        let mut _input = String::new();
        println!("enter a number");
        io::stdin()
            .read_line(&mut _input)
            .expect("fail to readline");

        match _input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    let number = _fibonacci(y);
    println!("fibonacci {y} is {number}")
}

fn _fibonacci(mut x: i32) -> i32 {
    let mut result = 0;

    while x != 0 {
        result += x;
        x -= 1;
    }

    result
}
