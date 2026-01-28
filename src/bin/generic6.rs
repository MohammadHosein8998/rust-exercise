

enum ChessesTeak<T> {
    Plain,
    Topping(T)
}


fn main(){
    let mushroom = ChessesTeak::Topping("mushroom");
    let onion = ChessesTeak::Topping("onion".to_string());

    let topping = "bacon".to_string();

    let mut plain : ChessesTeak<String> = ChessesTeak::Plain;

    plain = ChessesTeak::Topping(55.to_string());
    
}