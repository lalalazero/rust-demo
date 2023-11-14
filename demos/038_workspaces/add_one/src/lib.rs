use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_works() {
        let result = add_one(33);
        assert_eq!(34, result);
    }
}
