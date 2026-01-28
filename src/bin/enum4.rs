#[derive(Debug)]
enum Meat {
    Chicken,
    Steak
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito(Meat),
    Bow(Meat),
    VeganPlate,
}

fn main(){
    let lunch = RestaurantItem::Burrito(Meat::Chicken);
    let dinner = RestaurantItem::Burrito(Meat::Steak);
    let abandoned_meal = RestaurantItem::VeganPlate;
    println!(" ordered lunch : {:#?}" , lunch);
    println!(" ordered dinner : {:#?}" , dinner);
    println!(" ordered meal : {:#?}" , abandoned_meal);

}