### The Command Pattern

A. In Rust, the Command Pattern is implemented by defining a Command trait that
contains an execution method, which is then implemented by individual structs
representing specific actions. Because of Rust's strict memory ownership rules,
the idiomatic approach is to pass the Receiver (the state being modified)
directly into the execution method as a mutable reference, rather than storing
references inside the command structs:

1. Receiver (struct)
2. Command (trait)
3. Concrete Command (implements Command trait)
4. Invoker (struct) - invokes Concrete Commands on Receiver

B. Alternatively, Rust provides a highly idiomatic, lightweight variation of this
pattern using enums and matching, which completely avoids trait objects and
dynamic dispatch:

1. Commands (enum)
2. State (struct)
3. Processing loop (method of State using enum matching on the Commands)

### Key Trade-offs in Rust

| Feature | Trait Object Method (dyn Command) | Enum Matching Method |
|---|---|---|
| Dispatch Type | Dynamic dispatch (runtime performance cost) | Static dispatch (zero-cost at runtime) |
| Memory Location | Heap allocation via Box<dyn Command> | Stack allocation (highly cache-friendly) |
| Extensibility | Open. Third-party crates can implement your trait. | Closed. Adding new variants requires editing the enum. |
| State Storage | Best for storing unique, multi-variable undo histories. | Best for localized operations and lightweight message queues. |

If you are building an interactive GUI or software requiring deep undo/redo stacks, rely on Method 1.

If you are handling command-line arguments (e.g., using [Clap](https://crates.io/crates/clap)) or passing micro-actions across threads, rely on Method 2.
