

fn main(){
 let my_number = square(2);

 println!("my number is {}" , my_number);

}


fn square(number : i32) -> i32 {
    return number * number;
}


// THIS IS CORRECT TOO
// fn square(number : i32) -> i32 {
//      number * number
// }