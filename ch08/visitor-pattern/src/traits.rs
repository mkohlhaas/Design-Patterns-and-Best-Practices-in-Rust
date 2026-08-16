// =============== //
// Visiting Shapes //
// =============== //

// Shape is the Visitable.

// Visitor   trait has `visit_<functions>`.
// Visitable trait has `accept function`.
// Visitable accepts a Visitor and calls one of the visit_<functions>, thereby redirecting execution back to the visitor (double-dispatch)

//      1st dispatch      ->      2nd dispatch
// circle.accept(visitor) ->  visitor.visit_circle(circle)
// circle.......(visitor) ->  visitor.............(circle)
// circle  <->   visitor  <=> visitor <-> circle

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

    let mut total_area_calculator = TotalAreaCalculator::new();

    for shape in &shapes {
        shape.accept(&mut total_area_calculator);
    }

    println!("Total Area: {}", total_area_calculator.total_area);
}
