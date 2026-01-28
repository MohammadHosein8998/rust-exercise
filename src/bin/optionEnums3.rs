fn main(){
    let musical_instrument = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    let bass = musical_instrument.get(2);
    println!("{:#?}" , bass);
    
    play(bass);
    println!("{:#?}" , bass);

    let invalid_instrument = musical_instrument.get(100);


    play(invalid_instrument );
    println!("{:#?}", invalid_instrument );
}


fn play(instrument_option : Option<&String>){
    match instrument_option{
        Option::Some(instrument )=> println!("Playing the {instrument}"),
        Option::None => println!("singing with voice."),
    }
}