use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    read_username_from_file();
}

fn panic_directly() {
    panic!("crush and burn");
}

fn panic_at_runtime() {
    let v = vec![1, 2, 3];

    v[99];
}

fn fn_might_fail() {
    let file = File::open("non_exist_file.txt");

    let result = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {:?}", error),
    };
}

fn handle_error_kind() {
    let file = File::open("non_exist_file.txt");

    let result = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", error),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        },
    };
}

fn with_unwrap() {
    let file = File::open("non_exist_file.txt").unwrap();
}

fn with_expect() {
    let file = File::open("non_exist_file.txt").expect("不应该不存在这个文件");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
