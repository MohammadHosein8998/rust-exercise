

fn main() {
    let evaluation = true;

    match evaluation {
        true => {
            println!("evaluation is true");
        }
        false => {
            println!("evaluation is false");
        }
    }

    let season = "winter";

    match season {

        "spring"=> println!("season is spring"),
        "summer"=> println!("season is summer"),
        "winter"=> println!("season is winter"),
        "fall"=> println!("season is fall"),
        _ => println!("season is autumn")

    }

    let number = -5;

    match  number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        x if x % 2 != 0 => println!("{x} is an odd number "),
        _ => unreachable!()
        
    }
}
