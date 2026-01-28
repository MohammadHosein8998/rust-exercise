fn operation(great_Success: bool) -> Result<&'static str, &'static str> {
    if great_Success {
        Ok("success")
    } else {
        Err("Error")
    }
}

fn main() {
    let result = operation(true);

    let content = match result {
        Ok(message) => message,
        Err(error) => error
    };

    // using unwrap method for this action
    println!("{}" , result.unwrap());
    println!("{}" , result.unwrap());
    println!("{}" , result.unwrap());
    println!("{}" , result.unwrap());
}
