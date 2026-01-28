
use std::fmt::{Debug, Display, Formatter, Result};


enum AppleType {
    RedDelicious,
    GrannySmith,
}
impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter ,"🍎 Delicious 🍎"),
            AppleType::GrannySmith => write!(formatter , "🍏 GrannySmith🍏")
        }
    }    
}

struct Apple {
    kind: AppleType,
    price : f64
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter , "{} for {}" , self.kind , self.price)
    }
}
impl Debug  for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::GrannySmith => write!(formatter , "AppleType::GrannySmith"),
            AppleType::RedDelicious => write!(formatter , "AppleType::RedDelicious")
        }
    }
}

impl Debug for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter , "Apple ::: [ Kind: {:?} , Price: {}]" , self.kind , self.price)
    }
}
fn main(){
    let lunch_snack = Apple{
        kind: AppleType::RedDelicious,
        price: 1.24
    } ;

    println!("{}" , lunch_snack);
    println!("{:? }" , lunch_snack);
}