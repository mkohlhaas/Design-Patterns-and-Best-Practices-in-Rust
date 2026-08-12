### How to correctly unsize an Rc pointer before downgrading it to a Weak trait object pointer

If you try to bypass the intermediate step and write
Rc::downgrade(&dog_strong), the compiler expects a type of Weak<Dog>.

Because Rust's Weak<T> pointer cannot automatically apply the unsizing coercion
rules to turn into Weak<dyn Speaker> on the fly, assigning it directly to a
trait object variable fails. Converting the Rc first utilizes Rust's built-in
CoerceUnsized mechanisms designed specifically for strong smart pointers.

The CoerceUnsized trait in Rust is a standard library mechanism that allows
smart pointers and container types to implicitly convert from a sized type to
an unsized dynamically sized type (DST), such as turning a Box<[i32; 3]> into a
Box<[i32]> or a Box<T> into a Box<dyn Trait>.
