
fn main() {
    println!("Hello, match");

    let coin = Coin::Quarter2(UsState::Alaska);

    value_in_cents(coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

#[derive(Debug)] // 这句要写在这里哦，不然 println 会报错哦
enum UsState {
    Alabama,
    Alaska,
    // ---snip--- 省略
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Quarter2(UsState),
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin { 
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::Quarter2(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn catch_all_example() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        _ => (), // 也可以这样，啥也不干，用 _ 代替参数
        // other => (), // 这样会有一个没有用的参数的警告
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num: i32) {}

fn without_if_let() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("the maximum is configured to be {}", max),
        _ => (), // 啥也不干
    }
}

fn with_if_let() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("the maximum is configured to be {}", max); // 注意这里是 ; 号哦！！
    }
}

fn without_if_let_else() {
    let mut count = 0;
    let coin = Coin::Dime;
    match coin {
        Coin::Quarter2(state) => println!("state quarter"),
        _ => count += 1,
    };
}

fn with_if_let_else() {
    let mut count = 0;
    let coin = Coin::Dime;

    if let Coin::Quarter2(state) = coin {
        println!("state quarter")
    }else {
        count += 1;
    }
    
}