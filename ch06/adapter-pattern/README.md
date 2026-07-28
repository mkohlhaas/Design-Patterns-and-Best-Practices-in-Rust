### The Adapter Design Pattern

The Adapter design pattern in Rust is a structural pattern that allows two
incompatible interfaces to collaborate by wrapping an existing type inside a
new structure. Instead of relying on object inheritance—which Rust does not
natively support—this pattern uses struct composition and traits to translate
calls from a client-facing interface into a format that a legacy or third-party
type understands.

### Key Concepts in Rust

Unlike traditional object-oriented languages that use Class Adapters via
multiple inheritance, Rust exclusively relies on Object Adapters through
composition. The pattern involves three core components:

* *The Target Trait*: The standard interface your current client code expects.
* *The Adaptee*: The incompatible, legacy, or third-party struct containing the necessary functionality.
* *The Adapter*: A wrapper struct that holds an instance of the Adaptee and implements the Target Trait.

### Overcoming the Orphan Rule

In Rust, the adapter pattern often surfaces as the Newtype pattern. Rust's
Orphan Rule dictates that you cannot implement an external trait for an
external struct. Wrapping that third-party struct inside a locally defined
adapter tuple struct (struct MyAdapter(ExternalType);) allows you to legally
bypass this restriction and implement any local or remote traits you need.

### Idiomatic Ecosystem Variations

* *Iterator Adaptors*: The most famous use of this pattern in Rust is built straight into the standard library. Types like Map, Filter, and Zip take an existing iterator and adapt its output behavior into a new iterator type without allocating extra runtime overhead.
* *From and Into Traits*: For simple data-shape adaptations, implementing standard conversion traits like From<T> is considered the idiomatic way to safely change data from one interface into another.
* *Extension Traits*: Developers often combine adapters with blanket implementations to extend existing types uniformly across an application.
