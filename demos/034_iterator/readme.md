## 28. iterator 迭代器

rust 中的 iterators 迭代器都是 lazy 的，如下的代码并没有太大的意义

```rust
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
```

只有在 iterator 被消费(consumed) 比如在 for 循环中使用时，才是有意义的

```rust
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
```
所有的迭代器都实现了标准库中的 `Iterator` trait

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```
新语法 `type Item` 和 `Self::Item`，用来定义 `associated type` 关联类型。

每个实现了 `Iterator` trait 的都有一个 `next` 方法，可以手动调用这个 `next` 方法。比如写个测试用例
```rust
// src/lib.rs
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter(); // 注意这个 mut

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
```

但是要注意这个 `mut` 修饰，手动调用 `next` 方法的时候会改变 internal state of the iterator ，internal state 就是迭代器内部用来追踪和记录迭代顺序的状态，因此要手动用 `mut` 修饰。如果是 `for` 循环会自动帮忙处理这个可变，因此不用手动写。

还要注意在这里调用`next` 返回的值是 v1 数组的值的 immutable reference （不可变的引用），因为 `iter()` 方法返回的就是 immutable reference 的 iterator. 

如果我们对 v1 数组的值做 move 迭代，需要用 `into_iter` 方法来创建迭代器。同理还有针对创建 mutable reference 的迭代器方法，叫做 `iter_mut`

总结创建迭代器 iterator 有3个方法：
- `iter()` 迭代不可变引用
- `into_iter()` 转移迭代值的 ownership
- `iter_mut()` 迭代可变引用

consuming adaptors

什么是 consuming adaptor ？会自动调用 `.next` 方法的就叫做 consuming adaptor. `Iterator` trait 标准库中实现了很多方法，有一些就是 consuming adaptor，比如 `.sum` 方法

```rust
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum(); // 自动迭代并求和

        assert_eq!(total, 6);

        // v1.iter.next(); // 无法再调用，因为 .sum 拿取了 v1_iter iterator 的所有权
    }
```

与之相反的是 `iterator adaptor`，它们不消费 iterator，而且还会在原来的 iterator 的基础上产生新的 iterator。比如 `.map` 方法

```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
```
需要注意的是这个代码并不会执行，因为 iterator 是 lazy 的，目前为止没有代码消费 `.map` 产生的 iterator，因此 map 也不会去消费 v1.iter() 产生的 iterator，因此没有任何元素被遍历（消费）

加个 `.collect()` 就可以消费了

```rust
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

```

many iterator adaptors take closures as arguments. 许多 iterator adaptor 接受闭包作为参数，而大多数情况下这些闭包都会 capture their environment.

举个例子，`.filter()` 方法

```rust
#[test]
fn iterator_adaptor_2() {
    let v1 = vec![1, 2, 3, 4];
    let v2: Vec<_> = v1.into_iter().filter(|x| x % 2 == 0).collect(); // 注意这里为了方便计算，用的是 into_iter() 迭代

    assert_eq!(v2, vec![2, 4]);
}
```