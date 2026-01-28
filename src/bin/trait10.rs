trait Investment<T> {
    fn amount(&self) -> T;

    fn set_amount(&mut self, new_amount: T);

    fn double_amount(&mut self);
}

trait Taxable: Investment<f64> {
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}
#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Investment<f64> for Income {
    fn amount(&self) -> f64 {
        self.amount
    }
    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }

    fn double_amount(&mut self) {
        self.amount = self.amount * Self::TAX_RATE;
    }
}

impl Taxable for Income {}

struct Doubt {
    value: f64,
}

impl Investment<f64> for Doubt {
    fn amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.value = new_amount;
    }

    fn double_amount(&mut self) {
        self.value = self.value * Self::TAX_RATE;
    }
}

impl Taxable for Doubt {
    const TAX_RATE: f64 = 0.5;
}

struct QualityTime {
    minutes: u64,
}

impl Investment<u64> for QualityTime {
    fn amount(&self) -> u64 {
        self.minutes
    }
    
    fn set_amount(&mut self, new_amount: u64) {
        self.minutes = new_amount;
    }

    fn double_amount(&mut self) {
        self.minutes = self.minutes * 2;
    }
}

fn main() {
    let mut income = Income { amount: 50000.0 };
    let mut doubt = Doubt { value: 50000.0 };
    println!("Total Tax : ${:#?}", income);
    println!("Total Tax : ${:.2}", income.tax_bill());

    println!("Total tax owed : ${:.2}", income.tax_bill());
    println!("Bonus tax owned : ${:.2}", doubt.tax_bill());

    let weekend = QualityTime { minutes: 120 };
    println!("Relaxation Time {:.2}", weekend.minutes);
}
