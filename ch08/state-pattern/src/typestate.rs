#![allow(dead_code)]

// ========================================= //
// 1. Define distinct structs for each state //
// ========================================= //

pub struct Empty;
pub struct Filled {
    items: Vec<String>,
}
#[derive(Debug)]
pub struct Paid {
    invoice_id: String,
}

// ======================= //
// 2. Generic Cart wrapper //
// ======================= //

pub struct Cart<State> {
    state: State,
}

// ================================================== //
// 3. Implement behavior exclusive to the Empty state //
// ================================================== //

impl Default for Cart<Empty> {
    fn default() -> Self {
        Self::new()
    }
}

impl Cart<Empty> {
    pub fn new() -> Self {
        Cart { state: Empty }
    }

    // Transition: Consumes EmptyCart, returns FilledCart
    pub fn add_item(self, item: String) -> Cart<Filled> {
        Cart {
            state: Filled { items: vec![item] },
        }
    }
}

// =================================================== //
// 4. Implement behavior exclusive to the Filled state //
// =================================================== //

impl Cart<Filled> {
    // Transition: Consumes FilledCart, returns PaidCart
    pub fn checkout(self, payment: String) -> Cart<Paid> {
        Cart {
            state: Paid {
                invoice_id: payment,
            },
        }
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    let cart = Cart::new();
    let cart = cart.add_item("Rust Book".into());
    let cart = cart.checkout("TXN_12345".to_string());

    println!("{:?}", cart.state)

    // COMPILER ERROR: cart.add_item(...) or filled_cart.checkout(...)
    // cannot be called again because ownership was consumed!
}
