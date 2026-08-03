### The Command Pattern

The Command pattern, as traditionally defined, transforms operations into objects, enabling us to
store, pass, and manipulate operations just like any other data. By encapsulating operations as
first-class objects, we gain powerful capabilities, including operation history, undo/redo
functionality, command queuing, and operation logging. This pattern is fundamental to building
interactive applications where users expect to be able to reverse their actions.

Two ways to implement the Command pattern in Rust:

A. In Rust, the Command Pattern is implemented by defining a Command trait that
contains an execution method, which is then implemented by individual structs
representing specific actions. Because of Rust's strict memory ownership rules,
the idiomatic approach is to pass the Receiver (the state being modified)
directly into the execution method as a mutable reference, rather than storing
references inside the command structs:

1. Receiver (struct)
2. Command (trait) - execute(&mut self, &mut Receiver), undo(…), …
3. Concrete Command (struct) - implements Command trait
4. Command Processor (struct) - invokes Concrete Commands (execute, undo, …) on Receiver

B. Alternatively, Rust provides a highly idiomatic, lightweight variation of this
pattern using enums and matching, which completely avoids trait objects and
dynamic dispatch:

1. Commands (enum)
2. State (struct)
3. Processing loop (method of State using enum matching on the Commands)

### When to use the Command pattern

The Command pattern fits scenarios where you need undo/redo functionality, operation queuing,
or macro recording. It's appropriate when you want to parameterize objects with operations, queue
operations for later execution, or support logging of operations for debugging or audit trails.

### Key Trade-offs in Rust

| Feature | Trait Object Method (dyn Command) | Enum Matching Method |
|---|---|---|
| Dispatch Type | Dynamic dispatch (runtime performance cost) | Static dispatch (zero-cost at runtime) |
| Memory Location | Heap allocation via Box<dyn Command> | Stack allocation (highly cache-friendly) |
| Extensibility | Open. Third-party crates can implement your trait. | Closed. Adding new variants requires editing the enum. |
| State Storage | Best for storing unique, multi-variable undo histories. | Best for localized operations and lightweight message queues. |

Method 1: if you are building an interactive GUI or software requiring deep undo/redo stacks
Method 2: If you are handling command-line arguments (e.g. using
[Clap](https://crates.io/crates/clap)) or passing micro-actions across threads
