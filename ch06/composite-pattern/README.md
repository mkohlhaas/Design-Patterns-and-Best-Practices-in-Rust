### The Composite Design Pattern

The Composite pattern lets us treat individual objects and compositions of
objects uniformly through a shared interface.

The Composite design pattern in Rust is a structural design pattern that allows
you to compose objects into tree structures and work with them uniformly
through a single interface. Instead of writing custom logic to distinguish
between an individual object (a Leaf) and a collection of objects (a
Composite), the client code treats both as the exact same type via a shared
Trait.

Because Rust does not support classical object-oriented inheritance, the
pattern is elegantly implemented using Traits and Trait Objects (Box<dyn
Trait>) to achieve runtime dynamic dispatch.

### When to use the Composite pattern

The Composite pattern is most valuable when your data naturally forms a tree structure and you
want to treat individual elements and compositions uniformly. Expression trees, file system
hierarchies, UI widget trees, and organizational charts are all classic examples.

In Rust, an alternative to the Composite pattern is using enums. An Expr enum with variants for
Number(f64) , Variable(String) , BinaryOp { left: Box < Expr > , ... } , and FunctionCall
{ ... } achieves a similar structure without trait objects. The enum approach is better when you
know all expression types at compile time and want exhaustive pattern matching. The trait object
approach is better when you need extensibility: new expression types can be added by
implementing the Expression trait without modifying existing code.

the calculator example uses the trait object approach because it enables the Decorator pattern. If
Expression were an enum, we couldn't wrap arbitrary expressions in LoggingExpression or
CachingExpression without adding decorator variants to the enum itself. This is a concrete
example of the open/closed principle: the trait object approach is open to extension (new expression
types and new decorators) without requiring modification of existing code.

### Core Architecture Components

* *Component Trait*: Defines the common operations or behavior that both individual items and containers share.
* *Leaf Structs*: Represent individual atomic objects that have no children. They implement the component trait directly.
* *Composite Structs*: Represent containers that hold a collection of child components (which can be leaves or other composites). They implement the component trait by recursively calling the operations on their children.

### Trade-offs of Using the Composite Pattern in Rust

While highly flexible, implementing this pattern in Rust carries certain memory
management and safety trade-offs:

### Advantages

* *Uniform Processing*: Client code interacts with the trait, wiping out complex if/else or match conditions checking whether an element is a single item or a group.
* *Extensibility*: You can add entirely new types of leaf or composite components without rewriting or modifying your existing client logic (adhering to the Open/Closed Principle).

### Disadvantages & Rust Constraints

* *Dynamic Dispatch Overhead*: Relying on Box<dyn Trait> shifts function calls from static compile-time lookup to runtime dynamic dispatch, causing a small performance penalty.
* *Indirection*: Elements are allocated on the heap inside Box, which can disrupt CPU cache locality during recursive iterations across deep structural trees.
* *Alternative Approaches*: For small, closed hierarchies, idiomatic Rust often favors using Enums instead of traits. An enum variant can recursively reference vectors of itself (enum Node { Leaf, Composite(Vec<Node>) }). Enums utilize fast static dispatch and match expressions, but they lose out on extensibility because adding a new type requires modifying the core enum definition.
