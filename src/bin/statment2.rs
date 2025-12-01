fn main() {
    let mut seconds = 10;

    loop {
        if seconds == 0 {
            println!("BOOM");
            break;
        }

        if seconds == 4 {
            println!("just 3 second to blow up");
            seconds -= 1;
            continue;
        }
        println!("{seconds} seconds tpo blow ...");

        seconds -= 1;
    }
}
