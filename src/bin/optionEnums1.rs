fn main() {
    let a = Option::Some(4);
    let b = Option::Some("hello");
    let d = Option::Some(true);

    let a: Option<i32> = Option::Some(4);
    let b = Option::<i32>::Some(5);
    let d: Option<&str> = Option::None;
}
