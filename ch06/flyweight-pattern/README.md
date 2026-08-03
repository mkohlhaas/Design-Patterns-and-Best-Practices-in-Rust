### The Flyweight Design Pattern

The Flyweight design pattern in Rust is a structural design pattern used to
minimize memory consumption by sharing common, immutable data across thousands
of similar objects instead of duplicating it in each individual instance.

### Core Concepts

The pattern splits object data into two parts:

* *Intrinsic State (shared)*: Constant, heavy data shared across many objects (e.g., textures, fonts, or product templates).
  You can use Rust constructs with shared ownership, e.g. &, Rc, Arc, Cow, 'static, …
* *Extrinsic State (unique)*: Contextual, unique data stored outside the shared instance (e.g., coordinates, unique IDs, or quantities).
  You can use a Hashmap as a flyweight factory.

### When to Use It in Rust

* *Game Development*: Storing repetitive particle layouts, map tiles, or rendering millions of components via instanced rendering.
* *Text Parsers/Editors*: Storing formatting data, fonts, or styling tags applied across thousands of individual glyph instances.
* *String Interning*: Caching unique strings in compilers or data analysis pipelines to avoid duplicate heap allocations (e.g., using the [flyweights crate](https://docs.rs/flyweights) on Docs.rs).
