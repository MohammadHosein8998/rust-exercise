#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

fn main() {
    let gold_chest = TreasureChest {
        captain: String::from("FireBeard"),
        treasure: "Gold",
    };

    let silver_chest: TreasureChest<String> = TreasureChest {
        captain: String::from("BloodSail"),
        treasure: String::from("silver"),
    };

    let special_hest = TreasureChest {
        captain: String::from("Bootypluder"),
        treasure: ["gold", "silver", "Platinum"],
    };

    println!("{:?}", special_hest);
}
