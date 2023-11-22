fn main() {
    let a = 10i8;

    let b = a.plus();

    println!("{a} {b}");

    let a = MyI8(111i8);
    a.hi();
}

trait MyTrait {
    fn plus(self) -> Self;
}

impl MyTrait for i8 {
    fn plus(self) -> Self {
        self + self
    }
}

struct MyI8(i8);

impl MyI8 {
    fn hi(&self){
        println!("hi {}", self.0);
    }
}