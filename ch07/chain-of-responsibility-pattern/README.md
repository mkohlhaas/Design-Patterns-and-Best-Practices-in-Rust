### The Chain of Responsibility Pattern

In Rust, the Chain of Responsibility pattern is a behavioral design pattern
implemented using traits for interfaces, Option<Box<dyn Handler>> pointers for
recursive runtime linking, and enums for flexible requests:

1. Request (enum) - what has to be handled
2. Handler (trait) - `handle(…)` which matches on Request enums and `set_next_handler(…)` methods
3. Concrete Handlers (struct - implementing Handlers with a next_handler field)
