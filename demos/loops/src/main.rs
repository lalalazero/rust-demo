fn main() {
    
    _better_count_down();
}

fn _basic() {
  let mut counter = 0;

    let result =  loop {
        println!("again");

        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is {result}");
}


fn _with_label() {

    let mut count = 0;
    'outer: loop { // 注意这里只有一个单引号
        println!("count = {count}");
        let mut remaining = 11;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'outer;
            }

            remaining -= 1;
        }

        count += 1;
    };

    println!("End count = {count}");

}

fn _while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("number is {number}");

        number -= 1;
    }

    println!("over")
}

fn _for_loop() {
    let a = [1, 2, 3, 4, 5];

    for ele in a {
        println!("the value is {ele}")
    }
}

fn _better_count_down() {
    for number in (1..4).rev() {
        println!("{number}")
    }

    println!("liftoff")
}
