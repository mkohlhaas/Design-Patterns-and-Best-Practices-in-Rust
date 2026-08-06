### The Typestate Pattern

The Typestate pattern is Rust’s native superpower for managing states. Instead
of using traits or enums to check states at runtime, you encode the states
directly into distinct types.

By consuming self by value during transitions, the compiler physically destroys
the old state. This makes it statically impossible to call invalid methods
(like trying to approve a draft or view the text of a pending post).

## Why this is superior to the Classic OO Pattern

## 1. Zero Runtime Overhead

The classic pattern requires Box<dyn State>, which relies on heap allocation
and dynamic dispatch (vtable lookups at runtime). The Typestate pattern uses
standard, flat structs. The compiler tracks the states, meaning it compiles
down to highly optimized, direct function calls with zero runtime cost.

## 2. Goodbye Option::take() and unwrap()

In the classic approach, we had to use Option wrapped around states and call
.take().unwrap() to dance around Rust's ownership rules. Because Typestate
methods take self directly, ownership transitions are natural, clean, and
completely safe from panics.

## 3. Absolute API Safety

In the classic OO example, you could call post.content() on a draft, and it
would silently return an empty string "".

With the Typestate pattern, attempting to call .content() on a DraftPost
results in a compile-time error. Your users cannot write code that breaks the
business logic of your state machine.

## Reducing Code Duplication (Advanced Tip)

If your states share a lot of fields (e.g., author, date, tags) and you don't
want to copy them into every state struct, you can use a generic wrapper with
marker structs:

```rust
// Marker structs (zero-sized types)
pub struct Draft;
pub struct PendingReview;
pub struct Published;

// Generic wrapper
pub struct Post<State> {
    state: State,
    pub content: String,
    pub author: String, // Shared field
}

// Methods available only to Drafts
impl Post<Draft> {
    pub fn request_review(self) -> Post<PendingReview> {
        Post {
            state: PendingReview,
            content: self.content,
            author: self.author,
        }
    }
}
```
```
