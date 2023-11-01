fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = find_largest_i32(&number_list);

    println!("The largest number is {}", largest);

    let largest = find_largest(&number_list);
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = find_largest_i32(&number_list);

    println!("The largest number is {}", result);

    let largest = find_largest(&number_list);
    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = find_largest_char(&char_list);

    println!("the largest char is {}", result);

    let largest = find_largest(&char_list);
    println!("The largest number is {}", largest);

    new_point();
    new_point2();
}

fn find_largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn find_largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T>{
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> { // 只针对 f32 类型
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt() // 计算到原点的距离
    }
}

fn new_point() {
    let integer = Point { x: 1, y: 2 }; // 整型
    let float = Point { x: 1.22, y: 2.0 }; // 浮点型

    println!("x = {}", integer.x()); // 获取 x
    println!("x = {}", float.x()); // 获取 x

    println!("{}", float.distance_from_origin()); // 不报错
    // integer.distance_from_origin(); // 报错
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y
        }
    }
}

fn new_point2() {
    let p1 = Point2 { x: 1, y: 2.3 }; // 两种类型
    let p2 = Point2::<f32, String>  { x: 1.0, y: "hello".to_string() } ;

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

enum Option1<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}