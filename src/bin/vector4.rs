fn main(){

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");

    let pizza_topping = vec![pepperoni , mushroom , sausage];


    // using get method
    let option = pizza_topping.get(1);


    println!("{}" , option.unwrap_or(&"no value in this index position".to_string()));

}