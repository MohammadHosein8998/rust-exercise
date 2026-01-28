fn main() {
    let orange = String::from("orange");
    println!("{orange} this is in main method!!");
    print_my_value(&orange);
    println!("{orange} this is in main method!!");
}

fn print_my_value(value: &String) {
    println!("your value is {}", *value);
}
