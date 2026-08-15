// ======================== //
// 1. Define the core trait //
// ======================== //

trait Shape {
    // The first dispatch point
    fn collide(&self, other: &dyn Shape);

    // The second dispatch points (the callbacks)
    fn collide_with_asteroid(&self, asteroid: &Asteroid);
    fn collide_with_spaceship(&self, spaceship: &Spaceship);
}

// ========================================== //
// A. Implement the trait for Concrete Type A //
// ========================================== //

struct Asteroid;
impl Shape for Asteroid {
    fn collide(&self, other: &dyn Shape) {
        // First handoff: 'other' now knows 'self' is an Asteroid
        other.collide_with_asteroid(self);
    }

    fn collide_with_asteroid(&self, _asteroid: &Asteroid) {
        println!("Asteroid hit an Asteroid! Mutual destruction.");
    }

    fn collide_with_spaceship(&self, _spaceship: &Spaceship) {
        println!("Asteroid hit a Spaceship! Spaceship damaged.");
    }
}

// ========================================== //
// B. Implement the trait for Concrete Type B //
// ========================================== //

struct Spaceship;
impl Shape for Spaceship {
    fn collide(&self, other: &dyn Shape) {
        // First handoff: 'other' now knows 'self' is a Spaceship
        other.collide_with_spaceship(self);
    }

    fn collide_with_asteroid(&self, _asteroid: &Asteroid) {
        println!("Spaceship hit an Asteroid! Shield absorbed impact.");
    }

    fn collide_with_spaceship(&self, _spaceship: &Spaceship) {
        println!("Spaceship passed by another Spaceship. Friendly wave!");
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    // Create trait objects (types are erased to dyn Shape)
    let shape1: &dyn Shape = &Asteroid;
    let shape2: &dyn Shape = &Spaceship;

    // Triggers double dispatch cleanly at runtime
    shape1.collide(shape2); // Output: Spaceship hit an Asteroid! Shield absorbed impact.
    shape2.collide(shape1); // Output: Asteroid hit a Spaceship! Spaceship damaged.
}
