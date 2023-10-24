fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("the area of the rectangle is {} square pixels.", area(&rect));
    println!("the area of the rectangle is {:?} square pixels.", rect);
    println!("the area of the rectangle is {:#?} square pixels.", rect);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 20,
    };

    dbg!(&rect2); // 这里 &rect2 因为不希望 dbg 拿走所有权，只是借用

    let rect3 = Rectangle {
        width: 33,
        height: 2
    };

    println!("rect3's area is {}", rect3.area());

    println!("wether the width is empty, {}", rect3.width());

    let rect11 = Rectangle {
        width: 30,
        height: 40,
    };

    let rect22 = Rectangle {
        width: 100,
        height: 50,
    };

    let rect33 = Rectangle {
        width: 20,
        height: 20,
    };

    println!("whether rect11 can hold rect22 {}", rect11.can_hold(&rect22));
    println!("whether rect11 can hold rect22 {}", rect11.can_hold(&rect33));

    let rect44 = Rectangle::square(11); // 调用不需要 instance 的方法
    dbg!(rect44);

    Rectangle::foo();
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // &self 是 self: &Self 的缩写
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle { // 可以有多个 impl 块
    fn foo() {
        println!("nothing in foo");
    }
}