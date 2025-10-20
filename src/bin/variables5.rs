fn main() {
    let coffee_price = 100;

    {
        let tea_price = 200;
        println!("coffee_price : {}", coffee_price);


        println!("tea_price : {}", tea_price);
    }

    // println!("tea_price : {}", tea_price); //  error cannot find value `tea_price` in this scope
}
