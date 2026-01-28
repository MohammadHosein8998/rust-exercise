use std::{fmt::{Debug, Display, Formatter, Result}, fs};

enum AppleType {
    RedDelicious,
    GrannySmith,
}
impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "🍎 Delicious 🍎"),
            AppleType::GrannySmith => write!(formatter, "🍏 GrannySmith🍏"),
        }
    }
}

struct Apple {
    kind: AppleType,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{} for {}", self.kind, self.price)
    }
}
impl Debug for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::GrannySmith => write!(formatter, "AppleType::GrannySmith"),
            AppleType::RedDelicious => write!(formatter, "AppleType::RedDelicious"),
        }
    }
}

impl Debug for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("** Apple **")
            .field("Kind", &self.kind)
            .field("Cost", &self.price)
            .finish() 
    }
}

impl Drop for Apple {
    fn drop(&mut self) {
        match fs::remove_file("story.txt") {
            Ok(_)=> println!("GoodBoy, my sweet apple"),
            Err(error)=> eprintln!("Error deleting the file: {error}")
        }
    }
}

fn main() {
    let lunch_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 1.24,
    };

    let dinner_snack = Apple {
        kind: AppleType::GrannySmith,
        price : 1.99
    };

    println!("{}", lunch_snack);
    println!("{:? }", lunch_snack);


    println!("{}" , dinner_snack)
}
