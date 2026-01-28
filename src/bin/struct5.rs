#[derive(Debug)]
struct computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }
}
fn main() {
    let mut my_computer = computer::new(String::from("intel 7th"), 32, 2500);
    
    println!("{:#?}" ,my_computer);
    
    my_computer
        .upgrade_cpu(String::from("intel 11th"))
        .upgrade_hard_drive_capacity(5000)
        .upgrade_memory(64);

    println!("{:#?}" ,my_computer);
}
