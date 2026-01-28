#[derive(Debug)]
struct song {
    title: String,
    release_year: u64,
    duration_song: u64,
}

impl song {
    fn display_song(&self) {
        println!("Title : {}", self.title);
        println!("release year : {}", self.release_year);
        println!("Duration {} seconds ", self.duration_song);
    }

    fn double_length(&mut self) {
        self.duration_song = self.duration_song * 2;
        // println!("{:#?}" , self.duration_song);
    }
}

fn main() {
    let mut eminem = song {
        title: String::from("godzilla"),
        release_year: 2023,
        duration_song: 260,
    };

    println!(" this is {}", eminem.title);

    eminem.display_song();
    println!("---------------------------");
    eminem.double_length();
    println!("---------------------------");
    eminem.display_song();

    // eminem.display_song();

    // eminem.display_song();
}
