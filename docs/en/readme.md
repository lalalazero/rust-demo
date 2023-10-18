# 根据英文文档学习 rust

## 2. 猜字游戏

源码 demos/guessing_name

什么是 prelude ？

By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude, and you can see everything in it in the standard library documentation.

prelude 就是一个标准库

比如 `use std::io;` 就是使用标准库

`&` 表示这是一个 reference

`Result` is an enumeration, enum 枚举，枚举的每个可能的值叫做 variant。Result 枚举的值可能是 Ok 或者 Err， Err 需要写 handler

`crate` 是 rust 源码文件的集合，rust 标准库暂时没有提供 random 函数。

猜字游戏是一个 binary crate，也就是可执行文件。`rand` crate 是一个 library crate，它提供了其他程序可能需要的代码，但是无法被单独执行。

也就是 rust 程序分为两种
- library crate
- binary crate，可单独执行

.toml 的依赖也遵循 semantic version 语义化版本

`rand="0.8.5"` 等同于 `rand="^0.8.5"`，代表任何 >= 0.8.5 并且 < 0.9.0 的版本

依赖的 registry 是 Crates.io ，从这里下载项目依赖

cargo build 会

- 检查依赖有没有下载，只会下载一次然后 build 依赖
- 如果源码有修改，只 build 你的源码，而不会再 build 依赖

.lock 文件会记录依赖的具体版本，而不会像 npm 一样默默升级。
因此，除非手动更改版本，依赖的版本不会自动升级。确保每次有 reproducible builds.

cargo update 用来升级依赖，但是还是会遵循语义化版本的原则

没有用到的变量要用 `_` 开头标识，否则编译会报错

A `match` expression is made up of __arms__. An `arm` consists of a __pattern__ to match against, and the code that should be run if the value given to `match` fits that arm’s pattern

### 总结

介绍了 `let` `mut`, `match`, `loop` 和依赖 rand 的安装

`&` 表示引用类型

`Result` 有两种返回值类型，`Ok` 和 `Err`

match 里面写可能得情况处理用 `,` 不用 `;`

## 3. common programming concepts 共同的编程概念

### variables 变量

变量默认是不可变的 immutable. once a value is bound to a name, you can't change that value;

constants 常量，不可使用 `mut` 修饰，始终都是不可变的，约定全大小，用 _ 连接。比如 `const THREE_HOURS_IN_SECONDS = 60 * 60 * 3`

变量的 shadow 

### basic types 基本类型

`scalar` 表示单一值的数据类型，包括整型 `integer`，浮点数`floating-point-number`, 布尔值`boolean`, 字符型（不是字符串）`character`

an integer is a number without a fractional component. 没有小数点部分的数字称为整型，默认的整型是 `i32`

signed integer 有符号的整型，包含 i8 i16 i32 i64 i128 `isize`

unsigned integer 无符号的整型，包含 u8 u16 u64 u128 `usize`

用 `''` 声明的是字符，用 `""` 声明的是字符串，rust 支持 utf-8

`compound` 类型表示复合类型，rust 有两种基础复合类型，`tuple` 和 `array`

tuple 一旦声明，大小无法改变 `let tup:(i32, f64, u8) = (500, 6.4, 1);`

tuple 的解构 `let (x, y, z) = tup;`

`array` 的长度也无法更改，需要长度可变应该用 `vector`

数组的声明方式
- `let a:[i32; 5] = [1,2,3,4,5];`
- `let a = [3; 5];`

### functions 函数

expression 表达式不能句末有 `;`

statement 语句句末总是有 `;`

语句没有返回值，表达式有返回值。表达式句末加 ; 变成语句。

一个块级代码的返回值就是最后一个 expression 计算的值

### comments 注释

// 表示注释

### control flow 控制流程

if 的参数就是严格的 bool 不会做自动类型转换

if/else if/else if/else 只会运行第一个条件为 true 的分支，其他的都不会运行

如果有很多个条件分支，应该使用 `match` 优化

### loops 循环

带标签的 loop 的标签只能是单引号开始，比如 ``outer_loop: loop { ..省略.. }`;

推荐用 for in 来遍历数组而不是用 自增的index 和 while 循环，因为：
- while 更慢
- rust 需要添加额外代码来检测 runtime 时 index 是否越界

需要使用的时候可以考虑用 `Range`

## 4. 所有权 ownership
