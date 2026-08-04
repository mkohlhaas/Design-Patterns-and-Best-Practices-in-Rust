// This is the most idiomatic Rust approach.
// It relies on Generics and Trait Bounds.
// Because the wrapping happens at compile time, the compiler can inline the code, eliminating
// runtime performance overhead and heap allocations.

// 1. The Trait
trait TextProcessor {
    fn process(&self, text: &str) -> String;
}

// 2. The Base Component
struct SimpleProcessor;

impl TextProcessor for SimpleProcessor {
    fn process(&self, text: &str) -> String {
        text.to_string()
    }
}

// 3. The Static Decorator (Uses Generic T)
struct UppercaseDecorator<T: TextProcessor> {
    wrapped: T,
}

impl<T: TextProcessor> TextProcessor for UppercaseDecorator<T> {
    fn process(&self, text: &str) -> String {
        // Enhance base behavior
        self.wrapped.process(text).to_uppercase()
    }
}

fn main() {
    let base = SimpleProcessor;

    // Stacked at compile time. No allocation.
    let decorated = UppercaseDecorator { wrapped: base };

    println!("{}", decorated.process("hello rust!")); // Outputs: HELLO RUST!
}
