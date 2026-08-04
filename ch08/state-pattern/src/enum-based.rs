#![allow(dead_code)]

// ================================================= //
// 1. Define the possible states as an Enum variants //
// ================================================= //

#[derive(Debug)]
enum CartState {
    Empty,
    Filled { items: Vec<String> },
    Paid { invoice_id: String },
}

// ======================================================= //
// 2. A single Cart struct that mutates its internal state //
// ======================================================= //

pub struct Cart {
    state: CartState,
}

impl Default for Cart {
    fn default() -> Self {
        Self::new()
    }
}

impl Cart {
    // Initialize the cart in the Empty state
    pub fn new() -> Self {
        Cart {
            state: CartState::Empty,
        }
    }

    // Transition 1: Empty -> Filled
    // Takes &mut self instead of consuming ownership
    pub fn add_item(&mut self, item: String) -> Result<(), &'static str> {
        // We use std::mem::replace to safely extract the value out of &mut self
        let current_state = std::mem::replace(&mut self.state, CartState::Empty);

        match current_state {
            CartState::Empty => {
                self.state = CartState::Filled { items: vec![item] };
                Ok(())
            }
            CartState::Filled { mut items } => {
                items.push(item);
                self.state = CartState::Filled { items };
                Ok(())
            }
            CartState::Paid { .. } => {
                // Restore state if transition is invalid
                self.state = current_state;
                Err("Cannot add items to a paid cart!")
            }
        }
    }

    // Transition 2: Filled -> Paid
    pub fn checkout(&mut self, payment: String) -> Result<(), &'static str> {
        let current_state = std::mem::replace(&mut self.state, CartState::Empty);

        match current_state {
            CartState::Filled { .. } => {
                self.state = CartState::Paid {
                    invoice_id: payment,
                };
                Ok(())
            }
            _ => {
                // Restore state if transition is invalid
                self.state = current_state;
                Err("Cart must be filled before checking out!")
            }
        }
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    let mut cart = Cart::new(); // Starts empty

    // Successfully add an item
    cart.add_item("Rust Book".to_string()).unwrap();

    // Successfully checkout
    cart.checkout("TXN_12345".to_string()).unwrap();

    // RUNTIME ERROR: This will print an error message instead of failing at compile time
    if let Err(e) = cart.add_item("Another Book".to_string()) {
        println!("Error handled at runtime: {}", e);
    }
}
