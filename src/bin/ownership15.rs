fn main() {
    let mut burger = String::from("Burger");
    burger.push_str(" and milkShake"); 
    add_fries(burger);
    // let meal = burger;
}

fn add_fries(mut meal : String){
    meal.push_str(" and fries");
    println!("{meal}")
}
