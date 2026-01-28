#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl TreasureChest<[&str ; 3]> {
    fn amount_of_treasure(&self) -> usize {
        self.treasure.len()
    }
}


impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string()
    }
}

impl<T> TreasureChest<T> {
    fn capital_captain(&self)-> String {
        self.captain.to_uppercase()
    }
}

fn main() {
    let gold_chest = TreasureChest {
        captain: String::from("FireBeard"),
        treasure: "Gold",
    };

    let mut silver_chest: TreasureChest<String> = TreasureChest {
        captain: String::from("BloodSail"),
        treasure: String::from("                silver            "),
    };

    silver_chest.clean_treasure();
    println!("{:#?}" ,silver_chest.treasure );

    let special_hest = TreasureChest {
        captain: String::from("Bootypluder"),
        treasure: ["gold", "silver", "Platinum"],
    };

    println!("the length of the treasure : {}" , special_hest.amount_of_treasure());

    println!("{:?}", special_hest);

    println!("gold_chest captain : {} " , gold_chest.capital_captain());
    println!("silver_chest captain : {} " , silver_chest.capital_captain());

}
