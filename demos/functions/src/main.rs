fn main() {
    println!("Hello, world!");

    foo_function(5, 'a');

    statements();

    let x = five();

    println!("the value of x is {x}");

    let y = add(11);

    println!("the value of y is {y}");
}


fn foo_function(x: i32, s: char) {
    println!("foo function {x} {s}");
}

fn statements() {
    let _x = 6;

    // let x = let y = 7; // 抛错

    // let x = (let y = 7); // 抛错

    let y = {
        let x = 3;
        x + 1
    }; // 这是一个 expression，句末没有 ; 
    // statement 语句句末有 ; expression 加上 ; 就变成语句了，语句不会有返回值，express 表达式才有返回值

    println!("the value of y is: {y}")
}

fn five() -> i32 {
    5555
}

fn add(x: i32) -> i32 {
    x + 10 // 不能加分号，否则会变成语句
}
