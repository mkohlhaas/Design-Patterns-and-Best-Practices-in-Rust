// The Idiomatic Rust Alternative: Enums
//
// While double dispatch works, it requires modifying the trait whenever you add a new shape. In
// Rust, if your types are bound and known at compile time, using an enum with pattern matching is
// significantly more idiomatic and performant (it avoids vtable pointers entirely).

enum Shape {
    Asteroid,
    Spaceship,
}

fn collide(a: &Shape, b: &Shape) {
    match (a, b) {
        (Shape::Asteroid, Shape::Asteroid) => println!("Asteroid hits Asteroid!"),
        (Shape::Asteroid, Shape::Spaceship) => println!("Asteroid hits Spaceship!"),
        (Shape::Spaceship, Shape::Asteroid) => println!("Spaceship hits Asteroid!"),
        (Shape::Spaceship, Shape::Spaceship) => println!("Spaceship hits Spaceship!"),
    }
}

fn main() {
    let shape1 = Shape::Asteroid;
    let shape2 = Shape::Spaceship;

    collide(&shape1, &shape2);
    collide(&shape2, &shape1);
}
