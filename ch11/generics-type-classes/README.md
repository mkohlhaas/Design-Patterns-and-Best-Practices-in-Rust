### Generics as Type Classes 

Rust uses traits as its equivalent to type classes, and you can make both
traits and their implementations generic.

If you are coming from Haskell, Rust's trait system is directly inspired by
Haskell's type classes. When you combine Rust's generics with traits (using
trait bounds), you get the exact same compile-time ad-hoc polymorphism found in
functional languages.

### 1. The Standard Type Class Pattern

In Haskell, you define a type class and instantiate it for a type. In Rust, you
define a trait and implement (impl) it for a type.

```rust
// The "Type Class"
trait Serializable {
    fn serialize(&self) -> String;
}

// Implementing it for a specific type
impl Serializable for i32 {
    fn serialize(&self) -> String {
        self.to_string()
    }
}
```

### 2. Generics inside Type Classes (Generic Traits)

You can parameterize the trait itself with a generic type. This allows a single
concrete type to have multiple implementations of the same "type class" based
on the target generic type.

A classic example from the Rust standard library is the From<T> trait, which
allows conversion from one type to another:

```rust
// A Generic Type Class
trait ConvertTo<Target> {
    fn convert(&self) -> Target;
}

struct Pack(u8);

// Implementation 1: Convert Pack to String
impl ConvertTo<String> for Pack {
    fn convert(&self) -> String {
        self.0.to_string()
    }
}

// Implementation 2: Convert Pack to u32
impl ConvertTo<u32> for Pack {
    fn convert(&self) -> u32 {
        self.0 as u32
    }
}
```

### 3. Using Generics as Type Class Constraints

When you write a generic function, you use trait bounds to declare what
capabilities the generic type must have. This is identical to a type class
constraint in functional programming.

```rust
// The generic T is constrained by the Serializable type class (trait)
fn print_payload<T: Serializable>(payload: T) {
    println!("{}", payload.serialize());
}
```

### 4. Associated Types vs. Generics

When creating type classes in Rust, you have two choices for handling
output/related types: Generics or Associated Types.

* Use Generics when a type can have multiple valid implementations of the trait (like Pack converting to both String and u32 above).
* Use Associated Types when there should only be one logical implementation per type (like an Iterator, which only ever yields one specific item type).

```rust
// Type Class with an Associated Type (like a Type Family)
trait Graph {
    type Node; // Associated Type
    type Edge;
    
    fn edges(&self) -> Vec<Self::Edge>;
}
```

### Key Differences from Haskell

   1. Monomorphization: Rust compiles generic traits by creating a concrete copy of the code for every type used. This results in static dispatch with zero runtime overhead, unlike the dictionary-passing style often used by default in Haskell.
   2. No Higher-Kinded Types (HKTs): Rust does not natively allow you to abstract over containers (like trait Monad<M>). You cannot pass a type constructor like Vec or Option as a raw generic argument without evaluating it first. Instead, Rust uses `Generic Associated Types (GATs)` to achieve similar patterns.

### Generic Associated Types (GATs)

Generic Associated Types (GATs) allow you to define associated types inside a
trait that can take their own generic arguments (lifetimes or types). 

Before GATs were stabilized in Rust 1.65 (Nov. 2022), associated types had to be plain,
concrete types. GATs unlock Higher-Kinded Types (HKTs) capabilities in Rust,
allowing you to abstract over containers rather than just the concrete values
inside them. 

### The Problem: Why We Needed GATs

Imagine you want to build a StreamingIterator. Unlike a standard Iterator which
yields values that live independently, a StreamingIterator yields items that
borrow from the iterator itself.

Without GATs, you might try to write this:

```rust
trait StreamingIterator {
    type Item; // Cannot inject a lifetime here!

    fn next(&mut self) -> Option<Self::Item>; 
}
```

This fails. The return type of next must be tied to the lifetime of &mut self.
But the associated type Item is declared at the trait level and cannot
reference the short lifetime of the next function call. 

### The Solution: Using GATs

With GATs, you can add a generic parameter (like a lifetime 'a) directly to the
associated type itself:

```rust
trait StreamingIterator {
    // Look here: The associated type takes a lifetime parameter!
    type Item<'a> where Self: 'a; 

    // Now 'a can tie the returned item to the lifetime of &mut self
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}
```

When a collection implements this trait, it can specify exactly how that
lifetime relates to its data: 

```rust
struct ContextIterator {
    data: Vec<String>,
    index: usize,
}

impl StreamingIterator for ContextIterator {
    // The yielded type is a reference bound to the lifetime 'a
    type Item<'a> = &'a String where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.index < self.data.len() {
            let item = &self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}
```


### GATs with Type Parameters (Emulating Functors)

GATs don't just accept lifetimes; they can also accept types. This allows Rust
to emulate patterns like Functors from functional programming, where you want
to map over a container structure without locking it to a specific type. 

```rust
trait Mappable {
    // A GAT that takes a type parameter
    type Bound<U>;

    // Maps a container of T into a container of U
    fn map<T, U, F>(self, f: F) -> Self::Bound<U>
    where
        F: FnMut(T) -> U;
}
```

### Quick Summary: When to use GATs?

* Borrowed Data: When a trait method needs to return an associated type that borrows from &self or &mut self.
* Container Abstraction: When you want to define a type class that operates on a generic wrapper (like Option<T> or Vec<T>) and alter the inner type T dynamically. 

### Functor pattern

Here is how to implement the Functor pattern in Rust using Generic Associated
Types (GATs).

In functional programming, a Functor is a data structure that can be mapped
over. Because Rust lacks Higher-Kinded Types (HKTs), you cannot pass a type
constructor (like just Option or Vec) as a generic. Instead, GATs allow you to
define a trait where the structure stays the same, but the inner type can
change.

### 1. Defining the Functor Trait

The key is the GAT type Plug<U>. It represents the "container" plugged with a new type U.

```rust
trait Functor {
    // The current inner type of the container
    type Unplugged;

    // The GAT: The same container structure, but holding a new type U
    type Plug<U>: Functor;

    // The map function
    fn fmap<U, F>(self, f: F) -> Self::Plug<U>
    where
        F: FnMut(Self::Unplugged) -> U;
}
```

### 2. Implementing Functor for Option<T>

Here is how you implement this trait for Rust's native Option type.

```rust
impl<T> Functor for Option<T> {
    type Unplugged = T;
    
    // An Option<T> mapped over to U becomes an Option<U>
    type Plug<U> = Option<U>;

    fn fmap<U, F>(self, f: F) -> Self::Plug<U>
    where
        F: FnMut(Self::Unplugged) -> U {
        self.map(f) // Reuses Rust's built-in Option::map
    }
}
```

### 3. Implementing Functor for a Custom Container

You can apply the exact same pattern to your own custom data structures, like a
simple Boxed single-value container:

```rust
struct Container<T> {
    value: T,
}

impl<T> Functor for Container<T> {
    type Unplugged = T;
    type Plug<U> = Container<U>;

    fn fmap<U, F>(self, mut f: F) -> Self::Plug<U>
    where
        F: FnMut(Self::Unplugged) -> U {
        Container {
            value: f(self.value),
        }
    }
}
```

### 4. Writing Generic Code using the Functor

Now you can write a single, completely abstract function that works on any
Functor, whether it is an Option, a Container, or a Vec.

```rust
// This function takes ANY functor holding an i32 and turns it into a String
fn stringify_functor<F>(functor: F) -> F::Plug<String>where
    F: Functor<Unplugged = i32>,
{
    functor.fmap(|num| format!("Number: {}", num))
}

fn main() {
    let original_option = Some(42);
    let mapped_option = stringify_functor(original_option);
    assert_eq!(mapped_option, Some("Number: 42".to_string()));

    let original_container = Container { value: 100 };
    let mapped_container = stringify_functor(original_container);
    assert_eq!(mapped_container.value, "Number: 100".to_string());
}
```


### Limitations of GAT Functors in Rust

While this works perfectly for ownership (consuming self), you will hit
limitations if you try to map over references (e.g., &fmap). To make it work
with lifetimes, your GAT would need to accept both a lifetime and a type
parameter (e.g., type Plug<'a, U>), which significantly increases trait bound
complexity.

### Applicative Functor

To extend the Functor into an Applicative Functor in Rust, we need to introduce
two capabilities:

   1. pure: Lift a raw value into the functor context.
   2. apply (often written as <*>): Apply a function that is already inside a functor to a value inside a functor.

Because Rust's trait system requires a concrete receiver (self) for instance
methods, implementing pure requires a slightly different approach than pure
functional languages.

### 1. Defining the Applicative Trait

We build Applicative on top of our existing Functor trait using Generic
Associated Types (GATs).

```rust
trait Functor {
    type Unplugged;
    type Plug<U>: Functor;

    fn fmap<U, F>(self, f: F) -> Self::Plug<U>
    where
        F: FnMut(Self::Unplugged) -> U;
}

// Applicative extends Functor
trait Applicative: Functor {
    // Lifts a value into the functor structure
    fn pure(value: Self::Unplugged) -> Self;

    // Applies a function wrapped in the functor to a value wrapped in the functor
    fn apply<U, F>(self, fab: Self::Plug<F>) -> Self::Plug<U>
    where
        F: FnMut(Self::Unplugged) -> U;
}
```


### 2. Implementing Applicative for Option<T>

For Option<T>, pure is simply Some, and apply extracts the function and the
value to evaluate them together.

```rust
impl<T> Functor for Option<T> {
    type Unplugged = T;
    type Plug<U> = Option<U>;

    fn fmap<U, F>(self, f: F) -> Self::Plug<U>
    where
        F: FnMut(Self::Unplugged) -> U 
    {
        self.map(f)
    }
}

impl<T> Applicative for Option<T> {
    fn pure(value: Self::Unplugged) -> Self {
        Some(value)
    }

    fn apply<U, F>(self, fab: Self::Plug<F>) -> Self::Plug<U>
    where
        F: FnMut(Self::Unplugged) -> U 
    {
        // Unpack both the function and the value
        match (fab, self) {
            (Some(mut f), Some(a)) => Some(f(a)),
            _ => None,
        }
    }
}
```


### 3. Using Applicative for Multi-Argument Mapping

The most common use case for an Applicative is combining multiple wrapped
values using a multi-argument function. Here is how we can use our abstract
Applicative trait to add two Option values together:

```rust
fn main() {
    let x = Option::pure(5);  // Some(5)
    let y = Option::pure(10); // Some(10)

    // Goal: Compute x + y inside the Option context.
    // Step 1: Curry/Prepare a closure that takes the first argument 'a' 
    // and returns another closure taking 'b'.
    let add = |a| move |b| a + b;

    // Step 2: Use fmap to inject the first value 'x' into the function.
    // This yields: Some(move |b| 5 + b)
    let func_inside_functor = x.fmap(add);

    // Step 3: Use apply to execute that wrapped function on the second value 'y'.
    let result = y.apply(func_inside_functor);

    assert_eq!(result, Some(15));
    println!("Result: {:?}", result);
}
```


### Why is this pattern rare in production Rust?

While GATs make this mathematically elegant, you will rarely see Applicative
traits in real-world Rust code for several reasons:

   1. Ownership and Closures: Rust closures that capture variables (move |b| a + b) change their unique unnameable type based on what they capture. Forcing them into strict GAT signatures requires heavy boilerplate.
   2. Idiomatic Alternatives: Rust provides highly optimized native methods for these operations. Instead of the curried applicative style above, a Rust developer will write:

```rust
   let result = x.zip(y).map(|(a, b)| a + b);
```
   The zip (or and_then) approach achieves the same goal safely, faster, and without complex type-system abstractions.

### Monads

To complete the functional programming trilogy in Rust using Generic Associated
Types (GATs), we implement the Monad.

A Monad extends the Applicative functor by adding the bind operation (often
written as >>= in Haskell, or called flatMap / and_then in other languages).
bind allows you to chain operations where a function takes a raw value but
returns a new wrapped context.


### 1. Defining the Monad Trait

The Monad trait builds on top of Applicative. The core addition is bind, which
pipes a value inside a monad into a function that generates a new monad
structure.

```rust
trait Functor {
    type Unplugged;
    type Plug<U>: Functor;
    fn fmap<U, F>(self, f: F) -> Self::Plug<U> where F: FnMut(Self::Unplugged) -> U;
}

trait Applicative: Functor {
    fn pure(value: Self::Unplugged) -> Self;
    fn apply<U, F>(self, fab: Self::Plug<F>) -> Self::Plug<U> where F: FnMut(Self::Unplugged) -> U;
}

// Monad extends Applicative
trait Monad: Applicative {
    // Takes a value in the Monad, feeds it to a function returning a new Monad,
    // and flattens the nested result.
    fn bind<U, F>(self, f: F) -> Self::Plug<U>
    where
        F: FnMut(Self::Unplugged) -> Self::Plug<U>;
}
```


### 2. Implementing Monad for Option<T>

For Option<T>, bind simply checks if the value is Some. If it is, it executes
the function; if it is None, it short-circuits.

```rust
impl<T> Functor for Option<T> {
    type Unplugged = T;
    type Plug<U> = Option<U>;
    fn fmap<U, F>(self, f: F) -> Self::Plug<U> where F: FnMut(Self::Unplugged) -> U { self.map(f) }
}

impl<T> Applicative for Option<T> {
    fn pure(value: Self::Unplugged) -> Self { Some(value) }
    fn apply<U, F>(self, fab: Self::Plug<F>) -> Self::Plug<U> where F: FnMut(Self::Unplugged) -> U {
        match (fab, self) {
            (Some(mut f), Some(a)) => Some(f(a)),
            _ => None,
        }
    }
}

// The Monad implementation
impl<T> Monad for Option<T> {
    fn bind<U, F>(self, mut f: F) -> Self::Plug<U>
    where
        F: FnMut(Self::Unplugged) -> Self::Plug<U> 
    {
        match self {
            Some(x) => f(x),
            None => None,
        }
    }
}
```


### 3. Practical Usage: Chaining Operations

With bind, you can chain multiple operations that might fail without nesting
match statements.

```rust
fn main() {
    let lookup_user_id = |_name: String| Option::pure(42);
    let fetch_profile = |id: i32| {
        if id == 42 {
            Option::pure("Alice".to_string())
        } else {
            None
        }
    };

    // Chain operations using our abstract abstract Monad trait
    let starting_context = Option::pure("user_1".to_string());
    
    let result = starting_context
        .bind(lookup_user_id)
        .bind(fetch_profile);

    assert_eq!(result, Some("Alice".to_string()));
    println!("Profile found: {:?}", result);
}
```


### How Idiomatic Rust Replaces the Monad

While you can build Monads with GATs, Rust already built this directly into the
core language features using completely different semantics:

   1. The ? Operator: Haskell uses do-notation to make Monad chaining readable. Rust uses the ? operator for Option and Result. It acts as a compiler-level short-circuit mechanism:

```rust
   fn get_profile(name: String) -> Option<String> {
       let id = lookup_user_id(name)?; // Returns early if None
       let profile = fetch_profile(id)?;
       Some(profile)
   }
```

   2. Native Methods: Types like Option and Result already have standard library equivalents to bind called and_then.
