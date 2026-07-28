### The Prototype Pattern

In Rust, the Prototype Pattern is a creational design pattern that allows you
to copy or clone existing objects to create new instances instead of
constructing them from scratch. It is particularly useful when creating an
object is computationally expensive, requires heavy IO, or when you need many
similar objects with minor differences.

Because Rust features a built-in infrastructure for copying objects, this
design pattern is uniquely built into the language itself and is usually
trivial to implement.

### The Core: The Clone and Copy Traits

While traditional object-oriented languages rely on a custom interface
containing a .clone() method, Rust standardizes this behavior through the
standard library's std::clone::Clone trait.

- Clone: Used for deep copying. It can duplicate heap-allocated memory and
  complex structures. It is explicit and called using .clone().
- Copy: Used for cheap, bitwise copies (shallow copy) of stack-only data (like
  i32 or bool). It happens implicitly during assignment.

### Basic Implementation

For most types, you do not need to write cloning logic manually. You can
seamlessly implement the pattern using the `#[derive(Clone)]` macro attribute

### When to Use the Pattern in Rust

- Reusing Expensive Computations: If initializing an object requires
  downloading data, reading files, or parsing configuration tables, you can
  execute that cost once, store it in a prototype, and clone it later.
- Polymorphic Cloning (Trait Objects): When you are working with dynamic trait
  objects (&dyn MyTrait or Box<dyn MyTrait>) and need a way to duplicate them at
  runtime without knowing their exact underlying concrete struct.
- Overcoming Struct Update Limits: If a struct contains non-Copy elements,
  Rust's native .. struct update syntax can move ownership rather than copy it.
  Combining a prototype .clone() with field modifications bypasses this
  constraint.

### Copy-on-Write (Cow)

In Rust, Cow (Copy-on-Write) is a smart pointer that optimizes the Prototype
Pattern by avoiding allocations until a modification is strictly
necessary.Instead of cloning an entire object immediately, Cow lets multiple
instances share a read-only blueprint. It will only duplicate and clone the
data into heap memory the exact moment a clone is edited or written to.

### The Problem it Solves

Standard .clone() forces an immediate copy of the data, which wastes memory and
CPU cycles if the clone is never modified. Cow allows you to treat data as a
prototype that can be read instantly for free, while still allowing safe,
isolated edits on demand.

### How Cow Works Internally

std::borrow::Cow is an enum with two variants:
- Borrowed(&T): Wraps a reference to the prototype data. Zero allocation cost.
- Owned(<T as ToOwned>::Owned): Wraps owned, cloned data on the heap. Allocates only when written to.

### Direct Comparison: Clone vs Cow

| Feature | Standard Clone | Cow (Copy-on-Write) |
| --- | --- | --- |
| Allocation Timing | Happens immediately | Postponed until a mutation occurs |
| Memory Cost (Read) | High (duplicates heap memory) | Zero (shares the prototype pointer) |
| Memory Cost (Write) | High (already allocated) | Standard allocation happens at write time |
| Ideal Use Case | Modifying every cloned instance | Modifying only a few cloned instances |

### When to use Cow for Prototypes

- Mass Spawning: You need to create thousands of instances from a baseline
  prototype, but only 5% of them will ever get customized.
- String Processing: You are parsing or sanitizing text templates where most
  strings remain identical to the original asset, but a few need tokens replaced.
- OS Interoperability: Handling system paths or arguments using OsStr or
  network bytes where copying memory should be deferred.

### Summary

We can use: 
- `Clone` for general copying
- `Copy` for trivial types
- `Default` for template objects
- `Arc` for shared templates
- `Cow` for efficient string handling
