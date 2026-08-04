// Here is how you can implement an Abstract Factory for creating UI elements for different operating systems.

// 1. Abstract Products   (Traits)
// 2. Concrete Products   (implementation of Abstract Products)
// 3. Abstract Factory    (Trait with create functions using Associated Types to link to Abstract Products (1.))
// 4. Concrete Factories
// 5. Client Code         (Static Dispatch: in generic client code use Abstract Factory trait as trait boundary)

// ///////////////////////////////////// //
// --- 1. Abstract Products (Traits) --- //
// ///////////////////////////////////// //

pub trait Button {
    fn render(&self) -> String;
}

pub trait Checkbox {
    fn render(&self) -> String;
}

// //////////////////// //
// 2. Concrete Products //
// //////////////////// //

// /////////////////////////////////////// //
// --- 2.1. Concrete Product (Windows) --- //
// /////////////////////////////////////// //

struct WindowsButton;
impl Button for WindowsButton {
    fn render(&self) -> String {
        "Rendering a Windows-style button.".to_string()
    }
}

struct WindowsCheckbox;
impl Checkbox for WindowsCheckbox {
    fn render(&self) -> String {
        "Rendering a Windows-style checkbox.".to_string()
    }
}

// ///////////////////////////////////// //
// --- 2.2. Concrete Product (Linux) --- //
// ///////////////////////////////////// //

struct LinuxButton;
impl Button for LinuxButton {
    fn render(&self) -> String {
        "Rendering a Linux-style button.".to_string()
    }
}

struct LinuxCheckbox;
impl Checkbox for LinuxCheckbox {
    fn render(&self) -> String {
        "Rendering a Linux-style checkbox.".to_string()
    }
}

// /////////////////////////////////// //
// --- 3. Abstract Factory (Trait) --- //
// /////////////////////////////////// //

// NOTE: We use associated types to define the relationship between the factory and its products!
pub trait UIFactory {
    type B: Button;
    type C: Checkbox;

    fn create_button(&self) -> Self::B;
    fn create_checkbox(&self) -> Self::C;
}

// ///////////////////// //
// 4. Concrete Factories //
// ///////////////////// //

// /////////////////////////////////////// //
// --- 4.1. Concrete Factory (Windows) --- //
// /////////////////////////////////////// //

struct WindowsFactory;
impl UIFactory for WindowsFactory {
    type B = WindowsButton;
    type C = WindowsCheckbox;

    fn create_button(&self) -> Self::B {
        WindowsButton
    }

    fn create_checkbox(&self) -> Self::C {
        WindowsCheckbox
    }
}

// ///////////////////////////////////// //
// --- 4.2. Concrete Factory (Linux) --- //
// ///////////////////////////////////// //

struct LinuxFactory;
impl UIFactory for LinuxFactory {
    type B = LinuxButton;
    type C = LinuxCheckbox;

    fn create_button(&self) -> Self::B {
        LinuxButton
    }

    fn create_checkbox(&self) -> Self::C {
        LinuxCheckbox
    }
}

// //////////////////////////////////////// //
// --- 5. Client Code (Static Dispatch) --- //
// //////////////////////////////////////// //

// Using generics means zero runtime overhead for the abstraction
fn create_ui<F: UIFactory>(factory: F) {
    let button = factory.create_button();
    let checkbox = factory.create_checkbox();

    println!("{}", button.render());
    println!("{}", checkbox.render());
}

fn main() {
    println!("Testing Windows UI:");
    create_ui(WindowsFactory);

    println!("\nTesting Linux UI:");
    create_ui(LinuxFactory);
}
