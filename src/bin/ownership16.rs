fn main (){
    let cake = bake_cake();
    println!("{cake} is baked")
}

fn bake_cake()-> String{
    String::from("chocolate cake")
}