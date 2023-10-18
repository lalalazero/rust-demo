fn main() {
    variables();
    mut_variables();
    variables_2();
    variables_3();
}


fn variables() {
    let x = 3;
    println!("x = {x}");
    x = 6; // 这是不被允许的，会报错
    println!("x = {x}");
}

fn mut_variables() {
    let x = 5;
    
    let x = x + 1;

    {
        let x = x * 2;
        println!("inner scope: x = {x}"); // x = 12
    }

    println!("outer scope: x = {x}"); // x = 6
}

fn variables_2() {
    let spaces = "  ";
    let spaces = spaces.len();

    println!("spaces len = {}", spaces);
}

fn variables_3() {
    let mut spaces = "  ";
    spaces = spaces.len(); // 类型转换错误

    println!("spaces len = {}", spaces);
}