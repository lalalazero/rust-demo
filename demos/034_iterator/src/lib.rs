#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                }
            ]
        )
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter(); // 这里需要加 mut 修饰

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_take_ownership() {
        let v1 = vec![1, 2, 3];

        let mut iter = v1.into_iter();

        assert_eq!(iter.next(), Some(1)); // 这里没有 &1，因为 iter.next() 返回的值类型不是 reference 了
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
    }

    #[test]
    fn iterator_mut_reference() {
        let mut v1 = vec![1, 2, 3];

        let mut iter = v1.iter_mut();

        let a = 1;

        let b = match iter.next() {
            Some(num) => *num + 10,
            None => {
                println!("value is none");
                a
            }
        };

        println!("{:?}", v1); // v1=[1, 2, 3]
        println!("{}", b); // b=11
    }

    #[test]
    fn iterator_consumer_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum(); // 自动迭代并求和

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_adaptor() {
        let v1 = vec![1, 2, 3, 4];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4, 5]);
    }

    #[test]
    fn iterator_adaptor_2() {
        let v1 = vec![1, 2, 3, 4];
        let v2: Vec<_> = v1.into_iter().filter(|x| x % 2 == 0).collect();

        assert_eq!(v2, vec![2, 4]);
    }
}
