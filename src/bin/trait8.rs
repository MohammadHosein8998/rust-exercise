
trait Taxable {
    const TAX_RATE : f64 = 0.25;     

    fn tax_bill(&self) -> f64;
}
#[derive(Debug)]
struct Income{
    amount : f64
}

impl Taxable for Income {
    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
}

struct Doubt{
    amount : f64
}

impl Taxable for Doubt {
    const TAX_RATE : f64 = 0.5;

    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
    
}



fn main(){
    let income = Income { amount : 50000.0};
    println!("Total Tax : {:#?}" , income);
    println!("Total Tax : {:.2}" , income.tax_bill());
}