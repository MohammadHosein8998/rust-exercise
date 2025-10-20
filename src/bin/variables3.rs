fn main() {
    // immutable : incapable of change.
    // mutable : capable of change.

    let gym_reps = 10;
    let mut gym_reps2 = 10;

    println!("gym_reps : {}", gym_reps);

    println!("gym_reps2 : {}", gym_reps2);

    // gym_reps = 15; //error : cannot assign twice to immutable variable

    gym_reps2 = 20;
    println!("second value gym_reps2 : {}", gym_reps2);
}
