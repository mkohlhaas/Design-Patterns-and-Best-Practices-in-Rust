use std::cell::RefCell;
use std::rc::Rc;

// ============================ //
// 1. Define the Observer trait //
// ============================ //

trait Observer {
    fn update(&self, message: &str);
}

// ================================== //
// 2. Define the Subject (Observable) //
// ================================== //

struct Subject {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
}

impl Subject {
    fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    fn register(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn notify(&self, message: &str) {
        for observer in &self.observers {
            // Use interior mutability to safely access the observer
            observer.borrow_mut().update(message);
        }
    }
}

// ================================ //
// 3. Implement a Concrete Observer //
// ================================ //

struct NotificationWidget {
    name: String,
}

impl Observer for NotificationWidget {
    fn update(&self, message: &str) {
        println!("{} received update: {}", self.name, message);
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    let mut subject = Subject::new();

    // Create shared, mutable observers using Rc and RefCell
    let widget_one = Rc::new(RefCell::new(NotificationWidget {
        name: "Widget A".to_string(),
    }));
    let widget_two = Rc::new(RefCell::new(NotificationWidget {
        name: "Widget B".to_string(),
    }));

    // Register observers
    subject.register(widget_one.clone());
    subject.register(widget_two.clone());

    // Broadcast state change
    subject.notify("New data is available!");
}
