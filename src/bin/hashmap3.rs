use std::collections::HashMap;

fn main() {
    let mut coffee_pairing: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("oat milk");
    coffee_pairing.insert(&drink, &milk);
    coffee_pairing.insert("Flat white", "Almond Milk");
    println!("{:#?}", coffee_pairing);
    println!("{:#?}", coffee_pairing.len());
    println!("{drink} {milk}");
    coffee_pairing.insert("Latte", "Pistachio Milk");

    println!("{:#?}", coffee_pairing);

    coffee_pairing.entry("Cappuccino").or_insert("cow milk");

    println!("{:#?}" , coffee_pairing);
}
