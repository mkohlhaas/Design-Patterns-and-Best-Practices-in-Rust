// Define the data variations as an enum
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

// Write your "visitor" operations as simple functions or trait methods
fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    let shapes: Vec<Shape> = vec![
        Shape::Circle { radius: 2.0 },
        Shape::Rectangle {
            width: 3.0,
            height: 4.0,
        },
    ];

    for shape in &shapes {
        println!("{}", calculate_area(shape));
    }
}
