#[derive(PartialEq , Eq)]
struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}


fn main() {
    let division = 0.0 / 0.0;
    println!("{}" , division);
}
