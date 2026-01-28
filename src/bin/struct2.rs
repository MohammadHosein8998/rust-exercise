struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}

fn main() {
    let mut mocha = make_coffee(String::from("Mocha"), 300.12, true);
    drink_coffee(&mut mocha);

    println!("{}" , mocha.name);
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}

fn drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my Delicious {} with cost {}", (*coffee).name , coffee.price);
    coffee.is_hot = false;
}
