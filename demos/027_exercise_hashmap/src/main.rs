use std::collections::HashMap;

enum Department {
    Engineering,
    Sales,
}

fn main() {
    let mut map: HashMap<String, Staff> = HashMap::new();

    let user = _add_staff(String::from("Sally"), Department::Engineering);
    let name = user.name.clone();
    map.insert(name, user);
}

fn _add_staff(name: String, dept: Department) -> Staff {
    Staff::new(name, dept)
}

struct Staff {
    name: String,
    dept: Department,
}

impl Staff {
    fn new(name: String, dept: Department) -> Staff {
        Staff { name, dept }
    }
}
