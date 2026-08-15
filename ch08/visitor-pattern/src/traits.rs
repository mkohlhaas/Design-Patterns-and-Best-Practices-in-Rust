// =============== //
// Visiting Shapes //
// =============== //

// Visitor pattern is based on double dispatch:
// 1st dispatch: call `accept` of the Visitable (Shape), e.g. `circle.accept(visitor))``
// 2nd dispatch: `accept` calls visitor.visit_fn, e.g. visitor.visit_circle(<self=circle>)
// So it goes from the Visitable back to the Visitor!
//
// circle calls function (`accept`) with the visitor
// in `accept` visitor calls a function with circle as parameter

// The Visitable trait accepts a Visitor trait and calls a function in the visitor.
// Visitable: `accept`
// Visitor: visit_<the different shapes>

// ========================================== //
// 1. Define the Visitor and Visitable Traits //
// ========================================== //

// What shapes can we visit? circles, rectangles

pub trait Visitor {
    // the Visitor trait defines what actions can happen on each data type.
    fn visit_circle(&mut self, circle: &Circle);
    fn visit_rectangle(&mut self, rectangle: &Rectangle);
}

// ==================================================================== //
// 2. The Shape (Visitable) trait defines the entry point for a visitor //
// ==================================================================== //

pub trait Shape {
    fn accept(&self, visitor: &mut dyn Visitor);
}

// ================================ //
// 3. Implement Concrete Data Types //
// ================================ //

// ========= //
// A. Circle //
// ========= //

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn accept(&self, visitor: &mut dyn Visitor) {
        // Double-dispatch: Redirects execution back to the visitor
        visitor.visit_circle(self);
    }
}

// ============ //
// B. Rectangle //
// ============ //

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
// 4. Implement a Concrete Visitor //
// =============================== //

// A visitor that tracks internal state (the total calculated area)
pub struct TotalAreaCalculator {
    pub total_area: f64,
}

impl TotalAreaCalculator {
    pub fn new() -> Self {
        Self { total_area: 0.0 }
    }
}

impl Default for TotalAreaCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for TotalAreaCalculator {
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
    // because Shape is a trait we can collect them in a vec
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rectangle {
            width: 3.0,
            height: 4.0,
        }),
    ];

    let mut calculator = TotalAreaCalculator::new();

    for shape in &shapes {
        shape.accept(&mut calculator);
    }

    println!("Total Area: {}", calculator.total_area);
}
