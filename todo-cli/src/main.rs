fn main() {
    let action = std::env::args().nth(1).expect("Please enter the action");
    let item = std::env::args().nth(2).expect("Please enter the item");

    println!("{:?}, {:?}", action, item);
}
