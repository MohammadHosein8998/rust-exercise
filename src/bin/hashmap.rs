use std::collections::HashMap;



fn main(){
    let mut menu: HashMap<String ,  f64> = HashMap::new();

    menu.insert(String::from("Steak"), 29.99);
    menu.insert(String::from("Tuna"), 29.99);
    menu.insert(String::from("Burger"), 14.99);


    println!("{menu:?}");

    let mut country_capital : HashMap<&str , &str>= HashMap::new();
    country_capital.insert("France", "Paris");
    country_capital.insert("Germany", "Berlin");
    println!("{country_capital:?}");
    
}