

fn main(){
    open_store("brooklyn");
    bake_pizza(32 , "pepperoni");
    swim_in_profit();
}

fn open_store(neighborhood: &str){
    println!("Opening my pizza store in {neighborhood}");   
}


fn bake_pizza(number : i32 , topping: &str){
     println!("Backing {number} {topping} pizza");
}

fn swim_in_profit(){
    println!("so mush $$$ , so little time !");
}