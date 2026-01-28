fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("can not divide by zero .".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let result = divide(10.10, 2.0);

    match result {
        Ok(calculation) => {
            println!("divide Result : {}", calculation);
        }
        Err(message) => {
            println!("Error Result : {}", message);
        }
    }
}
