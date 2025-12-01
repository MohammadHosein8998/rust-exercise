fn count_down(second: i32) {
    if second != 0 {
        let number = if second % 2 == 0 { "even" } else { "odd" };
        println!("number {second} is an {number} number");
        count_down(second - 1);
    } else {
        println!(" Over ");
    }
}

fn main() {
    count_down(30);

    count_down(30);

    count_down(30);

}
