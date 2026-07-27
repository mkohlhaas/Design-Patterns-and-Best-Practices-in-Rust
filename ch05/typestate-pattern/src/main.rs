#![allow(unused)]

// Code Example: A Shopping Cart Flow
//
// This pattern is highly effective for wizard interfaces, network protocols, or multi-step
// builders. Below is an example of an e-commerce checkout flow where a cart must be filled before
// being paid for and shipped.

use std::marker::PhantomData;

// 1. Define the unique state markers (unit structs: Empty, Filled, Paid)
pub struct Empty;

pub struct Filled {
  items: Vec<String>,
}

pub struct Paid {
  items: Vec<String>,
  invoice_id: u64,
}

// 2. Define the main struct, parameterized by its state
pub struct ShoppingCart<State> {
  state: State,
}

// 3. Methods available ONLY in the 'Empty' state
impl ShoppingCart<Empty> {
  pub fn new() -> Self {
    ShoppingCart { state: Empty }
  }

  pub fn add_items(self, items: Vec<String>) -> ShoppingCart<Filled> {
    // Consumes Empty cart, returns a Filled cart
    ShoppingCart {
      state: Filled { items },
    }
  }
}

impl Default for ShoppingCart<Empty> {
  fn default() -> Self {
    Self::new()
  }
}

// 4. Methods available ONLY in the 'Filled' state
impl ShoppingCart<Filled> {
  pub fn checkout(self, invoice_id: u64) -> ShoppingCart<Paid> {
    // Consumes Filled cart, returns a Paid cart
    println!("Processing payment for items: {:?}", self.state.items);
    ShoppingCart {
      state: Paid {
        items: self.state.items,
        invoice_id,
      },
    }
  }
}

// 5. Methods available ONLY in the 'Paid' state
impl ShoppingCart<Paid> {
  pub fn ship(self) {
    // Final state transition; consumes the cart entirely
    println!("Shipping order #{}!", self.state.invoice_id);
  }
}

// Alternative implementation with PhantomData

// State markers (unit structs)
#[derive(Debug)]
pub struct Disconnected;

#[derive(Debug)]
pub struct Connected;

// The struct carries a zero-sized phantom marker
#[derive(Debug)]
pub struct Connection<State> {
  id: u32,
  _marker: PhantomData<State>,
}

impl Connection<Disconnected> {
  pub fn new(id: u32) -> Self {
    Self {
      id,
      _marker: PhantomData,
    }
  }

  pub fn connect(self) -> Connection<Connected> {
    Connection {
      id: self.id,
      _marker: PhantomData,
    }
  }
}

fn main() {
  {
    // Valid Usage
    let cart = ShoppingCart::new();
    let filled_cart = cart.add_items(vec!["Rust Book".to_string()]);
    let paid_cart = filled_cart.checkout(42);
    paid_cart.ship();
  }

  {
    // INVALID USAGE (Will not compile)
    let cart = ShoppingCart::new();
    // cart.checkout(42); // Error: no method named `checkout` found for struct `ShoppingCart<Empty>` in the current scope
  }
  {
    let connection = Connection::new(42);
    let connected = connection.connect();
    println!("{:?}", connected);
  }
}
