// =============================== //
// The Standard Type Class Pattern //
// =============================== //

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

// ============================================= //
// Generics inside Type Classes (Generic Traits) //
// ============================================= //

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

// ======================================== //
// Using Generics as Type Class Constraints //
// ======================================== //

// The generic T is constrained by the Serializable type class (trait)
fn print_payload<T: Serializable>(payload: T) {
    println!("{}", payload.serialize());
}

// =============================== //
// Generic Associated Types (GATs) //
// =============================== //

// The Problem: Why We Needed GATs

// #[allow(unused)]
// trait StreamingIterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// #[allow(unused)]
// struct MyLines {
//     text: String,
// }

// impl StreamingIterator for MyLines {
//     // ❌ Error: You want to yield a string slice (&str) that borrows from 'text'.
//     // But you have no way to attach a lifetime to this associated type!
//     type Item = &str;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         // Cannot return a borrow because 'Item' has no lifetime parameter
//         Some(&self.text[0..5])
//     }
// }

// ------------------------ //
// The Solution: Using GATs //
// ------------------------ //

// ------------------------------------------------- //
// 1. Define the Streaming Iterator Trait using GATs //
// ------------------------------------------------- //

trait StreamingIterator {
    // The associated type accepts a lifetime parameter
    type Item<'a>
    where
        Self: 'a;

    // `type Item<'a>`
    // This states that the associated type Item is not a fixed, single type. Instead, it is a type
    // constructor that needs a lifetime parameter ('a) passed to it before it can resolve to a real type.
    // It allows the trait to say: "The type of item I return can change its internal lifetime depending
    // on how long you borrow me."

    // `where Self: 'a;`
    // This is an explicit safety bound called an outlives constraint. It literally reads as: "The
    // implementing type (Self) must outlive the lifetime 'a."
    // It acts as a legal contract for the compiler, guaranteeing that the lifetime 'a (the duration of
    // the borrow) cannot possibly last longer than the iterator itself (Self).

    // The returned item is bound to the lifetime of the &mut self borrow
    fn next(&mut self) -> Option<Self::Item<'_>>;

    // This would be the explicit declaration without anonymous lifetime specifier
    // fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

// ----------------------------------------------------------------------- //
// 2. A struct that represents the item we want to yield (the result type) //
// ----------------------------------------------------------------------- //

// It holds a reference to data, meaning it must carry a lifetime ('a).
#[allow(unused)]
#[derive(Debug)]
struct ContextWindow<'a> {
    current: &'a str,
    previous: Option<&'a str>,
}

// ---------------------------- //
// 3. The Iterator State Struct //
// ---------------------------- //

struct ContextIterator {
    words: Vec<String>,
    index: usize,
}

// ----------------------------------- //
// 4. Implement the Streaming Iterator //
// ----------------------------------- //

impl StreamingIterator for ContextIterator {
    // GAT in action: We map the trait's 'a to our ContextWindow's lifetime
    type Item<'a>
        = ContextWindow<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.index >= self.words.len() {
            return None;
        }

        // Get a reference to the current word
        let current = &self.words[self.index];

        // Get a reference to the previous word, if it exists
        let previous = if self.index > 0 {
            Some(self.words[self.index - 1].as_str())
        } else {
            None
        };

        self.index += 1;

        // Return the window. Its lifetime is tied to 'self' via the GAT setup.
        Some(ContextWindow {
            current: current.as_str(),
            previous,
        })
    }
}

// ============================================== //
// GATs with Type Parameters (Emulating Functors) //
// ============================================== //

// ---------------------------- //
// 1. Define the Mappable Trait //
// ---------------------------- //

trait Mappable {
    // The current type contained inside the struct
    type Current;

    // The GAT: Defines what this structure looks like
    // when "plugged" with a new target type `U`
    type Plugged<U>: Mappable;

    // That line combines a Generic Associated Type (GAT) with a Trait Bound constraint.
    //
    // `type Plugged<U>` (The Type Constructor)
    //
    // This states that Plugged is an associated type that accepts a generic type parameter U. It
    // functions as a blueprint or "generic socket."
    // When you implement the trait, you must define what the container looks like when its inner type
    // is changed to U. For example:
    //
    // * For Option<T>, Plugged<U> becomes Option<U>.
    // * For Vec<T>, Plugged<U> becomes Vec<U>.

    // `: Mappable` (The Trait Bound Constraint)
    //
    // The colon (:) introduces a requirement for the compiler. It means: "Whatever concrete type
    // replaces Plugged<U> must also implement the Mappable trait itself."
    //
    // It forces the structural transformation to be infinitely repeatable. Because the resulting
    // container is guaranteed to still be Mappable, you are legally allowed to chain multiple .map()
    // operations back-to-back.

    // The mapping function
    fn map<U, F>(self, f: F) -> Self::Plugged<U>
    where
        F: FnMut(Self::Current) -> U;
}

// ------------------------------------------------------- //
// 2. Implement Mappable for a custom Single-Value Wrapper //
// ------------------------------------------------------- //

#[derive(Debug, PartialEq)]
struct Wrapper<T> {
    value: T,
}

impl<T> Mappable for Wrapper<T> {
    type Current = T;

    // GAT in action: Plugging Wrapper with U creates a Wrapper<U>
    type Plugged<U> = Wrapper<U>;

    fn map<U, F>(self, mut f: F) -> Self::Plugged<U>
    where
        F: FnMut(Self::Current) -> U,
    {
        Wrapper {
            value: f(self.value),
        }
    }
}

// ----------------------------- //
// 3. Implement Mappable for Vec //
// ----------------------------- //

impl<T> Mappable for Vec<T> {
    type Current = T;
    type Plugged<U> = Vec<U>; // Swaps out the inner type T for U

    fn map<U, F>(self, f: F) -> Self::Plugged<U>
    where
        F: FnMut(Self::Current) -> U,
    {
        self.into_iter().map(f).collect() // uses standard iterator map and collect
    }
}

// ----------------------------------------------------- //
// 4. Implement Mappable for Rust's standard Option type //
// ----------------------------------------------------- //

impl<T> Mappable for Option<T> {
    type Current = T;
    type Plugged<U> = Option<U>;

    fn map<U, F>(self, f: F) -> Self::Plugged<U>
    where
        F: FnMut(Self::Current) -> U,
    {
        self.map(f) // Leverages Option's native mapping
    }
}

// ---------------------------------------------------------- //
// 5. A completely abstract function using the Mappable trait //
// ---------------------------------------------------------- //

// This function takes ANY container holding an integer, doubles it, and wraps it in a String
// Notice the process_container function. It has absolutely no idea whether you are passing it a
// Wrapper, an Option, a Vec, or a custom Tree.
fn process_container<M>(container: M) -> M::Plugged<String>
where
    // The expression M: Mappable<Current = i32, Plugged<i32> = M> means: "M is a container type
    // currently holding i32 integers, and if you map it to a new type holding i32 integers, you must
    // get back the exact same container type M."
    M: Mappable<Current = i32, Plugged<i32> = M>,
{
    container
        .map(|num| num * 2)
        .map(|num| format!("Result: {}", num))
}

// The clause `M: Mappable<Current = i32, Plugged<i32> = M>` in the process_container function
// breaks down into three distinct constraints applied to type M:
//
// `M: Mappable`
// What it means: M must implement the Mappable type class.
// Example implementations: Vec<_>, Option<_>, Wrapper<_>.

// `Current = i32`
// What it means: The item currently inside the container M must be a 32-bit signed integer (i32).
// Result: This locks M down so that it must be something like Vec<i32> or Option<i32>. It prevents you from passing a Vec<String>.

// `Plugged<i32> = M`
// What it means: This is an equality constraint on the Generic Associated Type. It states that if
// you take the abstract container shape of M and "plug" an i32 into its generic slot, the resulting
// type must be identical to M.
// Why it matters: This prevents structure mutations. For example, it ensures that if M is Vec<i32>,
// mapping it with an integer-returning function yields another Vec<i32>, rather than mutating into
// an Option<i32> or a Result<i32>.

// Why is this needed in process_container?
// In process_container, you are likely doing something that manipulates integers, mapping them, and
// then expecting the container to maintain its original type context.
// Without the Plugged<i32> = M bound, the compiler cannot verify that the type returned by mapping
// M back onto integers is the same type M you started with. This bound gives the Rust compiler a
// mathematical guarantee of type stability during container transformations.

fn main() {
    println!("========== Standard Type Class Pattern =======================");

    // =============================== //
    // The Standard Type Class Pattern //
    // =============================== //

    {
        let n = 42;
        println!("{:?}", n.serialize())
    }

    println!("========== Generic Traits ====================================");

    // ============================================= //
    // Generics inside Type Classes (Generic Traits) //
    // ============================================= //

    // To call convert from the generic ConvertTo trait, you must explicitly tell Rust what your target
    // type is. Because a single type can implement ConvertTo for multiple different target types,
    // Rust cannot always infer which implementation you want to use.You can handle this in three
    // different ways depending on how you write your code.

    {
        // 1. The Cleanest Way: Explicit Type Annotation

        let my_pack = Pack(42);

        // 1. Explicitly ask for a String
        let as_string: String = my_pack.convert();
        println!("As String: {}", as_string);

        // 2. Explicitly ask for a u32
        let as_u32: u32 = my_pack.convert();
        println!("As u32: {}", as_u32);
    }

    {
        // 2. The Inline Way: Fully Qualified Syntax

        let my_pack = Pack(42);

        // Call the String implementation inline
        println!("Inline String: {}", ConvertTo::<String>::convert(&my_pack));

        // Call the u32 implementation inline
        println!("Inline u32: {}", ConvertTo::<u32>::convert(&my_pack));
    }

    fn print_string(text: String) {
        println!("Printed: {}", text);
    }

    {
        // 3. The Functional Way: Using it with Generic Functions

        let my_pack = Pack(42);

        // Rust knows print_string needs a String, so it automatically calls the ConvertTo<String> implementation.
        print_string(my_pack.convert());
    }

    println!("========== Generics as Type Class Constraints ================");

    // ======================================== //
    // Using Generics as Type Class Constraints //
    // ======================================== //

    {
        print_payload(42);
    }

    println!("========== Generic Associated Types ==========================");

    // =============================== //
    // Generic Associated Types (GATs) //
    // =============================== //

    {
        let mut iter = ContextIterator {
            words: vec!["Rust".to_string(), "is".to_string(), "awesome".to_string()],
            index: 0,
        };

        // Safely iterate through the streaming iterator
        while let Some(window) = iter.next() {
            println!("{:?}", window);
        }
    }

    println!("==============================================================");

    {
        // Mapping over our custom Wrapper
        let initial_wrapper = Wrapper { value: 10 };
        let processed_wrapper = process_container(initial_wrapper);
        println!("{:?}", processed_wrapper);
        // Output: Wrapper { value: "Result: 20" }

        // Mapping over an Option using the exact same generic logic
        let initial_option = Some(50);
        let processed_option = process_container(initial_option);
        println!("{:?}", processed_option);
        // Output: Some("Result: 100")
    }

    println!("==============================================================");
}
