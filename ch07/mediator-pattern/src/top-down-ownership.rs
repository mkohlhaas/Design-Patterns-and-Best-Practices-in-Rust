// Components are simple structures.
//
// The mediator orchestrates all interactions by taking ownership of the components and managing state transitions from the top level.

// ======================== //
// 1. Define the components //
// ======================== //

struct InputField {
    pub value: String,
}

struct SubmitButton {
    pub enabled: bool,
}

// ========================================================== //
// 2. Define the Mediator that completely owns the components //
// ========================================================== //

struct FormMediator {
    input_field: InputField,
    submit_button: SubmitButton,
}

impl FormMediator {
    fn new() -> Self {
        Self {
            input_field: InputField {
                value: String::new(),
            },
            submit_button: SubmitButton { enabled: false },
        }
    }

    // Components notify the mediator about events by passing values,
    // rather than holding a mutable reference back to the mediator.
    pub fn handle_input_change(&mut self, text: String) {
        self.input_field.value = text;

        // Mediator encapsulates the interaction logic between components
        self.submit_button.enabled = !self.input_field.value.is_empty();
    }

    pub fn print_status(&self) {
        println!(
            "Input: '{}', Button Enabled: {}",
            self.input_field.value, self.submit_button.enabled
        );
    }
}

fn main() {
    let mut form = FormMediator::new();

    // Simulating events coming into the system top-down
    form.handle_input_change("Hello Rust!".to_string());
    form.print_status(); // Output: Input: 'Hello Rust!', Button Enabled: true

    form.handle_input_change("".to_string());
    form.print_status(); // Output: Input: '', Button Enabled: false
}
