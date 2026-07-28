#![allow(unused)]

// Idiomatic Rust Implementation
//
// In Rust, the Flyweight pattern heavily leverages smart pointers like Rc<T> (for single-threaded
// contexts) or Arc<T> (for multi-threaded contexts) to safely manage shared reference counts
// without manual memory pooling.
//
// Here is an example representing a forest with millions of trees using Rc<T>.

use std::collections::HashMap;
use std::rc::Rc;

// 1. Intrinsic State (The Flyweight)
// This structure holds heavy, shared data.
#[derive(Debug, PartialEq, Eq, Hash)]
struct TreeType {
  name: String,
  color: String,
  texture_data: Vec<u8>, // Large byte data
}

// 2. Extrinsic State
// Contains unique coordinates and a shared pointer to the Flyweight data.
struct Tree {
  x: u32,
  y: u32,
  tree_type: Rc<TreeType>, // Shared reference
}

// 3. Flyweight Factory
// Manages the pool/cache of shared TreeType instances.
struct TreeFactory {
  types: HashMap<(String, String), Rc<TreeType>>,
}

impl TreeFactory {
  fn new() -> Self {
    TreeFactory {
      types: HashMap::new(),
    }
  }

  fn get_tree_type(&mut self, name: &str, color: &str) -> Rc<TreeType> {
    let key = (name.to_string(), color.to_string());

    // Return cached reference or insert a new one if it doesn't exist
    self
      .types
      .entry(key)
      .or_insert_with(|| {
        Rc::new(TreeType {
          name: name.to_string(),
          color: color.to_string(),
          texture_data: vec![0; 1024 * 1024], // Simulating a 1MB texture
        })
      })
      .clone()
  }
}

fn main() {
  let mut factory = TreeFactory::new();
  let mut forest = Vec::new();

  // Create 10,000 trees but only allocate memory for 2 TreeTypes
  for i in 0..5000 {
    let oak_type = factory.get_tree_type("Oak", "Green");
    forest.push(Tree {
      x: i,
      y: i * 2,
      tree_type: oak_type,
    });

    let pine_type = factory.get_tree_type("Pine", "Dark Green");
    forest.push(Tree {
      x: i + 1,
      y: i * 3,
      tree_type: pine_type,
    });
  }

  println!("Forest size: {} trees.", forest.len());
  println!(
    "Allocated tree types in factory cache: {}",
    factory.types.len()
  );
}
