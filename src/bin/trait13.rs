use std::{clone::Clone, fmt::{Display , Result , Formatter} };

struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String,
}

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time:start_time.to_string(),
            end_time: end_time.to_string(),
        }
    }
}

impl Display for Appointment {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter , "dr {}, staring time in {} and end of the shift {}" , self.doctor , self.start_time  , self.end_time)
    }
}



impl Clone for Appointment {
    fn clone(&self) -> Self {
        Self { doctor: self.doctor.to_string(), start_time: self.start_time.to_string() , end_time: self.end_time.to_string() }
    }
}

fn main() {

    let morning_app = Appointment::new("Dr. Andrew", "08:00", "15:00");
    let replacement_app = morning_app.clone();

    println!("{}" , morning_app);
    println!("{}" , replacement_app);
}
