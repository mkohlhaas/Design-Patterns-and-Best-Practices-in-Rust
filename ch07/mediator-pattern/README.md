### The Mediator Pattern

The Mediator design pattern in Rust is considered an anti-pattern when
implemented using the classic object-oriented approach. Having multiple
components hold mutable cross-references to each other directly violates Rust's
strict [aliasing and borrow checker rules](https://github.com/fadeevab/mediator-pattern-rust).

To implement it idiomatically in Rust, you must drop shared mutability pointers
like Rc<RefCell<T>> and instead adopt a top-down ownership approach or a
messaging registry.

------------------------------
## The Two Ways to Build a Mediator in Rust

| Approach | How it Works | Rust Safety Mechanism | Pros & Cons |
|---|---|---|---|
| Top-Down Ownership (Idiomatic) | The Mediator owns all components. Components do not point to the Mediator; events are processed via central loop methods. | Standard compile-time ownership (mut self) | 🟢 Fast and completely safe 🔴 Changes standard OOP control flow structure |
| Shared Mutability (Classic OOP) | Components hold shared pointers to the Mediator, and the Mediator holds pointers to components. | Run-time checking (Rc<RefCell<T>>) | 🟢 Familiar OOP setup 🔴 Risk of runtime crashes (panic!) and memory leaks |

