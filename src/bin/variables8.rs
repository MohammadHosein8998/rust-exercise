#![allow(unused_variables)] // whole of the file
type Meters = i32;
fn main() {
    let mile_race_length: Meters = 1600;

    // compiler directive
    // #[allow(unused_variables)]
    let two_mile_race_length: Meters = 1000;

    println!("one mile length is {} !!!", mile_race_length);

    // compiler directive => line by line
    // #[allow(unused_variables)]
    let three_mile_race_length: Meters = 99855;
}
