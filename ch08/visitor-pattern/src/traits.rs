// ========================================== //
// 1. Define the Visitor and Visitable Traits //
// ========================================== //

pub trait Visitor {
    // the Visitor trait defines what actions can happen on each data type.
    fn visit_circle(&mut self, circle: &Circle);
    fn visit_rectangle(&mut self, rectangle: &Rectangle);
}

// 2. The Shape (Visitable) trait defines the entry point for a visitor.
pub trait Shape {
    fn accept(&self, visitor: &mut dyn Visitor);
}

// ================================ //
// 2. Implement Concrete Data Types //
// ================================ //

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn accept(&self, visitor: &mut dyn Visitor) {
        // Double-dispatch: Redirects execution back to the visitor
        visitor.visit_circle(self);
    }
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Rectangle {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_rectangle(self);
    }
}

// =============================== //
// 3. Implement a Concrete Visitor //
// =============================== //

// A visitor that tracks internal state (the total calculated area)
pub struct AreaCalculator {
    pub total_area: f64,
}

impl Visitor for AreaCalculator {
    fn visit_circle(&mut self, circle: &Circle) {
        self.total_area += std::f64::consts::PI * circle.radius * circle.radius;
    }

    fn visit_rectangle(&mut self, rectangle: &Rectangle) {
        self.total_area += rectangle.width * rectangle.height;
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rectangle {
            width: 3.0,
            height: 4.0,
        }),
    ];

    let mut calculator = AreaCalculator { total_area: 0.0 };

    for shape in &shapes {
        shape.accept(&mut calculator);
    }

    println!("Total Area: {}", calculator.total_area);
}
