
#[derive(Debug , Clone, Copy)]
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Uh no"),
        }
    }
    fn unwrap_or(self , fallback_value : i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value, 
        }
    }
}

fn main() {
    let some_option = MyOption::Some(100);
    let missing_value : MyOption = MyOption::None;
    println!("{:#?}" , missing_value.unwrap_or(0) );
    println!("{}", some_option.unwrap());
    println!("{:#?}", some_option);
}
