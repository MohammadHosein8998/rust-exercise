fn main(){
    let mut current_meal =  String::from("bread");
    add_flour(&mut current_meal);
    show_my_meal(&current_meal)
}

fn add_flour(meal : &mut String){
    meal.push_str(" add flour!");
}

fn show_my_meal(message: &String){
    println!("Meal Steps : {message}")
}