

fn main(){
    let person = String::from("mohammad hosein");
    let member = person.clone();


    println!("this is {person}");

    println!("this is {member}");
    drop(person);
    drop(member);

}