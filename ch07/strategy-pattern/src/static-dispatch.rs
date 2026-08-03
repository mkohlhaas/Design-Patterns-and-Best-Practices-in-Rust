// ================================ //
// 1. Define the Strategy Interface //
// ================================ //

trait PaymentStrategy {
  fn pay(&self, amount: u32);
}

// ================================ //
// 2. Implement Concrete Strategies //
// ================================ //

// A. CreditCard

struct CreditCard {
  card_number: String,
}

impl PaymentStrategy for CreditCard {
  fn pay(&self, amount: u32) {
    println!("Paid {} using Credit Card ({})", amount, self.card_number);
  }
}

// B. PayPal

struct PayPal {
  email: String,
}

impl PaymentStrategy for PayPal {
  fn pay(&self, amount: u32) {
    println!("Paid {} using PayPal ({})", amount, self.email);
  }
}

// ==================================== //
// 3. Define the Context using Generics //
// ==================================== //

struct CheckoutContext<T: PaymentStrategy> {
  strategy: T, // similar to bridge pattern
}

impl<T: PaymentStrategy> CheckoutContext<T> {
  fn new(strategy: T) -> Self {
    Self { strategy }
  }

  fn complete_purchase(&self, amount: u32) {
    self.strategy.pay(amount);
  }
}

fn main() {
  {
    // Credit Card
    let card_checkout = CheckoutContext::new(CreditCard {
      card_number: String::from("1234-5678"),
    });
    card_checkout.complete_purchase(100);
  }

  {
    // PayPal
    let paypal_checkout = CheckoutContext::new(PayPal {
      email: String::from("user@example.com"),
    });
    paypal_checkout.complete_purchase(250);
  }
}
