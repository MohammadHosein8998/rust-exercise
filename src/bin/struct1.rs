
struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn main() {
    
    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 9.99,
        is_hot: true,
    };

    println!(
        "mt {} morning const was {} and it was so hot : {}",
        mocha.name, mocha.price, mocha.is_hot
    );
}
