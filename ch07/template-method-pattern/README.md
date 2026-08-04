### The Template Pattern

In Rust, the Template Method design pattern is a behavioral pattern used to
define the skeleton of an algorithm in a central place while allowing specific
steps to be customized. Because Rust does not have classical inheritance or
classes, this pattern cannot be implemented using abstract classes and
overrides. Instead, idiomatic Rust achieves this using traits with default
method implementations or closures.

The template pattern helps define reusable algorithms with customizable steps.
