use minigrep::Config;
use std::env;
use std::process;

// 执行 rust 程序 cargo run -- hello ./poem.txt
// IGNORE_CASE=1 cargo run -- rUsT poem.txt
fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("searching for text {}", config.query);
    // println!("in file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
