#[derive(Debug)]
enum OperatingSystem {
    Windows,
    MacOs,
    Linux
}


fn main(){
    let my_computer = OperatingSystem::Windows;
    let age = years_since_release(&my_computer);
    println!( " {:#?} is {} years old" , my_computer , age);
}

fn years_since_release(os : &OperatingSystem)-> u32{
    match os {
        OperatingSystem::Windows => 39,
        OperatingSystem::Linux => 34,
        OperatingSystem::MacOs => 23,
    }
}