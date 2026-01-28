

fn main(){
    let mut coffee = String::from("mocha");
    let a = &mut coffee;
    println!("{a}");

    let b  = a;

    println!("{b}");
}