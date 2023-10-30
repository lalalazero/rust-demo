fn main() {
    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let v4 = vec![1, 2, 3, 4, 5];
    let third = &v4[2];
    println!("the third element is {third}");

    let third: Option<&i32> = v4.get(2);
    match third {
        Some(third) => println!("the third element is {third}"),
        None => println!("there is no third element"),
    }

    // let does_not_exist = &v4[100]; // 编译通不过
    let does_not_exist = v4.get(100); // 编译能通过

    iterate_vector();
    iterate_mut_vector();
    with_enum();
    drop_values();
}

fn cant_have_mut_and_immut_reference_same_time() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable

    v.push(6);

    // println!("the first element is {first}"); // within same scope, exist both mut and immut reference;
}

fn iterate_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("iterator over vector, the item is {i}");
    }
}

fn iterate_mut_vector() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
}

fn with_enum() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("hello")),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(int) => println!("int {int}"),
            SpreadsheetCell::Float(f) => println!("float {f}"),
            SpreadsheetCell::Text(String) => println!("string {String}"),
            // _ => (), // do nothing
        }
    }
}

fn drop_values() {
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}
