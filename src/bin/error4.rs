use std::fs::File;
use std::io::stdin;
use std::process;

fn main() {
    println!("pleas enter the name of the file you'd like to read.");

    let mut input = String::new();

    if let Err(error) = stdin().read_line(&mut input) {
        eprintln!("Something went wrong collecting user input. the error was {error}");
        process::exit(1)
    }// story.txt\n

    let file = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Something went wrong reading the file . the error was {error}");
            process::exit(1)
        }
    };


    println!("{file:#?}")
}
