fn is_item_in_stock(item_is_in_system : bool , item_is_in_stock : bool)-> Option<bool>{
    if item_is_in_stock && item_is_in_system {
        Some(true)
    }else if item_is_in_system {
        Some(false)
    }else {
        None
    }
}

fn main(){
    let availability = is_item_in_stock(false, false);

    match availability {
        Some(true )=> println!("Item is Available"),
        Some(false )=> println!("Item is not Available in system"),
        None => println!("your item doesn't exist in system"),
    }
}