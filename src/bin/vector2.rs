


fn main(){


    let mut pizz_diameter = vec![8,5,9,74,225];

    pizz_diameter.push(16);
    pizz_diameter.push(78);

    println!("{:?}" , pizz_diameter);

    pizz_diameter.insert(1, 26);

    println!("{:?}" , pizz_diameter);


    let last_pizza_diameter = pizz_diameter.pop(); 

    println!("{last_pizza_diameter:?}");


    let second_diameter_pizza = pizz_diameter.remove(1);

    println!("{second_diameter_pizza:?}");


}