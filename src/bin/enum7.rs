enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

impl LaundryCycle {
    fn wash_Laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Running the laundry with cold temperature")
            }
            LaundryCycle::Hot { temperature } => {
                println!("Running the Laundry with a temperature of {temperature}")
            }
            LaundryCycle::Delicate(fabric_type) => {
                println!("Running the Laundry with a delicate cycle for {fabric_type}")
            }
            _ => {
                println!("your item is not there yet.")
            }
        }
    }
}

fn main() {
    LaundryCycle::Cold.wash_Laundry();
    LaundryCycle::Hot { temperature: 35 }.wash_Laundry();
    LaundryCycle::Delicate(String::from("Silk")).wash_Laundry();
}
