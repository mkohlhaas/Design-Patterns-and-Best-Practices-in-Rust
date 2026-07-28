#![allow(unused)]

// The macro automatically generates the boilerplate logic for cloning
#[derive(Clone, Debug)]
struct UIComponent {
  width: u32,
  height: u32,
  color: String,
}

fn main() {
  // 1. Create a "prototype" instance
  let blueprint_button = UIComponent {
    width: 100,
    height: 50,
    color: String::from("Blue"),
  };

  // 2. Clone the prototype to make an exact copy
  let mut submit_button = blueprint_button.clone();

  // 3. Tweak the copy as needed
  submit_button.color = String::from("Green");

  println!("Original: {:?}", blueprint_button);
  println!("Cloned & modified: {:?}", submit_button);
}
