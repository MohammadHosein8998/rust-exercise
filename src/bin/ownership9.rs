
fn main(){
    let mut  my_car = String::from("blue Car");

    let ref1 = &mut my_car; // ref1 start life time
    ref1.push_str(" BMW");// ref1 ends life time
    let ref2 = &mut my_car;

    println!("{ref2}");
}