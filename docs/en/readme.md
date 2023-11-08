## 创建一个 CLI 应用程序

`cargo run -- searchstring example-filename.txt`

这里的 `--` two hyphens(字号) 表示后面接的参数是给程序的，而不是传递给 cargo 的

用 rust 写一个 cli 程序 grep，接受 2 个参数，一个是 query ，一个是 file_path，程序的作用就是根据 query 在文件中查找匹配的行，并输出结果。

首先开始一把梭哈代码

```rust
fn main() {
    let args = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("query is {}", query);
    println!("search file is {}", file_path);
}
```

接下来重构，根据单一职责把功能分离，不要都写在 main 入口里
```rust
fn main() {
    let args = env::args().collect();

    let (query, file_path) = parse_config(&args);

    println!("query is {}", query);
    println!("search file is {}", file_path);
}

fn parse_config(args: &[String]) -> (&str, &str) {

    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
```
把 tuple 转化为 struct，增加类型定义
```rust
```