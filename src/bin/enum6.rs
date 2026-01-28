enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

fn main() {
    wash_Laundry(LaundryCycle::Cold);
    wash_Laundry(LaundryCycle::Hot { temperature: 35 });
    wash_Laundry(LaundryCycle::Delicate(String::from("Silk")));
}

fn wash_Laundry(cycle: LaundryCycle) {
    match cycle {
        LaundryCycle::Cold => {
            println!("Running the laundry with cold temperature")
        }
        LaundryCycle::Hot { temperature } => {
            println!("Running the Laundry with a temperature of {temperature}")
        }
        LaundryCycle::Delicate(fabric_type) => {
            println!("Running the Laundry with a delicate cycle for {fabric_type}")
        }
    }
}
