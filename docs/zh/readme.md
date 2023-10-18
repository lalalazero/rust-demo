## rust 资源

- rust 英文官网 https://doc.rust-lang.org/stable/book/
- rust 标准库 https://doc.rust-lang.org/std/
- prelude 标准库 https://doc.rust-lang.org/stable/std/prelude/index.html
- rust 中文圣经 https://course.rs/about-book.html
- rust 语言中文社区 https://rustcc.cn/
- rusty book 锈书 https://rusty.course.rs/

## 记录

- 第一天

### 安装环境 

- 安装 rust，使用 wsl 安装的 ubuntu 系统

    `$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

    检测是否安装成功

    `rustc -V` 或者 `rustc --version`

    `cargo -V` 或者 `cargo --version`

- 安装 gcc toolchain

`sudo apt-get install build-essential`

- 安装 vscode 插件
    - rust-analyzer
    - even better toml
    - error lens

### 创建 hello-world

Rust 项目主要分为两个类型：bin 和 lib，前者是一个可运行的项目，后者是一个依赖库项目。

cargo new hello-world

cargo run 运行

target 目录下分为 debug 和 release 两个文件夹

cargo check: 快速检测代码是否能编译通过

cargo.toml 类似 package.json

cargo.lock 类似 package.json 的 lock 文件

什么情况下该把 Cargo.lock 上传到 git 仓库里？很简单，当你的项目是一个可运行的程序时，就上传 Cargo.lock，如果是一个依赖库项目，那么请把它添加到 .gitignore 中

### 修改镜像

### 基本概念