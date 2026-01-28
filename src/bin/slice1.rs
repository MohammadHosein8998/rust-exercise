
fn do_hero_stuff(hero_name: &str){
println!("{hero_name} save a day");
}


fn main(){
    // let my_arr = [21,23,45,56,78,98];

    let hero = String::from("Mr incredible");
    do_hero_stuff(&hero);
    let an_other_hero = "arnold shrodinger";
    do_hero_stuff(an_other_hero);
}