fn main() {
    //    let mut full_name = String::from("Sylvester") ;
    //     let last_name = String::from("Stallone");
    //     full_name.push(' ');
    //     full_name.push_str(&last_name);
    //     println!("{}" , full_name);

    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");

    let full_name = first_name + &last_name;

    println!("{}", full_name);

    println!("{}" , last_name);
    
}
