fn main() {
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");

    let mut pizza_toppings = vec![pepperoni, mushroom, sausage];

    pizza_toppings[1].push_str(" and Souse");
    let topping = &pizza_toppings[1];
    print!("{pizza_toppings:#?}");

    let target_topping = &mut pizza_toppings[2];
    //  this will be panic because collusion in mutable and immutable barrow
    // let another_topping = &pizza_toppings[2];
    target_topping.push_str(" and MeatBalls");

    println!("{pizza_toppings:#?}");
}
