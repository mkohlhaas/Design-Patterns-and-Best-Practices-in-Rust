### The Iterator Design Pattern

In Rust, the iterator pattern is built directly into the language via the
[std::iter::Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait, which requires implementing a single method: next.
Rust iterators are lazy, meaning they do not perform computation or allocate
memory until they are explicitly consumed.

### Three Standard Flavors of Iteration

When applying the iterator pattern to custom collections, Rust conventions dictate providing three ways to iterate, based on the ownership rules:

* .iter(): Borrows elements immutably (&T). The collection remains usable afterward.
* .iter_mut(): Borrows elements mutably (&mut T). Allows modification of data in-place.
* .into_iter(): Consumes the collection, transforming it into owned values (T). The original collection can no longer be used.

### Zero-Cost Abstractions

Once you implement next, Rust automatically gives your struct access to over 70
built-in combinators, e.g. map(), filter(), sum(), ….

These optimize down to the same machine code as a manual while loop.
