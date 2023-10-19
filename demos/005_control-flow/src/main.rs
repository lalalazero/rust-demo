fn main() {
    correct_example_1();
    correct_example_2();
    correct_example_3();
    correct_example_4();
}

fn correct_example_1() {
    let number = 3;
    if number < 5 {
        println!("true")
    }else{
        println!("false")
    }
}

fn correct_example_2() {
    let number = 3;

    if number != 0{
        println!("true")
    }
}

fn correct_example_3() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn correct_example_4() {
    let condition = true;
    let num = if condition { 5 } else { 6 };

    println!("the value of num is {num}")
}


fn _wrong_example_1() {
    // let number = 3;

    // if number {
    //     println!("true")
    // }
}

fn _wrong_example_2() {
    // let condition = true;
    // let num = if condition { 5 } else { "six" };

    // println!("the value of num is {num}")
}