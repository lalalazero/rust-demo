fn main() {
    let mut a: u32 = 10;
    let b = &a;
    a = 20;
    
    println!("{}", b);
}
