fn main(){

    let pirate = "BloodHook";
    let sailer = String::from(pirate);
    let bad_guy =  pirate.to_string();

    println!("{pirate} and {sailer} , {bad_guy}" );

    let first_initial = &pirate[0..5];

    println!("{}", first_initial);
}