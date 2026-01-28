#[derive(Debug)]
struct DeliSandwich{}

fn main() {
    println!("{}", identity::<i32>(5));
    println!("{}", identity::<String>(String::from("hello world")));
    println!("{}", identity::<f32>(3.14));
    println!("{}", identity::<&str>("hello from stack &str"));
    println!("{}", identity::<bool>(true));
    println!("{:?}", identity(DeliSandwich {}));
    println!("{}", identity(true));
    println!("{:#?}", identity::<DeliSandwich>(DeliSandwich {}));
}

fn identity<T>(value: T) -> T {
    value
}
 