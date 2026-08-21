### Encoding invariants

Encoding invariants in types means designing your Rust data structures so that
the compiler’s type checker automatically prevents invalid data states or
illegal operations.

An invariant is a rule or property about your data that must always remain true
(e.g., "a username cannot be empty", or "a connection must be open before
sending data"). Instead of relying on documentation or manually checking these
rules at runtime using if statements, you force the Rust type system to enforce
them at compile time. If a programmer tries to break the rule, the code simply
will not compile.

This approach is heavily inspired by Alexis King's famous mantra: “Parse, don’t
validate.”

### How Invariants are Encoded in Rust

Rust provides several language features—like strict privacy, strong enum
matching, and ownership mechanics—that make encoding invariants incredibly
effective.

### 1. Making Invalid States Unrepresentable (Using Enums)

Instead of using loose data types and checking their combinations, use
algebraic data types (enum) to ensure only valid combinations can ever exist.

### 2. The Smart Constructor Pattern (Using Privacy)

By making struct fields private, you prevent external code from instantiating
or changing fields arbitrarily. You then provide a "smart constructor" (usually
new or try_from) that validates data once. Once the type is successfully
created, it is mathematically guaranteed to be valid forever.

### 3. The Typestate Pattern (Using Ownership)

You can encode the sequential state transitions of an object into entirely
different types. Because Rust enforces move semantics (ownership), consuming an
old state prevents a developer from accidentally reusing an outdated or illegal
object state.


### Summary of Benefits

* Zero-Cost Abstractions: Most type-level enforcement happens strictly at compile time, meaning no performance penalties at runtime.
* No Redundant Logic: You don't need to write if self.is_empty() { ... } in 50 different functions; the type itself guarantees it isn't empty.
* Eliminates Bug Classes: It wipes out entire categories of runtime bugs, such as Null Pointer Exceptions or out-of-order execution crashes.
