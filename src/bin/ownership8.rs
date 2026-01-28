

fn main(){
    let orange = String::from("8");
    let apples = 6;
    count_my_apple(apples);

    count_my_orange(orange.clone());
    println!("{apples} apples is still valid.");
    println!("{orange} oranges is still valid.");


}

fn count_my_apple(value : i32){
    println!("your apple value is {value}");
}

fn count_my_orange(value : String){
    println!("your Orange value is {value}");
}