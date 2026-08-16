### The Visitor Pattern

You can apply the visitor pattern to an Abstract Syntax Tree (AST) in Rust.
However, while traditional Object-Oriented languages rely on double-dispatch
(accept methods), idiomatic Rust leverages traits and pattern matching to
achieve the same result with significantly less boilerplate.

Industry-standard crates like syn (used for parsing macros) use a hybrid
approach: they combine a Visitor trait with standard default implementations to
make traversing massive syntax trees clean and fast.

### When to Use Traits vs. Rust Enums
Because Rust handles enum checking at compile time via match, you don't always need the visitor pattern. [3] 

| Pattern Structure | When to Choose It | Pros / Cons |
|---|---|---|
| Enum Pattern Matching | Simple ASTs, local compilers, few data types. | 🟢 No boilerplate, zero abstraction cost. 🔴 Adding a new operation means writing a giant, separate match block. |
| Trait Visitor Pattern | Huge ASTs (e.g., JS/Python tooling), linters, or type checkers. | 🟢 Highly modular. You can write custom behavior for just one specific node type and let defaults handle the rest. |


### Lifetime Parameters in the Sample Code

In Rust, lifetime parameters (like 'ast or 'a) are compile-time labels that tell the borrow checker exactly how long a reference remains valid. They do not change how long a value actually lives; they simply guarantee that references never outlive the data they point to. [1, 2, 3, 4, 5] 
In the AST visitor pattern example, lifetimes are critical because the visitor framework relies on borrowed references (&Expr) rather than cloning the entire syntax tree.

### Breaking Down the Trait Lifetime: impl<'ast> Visitor<'ast>
Let's dissect the signature of the trait and its methods to see exactly what the compiler sees:

```rust
pub trait Visitor<'ast> {
    fn visit_expr(&mut self, expr: &'ast Expr);
    fn visit_binary(&mut self, left: &'ast Expr, _op: &'ast Op, right: &'ast Expr);
}
```

* <'ast> declaration: This introduces a generic lifetime parameter named ast. You can name it anything (like 'a), but 'ast explicitly shows it belongs to the AST's memory lifespan.
* expr: &'ast Expr: This tells the compiler: "The expr argument is a reference to an Expr that must live at least as long as the 'ast lifetime." [6] 

### Why the Visitor Needs a Lifespan Label

When you create a concrete visitor struct, it often needs to store references
to the AST it is visiting, or you might want to call multiple visit methods
sequentially on the same AST.

By tying the trait implementation to 'ast, Rust ensures a critical safety rule:
**The Visitor instance cannot outlive the Expr tree it is currently inspecting.**

If the AST is dropped or goes out of scope while the Evaluator visitor is still trying
to look at it, the code will fail to compile, preventing a "use-after-free" bug
or a dangling pointer.

### Why &mut self Doesn't Have a Lifetime Label

You might notice that &mut self does not have a lifetime attached to it (it isn't written as &'ast mut self). This is an intentional and important design pattern:

   1. Short-Lived Mutex: &mut self uses lifetime elision. The compiler automatically assigns it a short, temporary lifetime unique to that specific function call.
   2. Allowing Re-use: If you wrote fn visit_expr(&'ast mut self, ...), you would lock the entire visitor to the AST's lifetime. Because it is a mutable reference, you would only ever be able to call visit_expr exactly once before the visitor became permanently locked and unusable.
   3. The Fix: Leaving &mut self unbound allows you to call .visit_expr() over and over again recursively on different branches of the tree.

### Summary Checklist for Lifetimes in Visitors

* 'ast represents the lifespan of the syntax tree allocation in memory.
* &'ast Expr guarantees the tree data will not be deleted while the visitor is reading it.
* Elided &mut self allows the visitor to maintain internal state and recurse deeply without locking itself out of future mutations.
