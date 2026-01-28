fn main(){
    let musical_instrument = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass = musical_instrument.get(2);
    println!("{:#?}" , bass);

    let valid_instrument: &String = bass.expect("Unable to retrieve element");

    println!("{valid_instrument}");
    
    let invalid_instrument = musical_instrument.get(100);
    println!("{:?}" , invalid_instrument);

    invalid_instrument.expect("Unable to retrieve element");
}