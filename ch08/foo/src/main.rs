#![allow(unused)]

// ====== //
// Traits //
// ====== //

trait A {
    fn a(&self);
}

trait B {
    fn b(&self);
}

// ======= //
// Structs //
// ======= //

struct Foo {}
impl Foo {
    fn new() -> Self {
        Self {}
    }
}

// ======== //
// Renderer //
// ======== //

impl A for Foo {
    fn a(&self) {
        println!("I am an A.")
    }
}

impl B for Foo {
    fn b(&self) {
        println!("I am a B.")
    }
}

// ========= //
// Functions //
// ========= //

fn check_foo(item: Foo) {
    item.a();
    item.b();
}

// Error: ⚠️ no method named `a` found for reference `&dyn B` in the current scope
// fn check_b(item: Box<dyn B>) {
//     item.a();
//     item.b();
// }

// Implement the B trait specifically for the dynamic A object
impl<'a> B for dyn A + 'a {
    fn b(&self) {
        println!("I am still an A!");
    }
}

// a dynamic A object can call b()
fn check_a(item: Box<dyn A>) {
    item.a();
    item.b();
}

fn main() {
    {
        let f = Foo::new();
        f.a();
        f.b();
    }

    println!();

    {
        let f = Box::new(Foo::new());

        check_a(f);
    }
}
