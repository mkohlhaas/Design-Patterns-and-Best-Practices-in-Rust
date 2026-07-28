### The Singleton Pattern

In Rust, the Singleton Pattern restricts the instantiation of a struct to a single instance and provides global access to it. Because Rust enforces strict compile-time thread safety and data-race prevention, implementing a classic object-oriented singleton is notoriously difficult and widely considered un-idiomatic.

Traditional approaches using static mut require unsafe blocks. Modern Rust relies on thread-safe synchronization primitives and lazy evaluation tools instead.

### The Idiomatic Way: std::sync::LazyLock

Introduced to the standard library in Rust 1.80, LazyLock (along with OnceLock)
is the safest, most modern way to build global state. It delays initialization
until the first time the singleton is accessed and ensures thread safety.

Because the instance must be globally accessible via a static item, it must
implement the Sync trait. If you need a mutable singleton, you must wrap the
inner data structure in a thread-safe lock like a Mutex or RwLock.

### Alternative Singleton Strategies

Depending on your environment and architectural needs, Rust projects utilize a few other common patterns:

- The "Just Create It Once" (Dependency Injection) Pattern:

  Instead of introducing global state, create the instance exactly once in your
  main() function. Pass references (& or &mut) or thread-safe shared pointers
  (Arc<T>) explicitly down into functions and structs that need it. This is
  highly favored in Rust as it simplifies unit testing and avoids hidden
  dependencies.

- OnceLock for Dynamic/Late Initialization:

  If you do not have the configuration details available at application startup
  to populate LazyLock, you can use std::sync::OnceLock. This allows you to
  explicitly trigger .set(value) at an arbitrary time, and fetch references
  globally via .get() later.
