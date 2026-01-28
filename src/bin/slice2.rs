fn main() {
    let value = [4, 5, 10, 6, 78];
    let my_slice: &[i32] = &value[0..3];
    println!("{my_slice:?}");

    let my_slice: &[i32] = &value[0..];
    println!("{my_slice:?}");

    let my_slice: &[i32] = &value[..3];
    println!("{my_slice:?}");

    let my_slice: &[i32] = &value[..];
    println!("{my_slice:?}");

    let my_slice: &[i32] = &value;
    println!("{my_slice:?}");
}
