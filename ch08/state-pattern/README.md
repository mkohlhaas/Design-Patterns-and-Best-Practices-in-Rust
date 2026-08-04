### The State Design Pattern

In Rust, the State design pattern is a behavioral pattern that allows an object
to change its behavior when its internal state changes.

Because Rust does not support traditional object-oriented inheritance,
developers generally choose between three distinct approaches to manage states:
the Classical OOP-style (Trait-based) approach, the Enum-based state machine
approach, and the idiomatic Typestate pattern approach.

### Comparison of State Approaches in Rust

| Approach | Where Transitions Happen | Safety Level | Idiomatic? |
|---|---|---|---|
| Classical Trait | Runtime (Box<dyn State>) | Runtime checks required | No (un-idiomatic Rust) |
| Enum Machine | Runtime (match statements) | Safe, but handles error states | Yes, for dynamic states |
| Typestate | Compile-time (Type changes) | Maximum (Impossible to misbehave) | Highly Idiomatic |
