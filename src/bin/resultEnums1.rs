fn main(){

    let ok : Result<i8 , &str> = Result::Ok(5);
    println!("{ok:?}");
    let disaster: Result<i32 , &str> = Result::Err("Something went wrong");
    println!("{:?}" , disaster);
}