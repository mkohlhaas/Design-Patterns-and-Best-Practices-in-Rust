#![allow(dead_code)]

// ========================================= //
// 1. Define distinct structs for each state //
// ========================================= //

pub struct EmptyCart;
pub struct FilledCart {
    items: Vec<String>,
}
pub struct PaidCart {
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

impl Default for Cart<EmptyCart> {
    fn default() -> Self {
        Self::new()
    }
}

impl Cart<EmptyCart> {
    pub fn new() -> Self {
        Cart { state: EmptyCart }
    }

    // Transition: Consumes EmptyCart, returns FilledCart
    pub fn add_item(self, item: String) -> Cart<FilledCart> {
        Cart {
            state: FilledCart { items: vec![item] },
        }
    }
}

// =================================================== //
// 4. Implement behavior exclusive to the Filled state //
// =================================================== //

impl Cart<FilledCart> {
    // Transition: Consumes FilledCart, returns PaidCart
    pub fn checkout(self, payment: String) -> Cart<PaidCart> {
        Cart {
            state: PaidCart {
                invoice_id: payment,
            },
        }
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    let cart = Cart::new(); // Starts empty
    let filled_cart = cart.add_item("Rust Book".to_string());
    let _paid_cart = filled_cart.checkout("TXN_12345".to_string());

    // COMPILER ERROR: cart.add_item(...) or filled_cart.checkout(...)
    // cannot be called again because ownership was consumed!
}
