use std::ops::Add;
use std::str::FromStr;



fn main(){
    let a = 5;
    let b = 10;
    let sum = a.add(b);

    println!("{ }"  , sum);

    let numeric_count = u64::from_str("5").unwrap();

    println!("{}" , numeric_count)
}