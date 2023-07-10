use std::collections::HashMap;
fn main() {
    let action = std::env::args().nth(1).expect("Please enter the action"); // Reading first arg
    let item = std::env::args().nth(2).expect("Please enter the item"); // Reading second arg
                                                                        // let j = env::args();

    println!("{:?}, {:?}", action, item);
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        // insert a new item into our map
        // we pass true as a value
        self.map.insert(key, true);
    }
}
