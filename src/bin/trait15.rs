struct BusTrip {
    origin: String,
    destination: String,
    time: String,
}

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

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

impl BusTrip {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

impl PartialEq<Flight> for BusTrip {
    fn eq(&self, other: &Flight) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}


fn main() {
    let a = Flight::new("New York", "London", "08:00");
    let b = Flight::new("New York", "London", "17:00");

    let bus_a = BusTrip::new("New York", "London", "15:00");
    let bus_b = BusTrip::new("Tehran", "Stambul", "17:00");

    let c = Flight::new("New York", "Los Angeles", "17:00");

    println!("{}", a == b);
    println!("{}", a == c);
    println!("{}", a.eq(&b));



    println!("comparing Fight with bus a : {}" , a == bus_a);
    println!("comparing Fight with bus a : {}" , a.eq(&bus_b));



    println!("comparing bus b with Flight a: {}" , bus_b.eq(&a));
}
