fn main() {
    let mut current_meal = String::new();
    current_meal = add_flour(current_meal);
    current_meal = add_sugar(current_meal);

    println!("{current_meal}")
}

fn add_flour(mut meal: String) -> String {
    meal.push_str("add flour ");
    meal
}

fn add_sugar(mut meal: String) -> String {
    meal.push_str("add sugar");
    meal
}
