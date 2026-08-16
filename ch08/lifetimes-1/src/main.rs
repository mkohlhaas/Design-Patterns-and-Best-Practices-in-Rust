// ========================================== //
// Diving Deeper: Lifetime Syntax and Elision //
// ========================================== //

#![allow(unused, clippy::needless_borrow)]

use std::fmt::Display;

// This won't compile without lifetime annotations!
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // The returned string slice is guaranteed to be valid for as long as both x and y are valid.
    // If either x or y goes out of scope, the returned reference becomes invalid, and the compiler will catch it.
    if x.len() > y.len() { x } else { y }
}

// ================================================= //
// Lifetime Elision Rules: The Compiler's Best Guess //
// ================================================= //

// --------------------------------------------------- //
// Rule 1: Each input reference gets its own lifetime. //
// --------------------------------------------------- //

// Here, x has an input lifetime. Since there's only one input lifetime, it's elided and assigned to
// the output reference. The output reference lives as long as x lives.
fn first_ref<T>(x: &T) -> &T {
    &x
}

// ------------------------------------------------------------------------------------------------------------------------------ //
// Rule 2: If there are multiple input lifetimes, but one of them is &self or &mut self, that lifetime is assigned to the output. //
// ------------------------------------------------------------------------------------------------------------------------------ //

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // `&self` has an input lifetime. Since it's the only input with a lifetime, it's elided and assigned to the output.
    fn x(&self) -> f64 {
        self.x
    }
}

// -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- //
// Rule 3: If there are multiple input lifetimes, and none of them is &self or &mut self, the compiler will analyze the relationships between input lifetimes to determine the output lifetime. //
// -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- //

// see fn longest(...) above

// ==================== //
// Lifetimes in Structs //
// ==================== //

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// ==================== //
// Lifetimes and Traits //
// ==================== //

// A trait that requires a method returning a reference with a lifetime.
// Not very good example. Rust would do this on its own. See Rule 2 above. No lifetime specifiers needed.
trait Summarizable<'a> {
    fn summary(&'a self) -> String;
}

struct Tweet {
    content: String,
}

impl<'a> Summarizable<'a> for Tweet {
    fn summary(&'a self) -> String {
        format!("{}: \"{}\"", "Tweet", self.content)
    }
}

fn main() {
    // ==================== //
    // The 'static Lifetime //
    // ==================== //

    // A reference with a 'static lifetime can live for the entire duration of the program.
    let s: &'static str = "I am a string literal";

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("The important excerpt is: {}", i.part);
    }

    // {
    //     // Does not compile. I lied to the compiler! Hihi!
    //
    //     let i; // We declare our struct here
    //
    //     {
    //         // A new, smaller scope begins
    //         let novel = String::from("Call me Ishmael.");
    //
    //         // We put a reference to `novel` inside our struct
    //         i = ImportantExcerpt {
    //             part: novel.as_str(),
    //         };
    //     } // BOOM! `novel` is destroyed right here because the scope ends!
    //
    //     // We want to print `i.part`, but the data it points to is gone!
    //     println!("{}", i.part);
    // }
}
