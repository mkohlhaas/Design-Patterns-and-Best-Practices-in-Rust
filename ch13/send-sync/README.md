### Send and Sync

In Rust, Send and Sync are built-in traits that form the foundation of the
language's thread-safety guarantees. They are marker traits, meaning they do
not have any methods to implement; they simply exist to label types and tell
the compiler how data can be safely used across threads.

### 1. Direct Definitions

* Send: Indicates that ownership of a type can be transferred to another thread.
* Sync: Indicates that it is safe for multiple threads to access a type through shared immutable references (&T).

The relationship between the two can be summed up by a golden rule in Rust:

**T is Sync if and only if &T is Send.**

This means that if a thread can safely read a piece of data via a reference,
that reference itself can be safely passed to another thread.

### 2. How They Work in Daily Coding

You rarely need to implement these traits manually. Rust automatically implements Send and Sync for your custom struct or enum if all of its internal fields are also Send and Sync.

### Types that are NOT Send

A type is not Send if transferring it to another thread would cause memory
corruption or undefined behavior.

* Rc<T> (Reference Counted pointer): This is a non-thread-safe smart pointer. When you clone an Rc, it increments an internal counter using fast, non-atomic operations. If you pass an Rc to another thread and both threads increment the counter at the same time, it causes a data race, leading to memory leaks or double frees. Therefore, Rc is !Send.

### Types that are Send but NOT Sync

A type can be safely moved to a new thread, but cannot be shared between multiple threads simultaneously without protection.

* RefCell<T> and Cell<T>: These allow "interior mutability" (modifying data through an immutable reference) without thread synchronization. If two threads tried to mutate a shared RefCell at the same exact time via &RefCell, they would corrupt the data. It is safe to give the whole RefCell away to another thread (Send), but unsafe to share references to it (!Sync).

### Types that are BOTH Send and Sync

These are primitive types and specialized primitives built for multi-threading.

* Primitives: i32, f64, bool, String, and standard collections are both Send and Sync because their data cannot be mutated through immutable references.
* Arc<T> (Atomic Reference Counted): The thread-safe sibling of Rc. It uses atomic CPU instructions to update the counter, making it safe to clone across threads. Arc<T> is Send and Sync as long as T is also Send and Sync.
* Mutex<T> and RwLock<T>: These wrap data and enforce strict locking mechanisms. They turn a type that is Send but !Sync into something that is safely Sync, because the lock guarantees only one thread can access the underlying data at a time.

### 3. Summary Cheat Sheet

| Type | Send? | Sync? | Thread-Safety Behavior |
|---|---|---|---|
| i32, String | Yes | Yes | Safe to move and safely immutable across threads. |
| Rc<T> | ⚠️ No | ⚠️ No | Strictly single-threaded. |
| RefCell<T> | Yes | ⚠️ No | Can be moved to a thread, but not shared concurrently. |
| Arc<T> | Yes | Yes | Safe to clone and share reference counts across threads. |
| Mutex<T> | Yes | Yes | Safe to share across threads; internal lock guarantees safe mutation. |

### 4. Why This Matters: Compile-Time Concurrency

In other languages, sharing a non-thread-safe pointer across threads results in
a runtime crash, intermittent bugs, or security vulnerabilities.

In Rust, functions that spawn threads (like std::thread::spawn) explicitly
require the closures and data passed into them to implement Send. If you
attempt to pass an unsafe type like an Rc, the code will refuse to compile.

### The Example Code

The example code is a complete, runnable example demonstrating why Send and
Sync matter.

This example simulates a multi-threaded web server parsing a configuration
file. It contrasts unsafe single-threaded types (Rc) with safe thread-safe
types (Arc and Mutex), showing exactly how the compiler protects you.

### The Scenario

We have a configuration string that multiple background threads need to read
simultaneously. One thread also needs to safely update the server's status
metric.

### What happens if we try to break the rules?

To see Send and Sync protection in action, look at what happens if we replace
Arc with Rc (the non-thread-safe reference counter).

If you change the setup to this:

```rust
use std::rc::Rc;

// This is NOT thread-safe
let unsafe_config = Rc::new(String::from("db_host=localhost;"));
let config_clone = Rc::clone(&unsafe_config);

thread::spawn(move || {
    println!("{}", config_clone);
});
```

The Rust compiler will immediately halt and print an error similar to this:

```
error[E0277]: `Rc<String>` cannot be sent between threads safely
   --> src/main.rs:10:19
    |
10  |       thread::spawn(move || {
    |  _____------------_^

    | |     |
    | |     required by a bound introduced by this call
11  | |         println!("{}", config_clone);
12  | |     });

    | |_____^ `Rc<String>` cannot be sent between threads safely
    |
    = help: within `[closure]`, the trait `Send` is not implemented for `Rc<String>`

```

### Key Takeaways from the Examples

* No Mutex needed for pure reading: The Arc<String> allows all three threads to read the configuration text simultaneously without any locking overhead because immutable reading is inherently Sync.
* Mutex unlocks mutation: The Mutex forces threads to take turns. Thread #2 safely modifies the string, while other threads are blocked from touching it, eliminating data races.
* Zero Runtime Overhead for Safety: The compiler verifies the presence of Send and Sync at compile time. Your finished binary runs at native speed with no hidden runtime check safety tracking.
