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