use std::collections::HashSet;


fn main(){
    let mut concert_queue: HashSet<&str> = HashSet::new();
    println!("{:#?}", concert_queue);
    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    
    println!("{:#?}", concert_queue);
    println!("{:#?}", concert_queue.len());

    concert_queue.insert("Megan");
    
    println!("{:#?}", concert_queue);
    println!("{:#?}", concert_queue.remove("Molly"));
    println!("{:#?}", concert_queue.remove("Franny")); 


    println!("{:#?}", concert_queue.contains("Megan"));
    println!("{:#?}", concert_queue.contains("Fred"));
    
    println!("{:#?}", concert_queue.get("Fred"));
    println!("{:#?}", concert_queue.get("Megan"));
}