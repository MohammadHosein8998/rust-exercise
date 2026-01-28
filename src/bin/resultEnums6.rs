

fn main(){
    let mut sauce = vec!["Mayonnaise" , "Ketchup" , "Ranch"];

    // if let Some(sauce) = sauce.pop() {
    //     println!("the next sauce is {sauce}")
    // }
    // if let Some(sauce) = sauce.pop() {
    //     println!("the next sauce is {sauce}")
    // }

    // if let Some(sauce) = sauce.pop() {
    //     println!("the next sauce is {sauce}")
    // }

    // if let Some(sauce) = sauce.pop() {
    //     println!("the next sauce is {sauce}")
    // }

    // if let Some(sauce) = sauce.pop() {
    //     println!("the next sauce is {sauce}")
    // }


    // while let construct
    while let Some(sauce) = sauce.pop() {
        println!("the next sauce is {sauce} : ");
    }


}