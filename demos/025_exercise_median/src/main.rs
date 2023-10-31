use std::collections::HashMap;

fn main() {
    let v = vec![23, 12, 14, 3, 22, 5, 45, 77, 3, 12, 22, 23, 11];
    let v = remove_repeat(v);
    println!("after remove repeat {:?}", v);
    let v = bubble_sort(v);
    println!("after sort {:?}", v);
    let m = median(&v);
    println!("the median value is {m}");
}

fn median(v: &Vec<u32>) -> u32 {
    let len = v.len();
    println!("len = {len}");
    if len % 2 == 0 {
        v[len/2]
    } else {
        let i: f32 = len as f32 / 2.0; // 如果 len 不 as f32，那么 9 /2 = 4 ，都是整数的运算，好神奇
        println!("{len} / 2 = {i}");
        let a = i.floor() as usize;
        let b = i.ceil() as usize;
        let mut result = 0;
        println!("i = {i}, a = {a}, b = {b}");
        for j in 0..len{
            if j == a || j == b {
                result += v[j]
            }
        }
        result / 2
    }
}

fn remove_repeat(v: Vec<u32>) -> Vec<u32> {
    let mut s = vec![];
    for x in v.iter() {
        if !s.contains(x) {
            s.push(*x)
        }
    }

    s
}

fn bubble_sort(v: Vec<u32>) -> Vec<u32> {
    let mut temp;
    let mut result = v.clone();
    let len = v.len();

    for _ in 0..len {
        for j in 0..len - 1 {
            if compare_fn(result[j], result[j + 1]) > 0 {
                temp = result[j + 1];
                result[j + 1] = result[j];
                result[j] = temp;
            }
        }
    }

    result
}

fn compare_fn(v1: u32, v2: u32) -> i32 {
    if v1 > v2 {
        1
    } else if v1 == v2 {
        0
    } else {
        -1
    }
}
