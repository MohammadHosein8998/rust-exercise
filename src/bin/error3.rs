use std::{fs::File, process};




fn main(){
    let file = match File::open("story.txt"){
        Ok(file)=> file,
        Err(error)=> {
            eprintln!("Something went wrong reading the file . the error was {error}");
            process::exit(1)
        }
    };

    println!("{file:?}");

}