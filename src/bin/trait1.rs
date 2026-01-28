use std::collections::HashMap;

trait Accommodation {
    fn book(&mut self, name: &str, nights: u32) -> ();
}

trait Description {
    fn get_description(&self) -> String {
        String::from("this place is awesome")
    }
}
#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }

    fn summarize(&self) -> String {
        format!("{} {}", self.name, self.get_description())
    }
}

impl Description for Hotel {
    fn get_description(&self) -> String {
        format!("{} is th pinnacle of luxury", self.name)
    }
}

impl Accommodation for Hotel {
    fn book(&mut self, name: &str, nights: u32) -> () {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("please enjoy {}'s apartment.", self.host)
    }
}

impl Accommodation for AirBnB {
    fn book(&mut self, name: &str, nights: u32) -> () {
        self.guests.push((name.to_string(), nights));
    }
}

fn book_for_one_night(entity: &mut (impl Accommodation + Description), guest: &str) {
    entity.book(guest, 1);
}
   
fn mix_and_match<T: Accommodation + Description, U: Accommodation>(first: &mut T, second: &mut U, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 1);
}

fn main() {
    let mut hotel = Hotel::new("The Luxe");
    println!("{}", hotel.summarize());

    let mut airBnB = AirBnB::new("expensive");

    // println!("{}" , airBnB.get_description());
    book_for_one_night(&mut airBnB, "mohammad hussein");

    mix_and_match(&mut hotel, &mut airBnB, "monster");

    println!("{:#?}", airBnB);
    println!("hotel : {:#?} , airBnb : {:#?}", hotel, airBnB);
}
