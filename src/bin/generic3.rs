fn make_tuple<T , U>(first: T, second: U) -> (T, U) {
    (first, second)
}

fn main() {
    make_tuple("hello", " world");
    make_tuple("hello", 34.87);
    make_tuple(true, "world"); 
 
}
