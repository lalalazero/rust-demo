## 27. closure 闭包

### 1. 认识闭包

rust 中的闭包：anonymous functions that capture their environment. 保留了执行环境的匿名函数。

一个最简单的闭包表达式 closure expression `|| self.most_stocked()`

如果闭包有参数，参数在双竖线中间， `|abc|`

和 function 相反的是，closure 不要求严格的类型注释。通常闭包使用的范围非常小，编译器可以 infer their types of the parameters and the return types. （自动推导参数和返回值的类型）

但是如果你非要个 closure 加类型注解也可以，比如
```rust
let expensive_closure = |num: u32 | -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
}
```

对比 function 和 closure 的类型注解的完整写法

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }; // function definition
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // fully annotated closure definition
let add_one_v3 = |x|             { x + 1 }; // 不带 type annotation 的 closure
let add_one_v4 = |x|               x + 1  ; // 既不带 type annotation 也不带 {}，因为这里 body 只有一句
```
其中 v3 和 v4 需要具体的上下文来知道 x 的类型，就像 `let v = Vec::new();` 要么显式声明类型，要么插入一个值，这样编译器才会知道 v 具体是什么类型。

再举个例子，example_closure 没有具体使用之前，编译器不知道 x 的类型，但是一旦使用（ evaluate）之后，x 的类型就确定了。那么再把 x 当作另外的类型使用就会报错。
```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5); // 报错
```

### 2. closure 如何 capture reference 或者 move ownership

closure 获取值的方式有3种，正如 function 获取参数的 3 种方式：
- borrowing immutably 借用不可变引用
- borrowing mutably 借用可变引用
- take ownership 拿取所有权

具体是哪一种方式取决于 closure body 怎么使用拿到的值

但是即便 closure body 不需要拿取 ownership，也可以通过 `move` 关键字强制拿取所有权，这个强制所有权转移通常在 new thread 的时候很有用。
新建线程，强制 data 的所有权从旧的线程转移到新的。

这个例子中，move 关键字是强制必须的。因为 main 新起了一个线程，谁也不知道 main 线程和新线程谁会更快结束运行，如果不是显式 move，main 线程优于新线程结束后会导致新线程找不到 list 而报错（这是 runtime 的时候可能会报的错），因此这里需要 move 关键字。去掉之后 rust 编译器也会提示你 `may outlive borrowed value list`，也是说明 list 的生命周期的问题。

```rust
fn main() {
    let list = vec![1, 2, 3];

    println!("Before dining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();

    // println!("After calling closure: {:?}", list); // 报错，list 的所有权已经被转移了
}
```

### 3. moving captured values out of closures 以及 `Fn` traits

**怎么理解 moving captured values out of closures?**

一开始我理解的是 closure 的返回值，但是通过例子我觉得这样是错误的理解。

这句话应该就是从字面意思上去理解，有 move 动作发生就是命中了这句话。而有 move 动作发生，不一定是 closure body 内部转移到外部，也有可能是从外界 A 的身上转移到外界 B 的身上（参见 sort_by_key 这个排序的例子，为什么 closure_example_7 是错误的）。

**Fn** traits 的实现

根据 closure body 对环境值的处理决定了 closure 实现的 traits. 而 traits 决定了 functions 和 structs 能用的 closures 的类型。

也就是根据对值的处理闭包会自动实现 1-3 种 `Fn` traits

- `FnOnce` 每个闭包都会实现，意为每个闭包都能被调用。而一个特定的从 closure body 中 move out captured value 只会实现这个 trait，其他的都不实现，因为这个闭包只能被调用一次。
- `FnMut` 适用不会把 value move out of its closure body 的闭包且可能会 mutate the captured values，可以被调用多次。
- `Fn` 适用于既不 move out 值也不会 mutate 引用值的闭包，或者不 capture 任何值（没有参数）的闭包。也可以被调用多次，而且由于这类闭包的特性，所以导致不会对 environment 造成任何影响（无副作用），这在并发执行多次的场景下是一个重要特性。

举个例子，标准库的 `Option` 有一个方法 `unwrap_or_else`，这个方法会判断 `Option` 的值（值的类型是T），如果有值，返回该值，否则会调用参数 f，f 的类型有3个点：

1. f 可以被调用，并且 f 没有参数
2. f 实现了 `FnOnce` trait，表示只会执行一次
3. f 会返回类型 T

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```
因为所有的 closure 都实现了 `FnOnce` 这个 trait，所以可以写
```rust
fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
       user_preference.unwrap_or_else(|| self.most_stocked()) // 这个 closure 没有参数，而且有返回值，实现了 FnOnce trait
   }
```

再来看 `FnMut` 的例子
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    closure_example_6();
}

fn closure_example_6() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width); // sort_by_key 接受一个实现了 FnMut 的 closure
    println!("{:#?}", list);
}
```
`sort_by_key`会多次调用 closure（数组里的每个元素都会调用一次），而这个 closure `|r| r.width` 不会 capture，mutate 或者 move out anything from its environment. 所以满足 `FnMut` 这个 trait。

这里怎么理解 move out anything from its environment ？

看起来闭包里面返回了 r.width，但是这个 r 是一个 borrow reference（类型是 &Rectangle），访问 r.width 不会转移 width 的所有权，所以这里没有 move 发生，这样就说的通了。

**总结**
1. 闭包可以 capture, mutate, move values of its environment
2. 根据第 1 条闭包对值的操作，闭包自动会具有相关 `Fn` traits
3. `FnOnce`，适用于有 move 动作发生的闭包，move 意味着只能被 call once，同时所有的闭包都可以被 call once，因此所有的闭包都会实现 `FnOnce`
4. `FnMut`，适用于可以 call multiple 的闭包，从 multiple 反推，闭包不能有 move 动作，但是可以有 capture, mutate
5. `Fn` 适用于可以 call multiple 的闭包，而且没有 mutate，没有副作用，可以有 capture 也可以没有，没有意味着参数为空，在 concurrency 场景会是一个重要特性。