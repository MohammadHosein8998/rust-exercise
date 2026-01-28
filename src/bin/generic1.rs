#[derive(Debug)]
struct DeliSandwich{}

fn main() {
    println!("{}", identity(5));
    println!("{}", identity(String::from("hello world")));
    println!("{}", identity(3.14));
    println!("{}", identity("hello from stack &str"));
    println!("{}", identity(true));
    println!("{:?}", identity(DeliSandwich {}));
    println!("{}", identity(true));
}

fn identity<T>(value: T) -> T {
    value
}
 