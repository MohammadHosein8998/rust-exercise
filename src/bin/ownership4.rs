

fn main(){
    let person = String::from("mohammad hosein");

    println!("{person}");

    let member = person;

    println!("{member}");

    drop(member);

}