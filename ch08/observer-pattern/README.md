### The Observer Design Pattern

The Observer design pattern is a behavioral design pattern that establishes a
one-to-many dependency between objects. When one object (the Subject or
Observable) changes its state, all its dependent objects (the Observers) are
automatically notified and updated.

Implementing the classical object-oriented Observer pattern in Rust is
notoriously tricky due to Rust's strict ownership and lifetimes model. Standard
OO implementations rely on cyclic references (the Subject holds a list of
Observers, and Observers often hold a reference back to the Subject), which
violates Rust's rule of having only one mutable reference at a time.

### 🛠️ Key Challenges in Rust

* Ownership Cycles: A naive implementation creates a reference loop, which Rust prevents by default to ensure memory safety.
* Shared Mutability: The Subject needs to iterate through its list of Observers to trigger updates, but those Observers may need to mutate their own internal states during the notification process.

### ⚡ Rust Alternatives to the Observer Pattern
Because the object-oriented approach requires verbose wrappers like Rc<RefCell<T>>, Rust developers frequently leverage alternative, more idiomatic paradigms:

* Function Closures: Instead of implementing a heavy trait hierarchy, the Subject simply accepts a vector of generic callback functions (Box<dyn Fn(&State)>).
* Channels (CSP): For modern asynchronous or multi-threaded systems, developers ditch observers altogether in favor of std::sync::mpsc channels or broadcast channels (like those in the tokio crate) to pass event messages safely across threads.
