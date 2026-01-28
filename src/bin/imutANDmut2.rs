fn main() {
    let mut cat: String = String::from("persian");
    let ref1 = &mut cat;
    ref1.push_str(" and silver ");
    let ref2 = &cat;
    println!("{ref2}");
}
