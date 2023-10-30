use std::collections::HashMap;

fn main() {
    update_1();
    update_2();
    update_3();
}

fn _basic() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // get 返回 Option<&V>，如果没有值返回 None
                                                              // copied 得到 Option<i32> 而不是原始的 Option<&i32>
                                                              // unwrap_or 设置 score = 0 如果没有值
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn _ownership_1() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // println!("{field_name} {field_value}");

}

fn update_1() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 直接覆盖掉旧的值

    println!("{:?}", scores);

}

fn update_2() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // 旧值不存在再插入

    println!("{:?}", scores);
}

fn update_3() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // 在旧值的基础上更新，返回的值是 &mut V
        *count += 1; // 解引用
    }

    println!("{:?}", map);
}
