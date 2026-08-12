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

The Observer pattern establishes a one-to-many dependency between objects so
that when one object changes state, all its dependents are notified and updated
automatically. This pattern is fundamental to implementing event handling
systems, reactive programming models, and components that need to stay
synchronized.

The Observer pattern decouples the subject (the object being observed) from its
observers. The subject maintains a list of observers and notifies them of
changes, but doesn't know what they do with that information. Observers
register themselves with subjects they care about and react when notified.

### When to use the Observer pattern

The Observer pattern fits scenarios where one object's state changes should
trigger reactions in multiple other objects, especially when you don't want the
subject to know specifics about its observers. The pattern excels at
implementing event systems, maintaining consistency between related objects
(like model and view in the model–view–controller architecture), and building
reactive data pipelines.

Consider the Observer pattern when changes to one object require changing
others but you don't know how many or exactly which objects, when an object
should notify others without making assumptions about who they are, or when you
want to add or remove reactivity at runtime. The tradeoffs include potential
for unexpected cascading updates and difficulty debugging notification chains.
Using Rust's built-in features with an event-based approach mitigates some of
these concerns: events carry explicit data (making it clear what changed),
pattern matching makes observer logic visible, and the type system ensures
observers handle the event types they care about.

For concurrent applications, channels (such as std::sync:: mpsc ) offer an
alternative to the Mutex -based approach we used in DisplayObserver . Channels
naturally decouple the sender (subject) from the receiver (observer) and avoid
the need for shared mutable state entirely. In async contexts, the Observer
pattern maps naturally onto runtimes like Tokio using broadcast channels or the
Notify primitive. The choice between Mutex -based observers, channel-based
observers, and async alternatives depends on your application's concurrency
model.

### 🛠️ Key Challenges in Rust

* Ownership Cycles: A naive implementation creates a reference loop, which Rust prevents by default to ensure memory safety.
* Shared Mutability: The Subject needs to iterate through its list of Observers to trigger updates, but those Observers may need to mutate their own internal states during the notification process.

### ⚡ Rust Alternatives to the Observer Pattern
Because the object-oriented approach requires verbose wrappers like Rc<RefCell<T>>, Rust developers frequently leverage alternative, more idiomatic paradigms:

* Function Closures: Instead of implementing a heavy trait hierarchy, the Subject simply accepts a vector of generic callback functions (Box<dyn Fn(&State)>).
* Channels (CSP): For modern asynchronous or multi-threaded systems, developers ditch observers altogether in favor of std::sync::mpsc channels or broadcast channels (like those in the tokio crate) to pass event messages safely across threads.
