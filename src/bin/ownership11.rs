

fn main(){
    let reference = create_city();

    println!("{reference}")
}


fn create_city() -> String {
    String::from("new york")
}