fn main() {
    foo1();
    foo2();
    foo3();
    foo4();
    foo5();
    foo6();
}

fn foo1() {
    let a: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut an_iter = a.into_iter(); // 将Vec<u32>转换为迭代器

    while let Some(i) = an_iter.next() {
        // 调用 .next() 方法
        println!("{i}");
        // 输出
        // 1
        // 2
        // 3
        // 4
        // 5
    }
}

fn foo2() {
    let mut a = [1, 2, 3]; // 一个整数数组

    let mut an_iter = a.iter(); // 转换成第一种迭代器

    assert_eq!(Some(&1), an_iter.next());
    assert_eq!(Some(&2), an_iter.next());
    assert_eq!(Some(&3), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.iter_mut(); // 转换成第二种迭代器

    assert_eq!(Some(&mut 1), an_iter.next());
    assert_eq!(Some(&mut 2), an_iter.next());
    assert_eq!(Some(&mut 3), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.into_iter(); // 转换成第三种迭代器，并消耗掉a

    assert_eq!(Some(1), an_iter.next());
    assert_eq!(Some(2), an_iter.next());
    assert_eq!(Some(3), an_iter.next());
    assert_eq!(None, an_iter.next());

    println!("foo2 output {:?}", a); // a 集合里的元素执行了 copy
}

fn foo3() {
    let mut a = ["1".to_string(), "2".to_string(), "3".to_string()];
    let mut an_iter = a.iter();

    assert_eq!(Some(&"1".to_string()), an_iter.next());
    assert_eq!(Some(&"2".to_string()), an_iter.next());
    assert_eq!(Some(&"3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.iter_mut();

    assert_eq!(Some(&mut "1".to_string()), an_iter.next());
    assert_eq!(Some(&mut "2".to_string()), an_iter.next());
    assert_eq!(Some(&mut "3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());

    let mut an_iter = a.into_iter();

    assert_eq!(Some("1".to_string()), an_iter.next());
    assert_eq!(Some("2".to_string()), an_iter.next());
    assert_eq!(Some("3".to_string()), an_iter.next());
    assert_eq!(None, an_iter.next());

    // println!("foo3 output {:?}", a); // 编译不通过，a 集合里的元素执行了 move
}

fn foo4() {
    println!("foo4 output");
    let mut a = ["1".to_string(), "2".to_string(), "3".to_string()];

    for item in &a {
        println!("{}", item);
    }

    for item in &mut a {
        println!("{}", item);
    }

    for item in a {
        // 请想一想为什么要把这一句放在后面
        println!("{}", item); // 这个 for 循环导致发生了 move，因此要放在后面
    }

    // println!("foo4 output {:?}", a);  // 编译报错，已经 move 不能再在 println! 中 borrow

    // 输出
    // 1
    // 2
    // 3
    // 1
    // 2
    // 3
    // 1
    // 2
    // 3
}

fn foo5() {
    let s1 = String::from("aaa");
    let s2 = String::from("bbb");
    let s3 = String::from("ccc");
    let s4 = String::from("ddd");

    let v = vec![s1, s2, s3, s4];
    // let a = v[0]; // move 操作不可以
    let a = &v[0]; // 明确a只获得v中第一个元素的引用, borrow 可以
    for s in v {
        // 这里，s拿到了集合元素的所有权
        println!("foo5 output {}", s);
    }
}

fn foo6() {
    let v = vec![1, 2, 3];
    let a = v[0]; // move 可以
}
