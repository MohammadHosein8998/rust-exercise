

fn main(){

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");

    let pizza_toppings = vec![pepperoni , mushroom , sausage];

    let mut delouse_topping  = pizza_toppings;

    let topping_reference = &delouse_topping[1];

    println!("the topping is {:?}" , topping_reference);
    delouse_topping.push(String::from("Olives"));
    println!("{:?}" , delouse_topping);



}