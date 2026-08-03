// 1. The Receiver: Holds the application state
pub struct Document {
  pub text: String,
}

// 2. The Command Trait: Decouples execution logic
pub trait Command {
  fn execute(&mut self, doc: &mut Document);
  fn undo(&mut self, doc: &mut Document);
}

// 3. Concrete Command: Adds text to the document
pub struct AddTextCommand {
  text_to_add: String,
}

impl AddTextCommand {
  pub fn new(text: &str) -> Self {
    Self {
      text_to_add: text.to_string(),
    }
  }
}

impl Command for AddTextCommand {
  fn execute(&mut self, doc: &mut Document) {
    doc.text.push_str(&self.text_to_add);
  }

  // very special form of undo;-)
  // just for demonstration purposes
  fn undo(&mut self, doc: &mut Document) {
    let new_len = doc.text.len().saturating_sub(self.text_to_add.len());
    doc.text.truncate(new_len);
  }
}

// 4. The Invoker: Manages history and schedules execution
pub struct HistoryInvoker {
  history: Vec<Box<dyn Command>>, // Stores trait objects
}

impl HistoryInvoker {
  pub fn new() -> Self {
    Self {
      history: Vec::new(),
    }
  }

  pub fn execute_command(&mut self, mut command: Box<dyn Command>, doc: &mut Document) {
    command.execute(doc);
    self.history.push(command);
  }

  pub fn undo_command(&mut self, doc: &mut Document) {
    if let Some(mut command) = self.history.pop() {
      command.undo(doc);
    }
  }
}

impl Default for HistoryInvoker {
  fn default() -> Self {
    Self::new()
  }
}

// Usage Example
fn main() {
  // the receiver
  let mut doc = Document {
    text: String::new(),
  };

  // the invoker
  let mut invoker = HistoryInvoker::new();

  // invoker executes commands on the receiver
  invoker.execute_command(Box::new(AddTextCommand::new("Hello ")), &mut doc);
  invoker.execute_command(Box::new(AddTextCommand::new("World!")), &mut doc);
  println!("Current: {}", doc.text); // Prints: Hello World!

  invoker.undo_command(&mut doc);
  println!("After Undo: {}", doc.text); // Prints: Hello 
}
