use add_one;

// cargo run -p adder 执行
fn main() {
    let sum = 10;
    println!("Hello, world! {sum} plus one is {}", add_one::add_one(sum));
}
