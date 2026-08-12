use std::rc::{Rc, Weak};

// ========================================================== //
// 1. Define a trait and a concrete struct that implements it //
// ========================================================== //

trait Speaker {
    fn speak(&self);
}

struct Dog {
    name: String,
}

impl Speaker for Dog {
    fn speak(&self) {
        println!("{} says: Woof!", self.name);
    }
}

fn main() {
    // Create a strong pointer to the concrete type
    let dog_strong: Rc<Dog> = Rc::new(Dog {
        name: String::from("Barkley"),
    });

    // Doesn't work!
    // let trait_strong = dog_strong;

    // --- THE FIX ---
    // We explicitly coerce the Rc<Dog> to Rc<dyn Speaker> first.
    // Rust handles this automatically during assignment.
    let trait_strong: Rc<dyn Speaker> = dog_strong;

    // Now we can successfully downgrade it to a Weak trait object pointer.
    let weak_speaker = Rc::downgrade(&trait_strong);

    // --- Using the Weak pointer later ---
    // To use it, we upgrade it back to a strong pointer.
    if let Some(speaker) = weak_speaker.upgrade() {
        // LSP can't inference the concrete type of `speaker`
        speaker.speak(); // "Barkley says: Woof!"
    } else {
        println!("The speaker has already been dropped.");
    }
}
