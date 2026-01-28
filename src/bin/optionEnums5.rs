

fn main(){
 let present_value = Some(13);
 let missing_value : Option<&String>= None;
let present_bool_value : Option<bool> = Some(true);
println!("{:#?}" , present_bool_value.unwrap_or(false));
 println!("{:#?}" , present_value.unwrap_or(0));
println!("{:#?}" , missing_value.unwrap_or(&"there is no value".to_string()));
}
