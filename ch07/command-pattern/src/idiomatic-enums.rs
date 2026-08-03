// 1. Concrete action definitions wrapped in a single type
pub enum DocumentCommand {
  AddText(String),
  Clear,
  Backspace(usize),
}

// 2. The state holding structure
pub struct Document {
  pub text: String,
}

// 3. Centralized processing loop
impl Document {
  pub fn execute(&mut self, cmd: DocumentCommand) {
    match cmd {
      DocumentCommand::AddText(t) => self.text.push_str(&t),
      DocumentCommand::Clear => self.text.clear(),
      DocumentCommand::Backspace(n) => {
        let new_len = self.text.len().saturating_sub(n);
        self.text.truncate(new_len);
      }
    }
  }
}

// Usage Example
fn main() {
  let mut doc = Document {
    text: String::new(),
  };

  // Commands can be safely queued in a standard vector without Boxing
  let command_queue = vec![
    DocumentCommand::AddText("Rust Lang".to_string()),
    DocumentCommand::Backspace(5),
    DocumentCommand::AddText("ace".to_string()),
  ];

  for command in command_queue {
    doc.execute(command);
  }

  println!("Final Text: {}", doc.text); // Prints: Rustace
}
