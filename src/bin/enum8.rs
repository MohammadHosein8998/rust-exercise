enum Milk {
    lowFat(i32),
    Whole,
    Coconut
}

impl Milk {
    fn drink(self){
        match self {
            Milk::lowFat(2)=>{
                println!("Delicious , 2% milk is my favorite")
            }
            Milk::lowFat(percent)=>{
                println!("you have got the lowFat {percent} percent version")
            }
            Milk::Whole =>{
                println!("you got whole milk");
            }
            _ =>{
                println!("you get milk any way !")
            }
        }
    }
}


fn main(){
    Milk::lowFat(1).drink();
    Milk::lowFat(2).drink();
    Milk::Whole.drink();
    Milk::Coconut.drink();

}