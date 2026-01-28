use std::{collections::HashMap, fmt::Display};

trait Accommodation {
    fn book(&mut self, name: &str, nights: u32) -> ();
}

trait Description {
    fn get_description(&self) -> String {
        String::from("this place is awesome")
    }
}
#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

impl<T> Hotel<T> {
    fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    fn summarize(&self) -> String {
        format!("{} {}", self.name, self.get_description())
    }
}

impl<T: Display> Description for Hotel<T> {
    fn get_description(&self) -> String {
        format!("{} is th pinnacle of luxury", self.name)
    }
}

impl<T> Accommodation for Hotel<T> {
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

fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    first.book(guest, 1);
    second.book(guest, 1);
}

fn main() {
    let mut hotel = Hotel::new(String::from("The Luxe"));
    let mut airBnb = AirBnB::new("the Golden standard");

    let mut stays: Vec<&mut dyn Accommodation> = vec![&mut hotel , &mut airBnb];

    stays[0].book("Pier", 2);
    stays[1].book("Amanda", 3);

    println!("{:#?}" , hotel);
    println!("{:#?}" , airBnb);
}
