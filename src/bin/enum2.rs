#[derive(Debug)]
enum PaymentMethod {
    CreditCard(String),
    DebitCard(String),
    PayPal { userName: String, Password: String },
    Cash,
}

fn main() {
    let visa = PaymentMethod::CreditCard(String::from("1245-9862-7943"));

    let payPal = PaymentMethod::PayPal {
        userName: String::from("bob@gmail.com"),
        Password: String::from("Password"),
    };

    println!("{:?}" , payPal);

}
