fn main() {
    let pizza_diameters = vec![8, 10, 12, 48];

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");

    let pizza_topping = vec![pepperoni , mushroom , sausage];

    let value = &pizza_diameters[1..];


    println!("{:?}" , value);
}
