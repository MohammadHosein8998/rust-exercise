#[derive(Debug)]
enum paymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String,String),
}

fn main() {
    let visa = paymentMethodType::CreditCard(String::from("0034-5678-9012-3456"));
    let master_card: paymentMethodType = paymentMethodType::PayPal(String::from("2525-3541"), String::from("password"));

    println!("the visa {:#?} " , visa);
    println!("the master_card {:#?} " , master_card);
    

}
