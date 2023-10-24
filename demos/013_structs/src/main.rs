fn main() {
    println!("Hello structs");

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from(""),
        sign_in_count: 1,
    };

    let user1_1 = User {
        active: user1.active,
        username: user1.username.clone(), // 这里要加 .clone() 哦
        email: String::from("another@domain.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user1_2 = User {
        email: String::from("another@domain.com"),
        ..user1 // 不能调换顺序，移到上一行会报错
    };

    let user2 = build_user("haha".to_string(), "wowo".to_string());

    // user1.email = String::from("example@somedomain.com");

    let point = build_point(1,2,3);

    let object = AlwaysEqual;
}

struct User {
    username: String, // 这里用 String 而不是 &str 是刻意的，因为这样可以保证 user 对 username 的所有权，而不是一个借用的东西
                      // 也可以用 &str 但是要涉及到 rust 的特性 lifetimes 这个特性用来保证 
                      // the data referenced by a struct is valid for as long as the struct is
    active: bool,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    } // 这个函数返回的就是 User 的实例 instance
}

struct Point(i32, i32, i32);

fn build_point(x: i32, y: i32, z: i32) -> Point {
    Point(x, y, z)
}


struct AlwaysEqual;
