// Code Example: The Shape Factory Pattern
//
// Instead of using heap allocation or dynamic dispatch (Box<dyn Trait>) which incurs a performance
// cost, you can utilize an enum factory to handle object creation statically on the stack.

// Different structural payloads for our factory
#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Debug)]
pub struct Square {
    pub side: f64,
}

// The unified Enum type wrapping the shapes
#[derive(Debug)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

impl Shape {
    // The Factory Method: Creates a specific Shape variant from runtime data
    pub fn new(shape_type: &str, size: f64) -> Result<Self, String> {
        match shape_type {
            "circle" => Ok(Shape::Circle(Circle { radius: size })),
            "square" => Ok(Shape::Square(Square { side: size })),
            _ => Err(format!("Unknown shape type: {}", shape_type)),
        }
    }

    // factory method
    pub fn circle(radius: f64) -> Self {
        Shape::Circle(Circle { radius })
    }

    // factory method
    pub fn square(side: f64) -> Self {
        Shape::Square(Square { side })
    }
}

fn main() {
    {
        // Using the factory method to safely generate our enum objects
        if let Ok(my_shape) = Shape::new("circle", 5.0) {
            // Rust's match guarantees exhaustive handling later
            match my_shape {
                Shape::Circle(ref c) => println!("Circle radius: {}", c.radius),
                Shape::Square(ref s) => println!("Square side: {}", s.side),
            }
            println!("{my_shape:?}")
        }
    }

    {
        let circle = Shape::circle(5.0);
        println!("{circle:?}")
    }

    {
        let square = Shape::square(5.0);
        println!("{square:?}")
    }
}
