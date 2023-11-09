用 rust 写一个 cli 程序 grep，接受 2 个参数，一个是 query ，一个是 file_path，程序的作用就是根据 query 在文件中查找匹配的行，并输出结果。

期望的使用方式是 `cargo run -- searchstring example-filename.txt`

这里的 `--` two hyphens(字号) 表示后面接的参数是给程序的，而不是传递给 cargo 的

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

接下来重构，根据单一职责把功能分离，用函数归纳功能，不要把所有的细节都写在 main 入口里
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
fn main() {
    let args = env::args().collect();

    let config = parse_config(&args);

    println!("query is {}", config.query);
    println!("search file is {}", config.file_path);
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {

    let query = &args[1];
    let file_path = &args[2];

    Config {
        query, 
        file_path,
    }
}
```
把 parse_config 和 Config 挂钩，变成它的方法，同时切断和原来 args 字符串的联系，使用 clone 再新建一份 config 自己的属性
```rust
fn main() {
    let args = env::args().collect();

    let config = Config::build(&args);

    println!("query is {}", config.query);
    println!("search file is {}", config.file_path);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {

    fn build(args: &[String]) -> Config {

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config {
            query, 
            file_path,
        }
    }
}
```
把 Config 想过逻辑放到 lib.rs 中
```rust
// 新建 lib.rs ，这里写 Config 逻辑
pub struct Config {
    query: String,
    file_path: String,
}

impl Config {

    pub fn build(args: &[String]) -> Config {

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config {
            query, 
            file_path,
        }
    }
} 
```
在 main.rs 中使用
```rust
use minigrep::Config; // 导入 Config

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args);
}
```
加上错误处理，可以直接 panic! 报错结束程序
```rust
fn build(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // 省略
}
```
用 `Result` 改写程序的返回结果
```rust
fn build(args: &[String]) -> Result<Config, &'static str> { // 返回的结果 Ok 和 Err 类型
    if args.len() < 3 {
        return Err("not enough arguments");
    }

    // 正常返回 Ok
    Ok(Config {
        query,
        file_path,
    })
}
```
在 `main` 中使用
```rust
use minigrep::Config; // 导入 Config
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1); // 退出程序
    });
}
``` 

### 总结

首先直接在 `main` 里面写代码，

然后把解析参数的过程收敛到了 `parse_config` 函数里，返回 tuple 包含参数 query，file_path，

然后增加 `struct Config` 类型约束，并把 `parse_config` 关联为 `Config` 的 `build` 方法，

将 Config 相关的逻辑写到 `lib.rs`, `main` 函数中只需要使用 lib 暴露的 API 就够了，无需关心实现细节，

接着添加错误处理，首先直接用 panic 使程序报错退出，然后再用 `Result`改写，包装返回结果，在 `main` 中根据返回结果做下一步操作（程序退出或者继续执行），

接着添加测试，

然后加了对环境变量的支持，

最后我们把错误结果用 `eprintln!` 宏输出到 `stderr`，这样和 `stdout` 区别开来，将程序的运行结果收集到 `> output.txt` 可以观察到 `stdout` 的输出结果。

