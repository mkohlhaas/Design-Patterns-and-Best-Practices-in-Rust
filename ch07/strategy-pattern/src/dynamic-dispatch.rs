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

// ========================================= //
// 3. Define the Context using Trait Objects //
// ========================================= //

struct DynamicCheckout {
  // Box allows holding any type implementing the trait
  strategy: Box<dyn PaymentStrategy>, // similar to the bridge pattern
}

impl DynamicCheckout {
  fn new(strategy: Box<dyn PaymentStrategy>) -> Self {
    Self { strategy }
  }

  fn complete_purchase(&self, amount: u32) {
    self.strategy.pay(amount);
  }

  // Allows swapping the strategy at runtime
  fn change_strategy(&mut self, new_strategy: Box<dyn PaymentStrategy>) {
    self.strategy = new_strategy;
  }
}

fn main() {
  // Initialize with Credit Card
  let mut checkout = DynamicCheckout::new(Box::new(CreditCard {
    card_number: String::from("4321-8765"),
  }));
  checkout.complete_purchase(50);

  // Swap to PayPal at runtime
  checkout.change_strategy(Box::new(PayPal {
    email: String::from("switch@example.com"),
  }));
  checkout.complete_purchase(75);
}
