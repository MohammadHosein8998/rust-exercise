trait Investment {
    fn amount(&self) -> f64;

    fn set_amount(&mut self, new_amount: f64);

    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0);
    }
}

trait Taxable: Investment {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}
#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Investment for Income {
    fn amount(&self) -> f64 {
        self.amount
    }
    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }
}

impl Taxable for Income {}

struct Doubt {
    value: f64,
}

impl Investment for Doubt {
    fn amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.value = new_amount;
    }
}

impl Taxable for Doubt {
    const TAX_RATE: f64 = 0.5;
}


struct QualityTime {
    minutes : f64
}

impl Investment for QualityTime {
    fn amount(&self) -> f64 {
       self.minutes
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.minutes = new_amount;
    }
}

fn main() {
    let mut income = Income { amount: 50000.0 };
    let mut doubt = Doubt { value: 50000.0 };
    println!("Total Tax : ${:#?}", income);
    println!("Total Tax : ${:.2}", income.tax_bill());

    income.double_amount();
    doubt.double_amount();
    println!("Total tax owed : ${:.2}", income.tax_bill());
    println!("Bonus tax owned : ${:.2}", doubt.tax_bill());

    let weekend = QualityTime {minutes : 120.0};
    println!("Relaxation Time {:.2}" , weekend.minutes);
}
