#[cfg(test)]
mod tests {

    // expect() unwrap() unwrap_or() unwrap_or_default() 使用示例
    #[test]
    #[should_panic]
    fn test() {
        // Option
        let x = Some("value");
        assert_eq!(x.expect("fruits are healthy"), "value");
        // Result
        let path = std::env::var("IMPORTANT_PATH")
            .expect("env variable `IMPORTANT_PATH` should be set by `wrapper_script.sh`");
    }

    #[test]
    fn test2() {
        // Option
        let x = Some("air");
        assert_eq!(x.unwrap(), "air");
        // Result
        let x: Result<u32, &str> = Ok(2);
        assert_eq!(x.unwrap(), 2);
    }

    #[test]
    fn test3() {
        // Option
        assert_eq!(Some("car").unwrap_or("bike"), "car");
        assert_eq!(None.unwrap_or("bike"), "bike");

        // Result
        let default = 2;
        let x: Result<u32, &str> = Ok(9);
        assert_eq!(x.unwrap_or(default), 9);

        let x: Result<u32, &str> = Err("error");
        assert_eq!(x.unwrap_or(default), default);
    }

    #[test]
    fn test4() {
        // Option
        let x: Option<u32> = None;
        let y: Option<u32> = Some(12);

        assert_eq!(x.unwrap_or_default(), 0);
        assert_eq!(y.unwrap_or_default(), 12);

        // Result
        let good_year_from_input = "1909";
        let bad_year_from_input = "190blarg";
        let good_year = good_year_from_input.parse().unwrap_or_default();
        let bad_year = bad_year_from_input.parse().unwrap_or_default();

        assert_eq!(1909, good_year);
        assert_eq!(0, bad_year);
    }

    // Option<T> 上的常用方法示例
    #[test]
    fn test5() {
        let maybe_some_string = Some(String::from("Hello, World!"));
        let maybe_some_len = maybe_some_string.map(|s| s.len());
        assert_eq!(maybe_some_len, Some(13));

        let x: Option<&str> = None;
        assert_eq!(x.map(|s| s.len()), None);
    }

    #[test]
    fn test6() {
        let x = 12;
        let opt_x = Some(&x);
        assert_eq!(opt_x, Some(&12));
        let cloned = opt_x.cloned();
        assert_eq!(cloned, Some(12));
    }

    #[test]
    fn test7() {
        let x: Option<u32> = Some(2);
        assert_eq!(x.is_some(), true);

        let x: Option<u32> = None;
        assert_eq!(x.is_some(), false);
    }

    #[test]
    fn test8() {
        let x: Option<u32> = Some(2);
        assert_eq!(x.is_none(), false);

        let x: Option<u32> = None;
        assert_eq!(x.is_none(), true);
    }

    #[test]
    fn test9() {
        let text: Option<String> = Some("Hello, world!".to_string());
        let text_length: Option<usize> = text.as_ref().map(|s| s.len());
        println!("still can print text: {text:?}");
    }

    #[test]
    fn test10() {
        let mut x = Some(2);
        match x.as_mut() {
            Some(v) => *v = 42,
            None => {}
        }
        assert_eq!(x, Some(42));
    }

    #[test]
    fn test11() {
        let mut x = Some(2);
        let y = x.take();
        assert_eq!(x, None);
        assert_eq!(y, Some(2));

        let mut x: Option<u32> = None;
        let y = x.take(); // 拿一个出来用，但是没有消解原来的 Option
        assert_eq!(x, None);
        assert_eq!(y, None);
    }

    #[test]
    fn test12() {
        let mut x = Some(2);
        let old = x.replace(5);
        assert_eq!(x, Some(5));
        assert_eq!(old, Some(2));

        let mut x = None;
        let old = x.replace(3);
        assert_eq!(x, Some(3));
        assert_eq!(old, None);
    }

    #[test]
    fn test13() {
        fn sq_then_to_string(x: u32) -> Option<String> {
            x.checked_mul(x).map(|sq| sq.to_string())
        }

        assert_eq!(Some(2).and_then(sq_then_to_string), Some(4.to_string()));
        assert_eq!(Some(1_000_000).and_then(sq_then_to_string), None); // overflowed!
        assert_eq!(None.and_then(sq_then_to_string), None);
    }

    // Result<T, E> 上的常用方法示例

    #[test]
    fn test14() {
        let line = "1\n2\n3\n4\n";

        for num in line.lines() {
            match num.parse::<i32>().map(|i| i * 2) {
                // .map()
                Ok(n) => println!("{n}"),
                Err(..) => {}
            }
        }
    }

    #[test]
    fn test15() {
        let x: Result<i32, &str> = Ok(-3);
        assert_eq!(x.is_ok(), true);

        let x: Result<i32, &str> = Err("Some error message");
        assert_eq!(x.is_ok(), false);
    }

    #[test]
    fn test16() {
        let x: Result<i32, &str> = Ok(-3);
        assert_eq!(x.is_err(), false);

        let x: Result<i32, &str> = Err("Some error message");
        assert_eq!(x.is_err(), true);
    }

    #[test]
    fn test17() {
        let x: Result<u32, &str> = Ok(2);
        assert_eq!(x.as_ref(), Ok(&2));

        let x: Result<u32, &str> = Err("Error");
        assert_eq!(x.as_ref(), Err(&"Error"));
    }

    #[test]
    fn test18() {
        fn mutate(r: &mut Result<i32, i32>) {
            match r.as_mut() {
                Ok(v) => *v = 42,
                Err(e) => *e = 0,
            }
        }
        let mut x: Result<i32, i32> = Ok(2);
        mutate(&mut x);
        assert_eq!(x.unwrap(), 42);
        let mut x: Result<i32, i32> = Err(13);
        mutate(&mut x);
        assert_eq!(x.unwrap_err(), 0);
    }

    #[test]
    fn test19() {
        fn sq_then_to_string(x: u32) -> Result<String, &'static str> {
            x.checked_mul(x)
                .map(|sq| sq.to_string())
                .ok_or("overflowed")
        }

        assert_eq!(Ok(2).and_then(sq_then_to_string), Ok(4.to_string()));
        assert_eq!(Ok(1_000_000).and_then(sq_then_to_string), Err("overflowed"));
        assert_eq!(
            Err("not a number").and_then(sq_then_to_string),
            Err("not a number")
        );
    }

    #[test]
    fn test_20() {
        fn stringify(x: u32) -> String {
            format!("error code: {x}")
        }
        let x: Result<u32, u32> = Ok(2);
        assert_eq!(x.map_err(stringify), Ok(2));
        let x: Result<u32, u32> = Err(13);
        assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));
    }

    // Option<T> 和 Result<T, E> 互相转换
    #[test]
    fn test_21() {
        let x = Some("foo");
        assert_eq!(x.ok_or(0), Ok("foo"));

        let x: Option<&str> = None;
        assert_eq!(x.ok_or(0), Err(0));

        let x: Result<u32, &str> = Ok(2);
        assert_eq!(x.ok(), Some(2));

        let x: Result<u32, &str> = Err("Nothing here");
        assert_eq!(x.ok(), None);

        let x: Result<u32, &str> = Ok(2);
        assert_eq!(x.err(), None);

        let x: Result<u32, &str> = Err("Nothing here");
        assert_eq!(x.err(), Some("Nothing here"));
    }
}
