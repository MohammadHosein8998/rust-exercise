#[derive(Debug)]
enum Musician {
    SingerSongWriter(String),
    Band(u32),
}

use Musician::{Band, SingerSongWriter};

impl PartialEq for Musician {
    fn eq(&self, other: &Self) -> bool {
        match self {
            SingerSongWriter(name) => match other {
                SingerSongWriter(other_name) => name == other_name,
                Band(_) => false,
            },
            Band(members) => match other {
                SingerSongWriter(_) => false,
                Band(other_member) => members == other_member,
            },
        }
    }
}

fn main() {
    let eminem = SingerSongWriter("Godzilla".to_string());
    let mickle_Jackson = SingerSongWriter("they care about us".to_string());
    let holly = SingerSongWriter("Holly".to_string());

    let rust_no_one = Band(5);
    let unrustWorthy = Band(4);
    let rust_for_vengeance = Band(5);

    println!("{}", eminem == rust_for_vengeance);
    println!("{}", mickle_Jackson == unrustWorthy);
    println!("{}", rust_for_vengeance == rust_no_one);

    println!("{}", rust_for_vengeance == mickle_Jackson);
    println!("{}", unrustWorthy == holly);
}
