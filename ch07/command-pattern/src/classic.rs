// ============================================ //
// 1. The Receiver: Holds the application state //
// ============================================ //

#[derive(Default)]
pub struct Document {
    pub text: String,
}

impl Document {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

// =============================================== //
// 2. The Command Trait: Decouples execution logic //
// =============================================== //

pub trait Command {
    fn execute(&mut self, doc: &mut Document);
    fn undo(&mut self, doc: &mut Document);
}

// ============================================== //
// 3. Concrete Command: Adds text to the document //
// ============================================== //

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

// ================================================================= //
// 4. The Command Processor: Manages history and schedules execution //
// ================================================================= //

pub struct CommandProcessor {
    // not absolutely necessary, but you'd need to provide a Document in the functions, e.g. execute_command, undo_command, …
    doc: Document,
    history: Vec<Box<dyn Command>>, // Stores trait objects
}

impl Default for CommandProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandProcessor {
    pub fn new() -> Self {
        Self {
            doc: Default::default(),
            history: Vec::new(),
        }
    }

    pub fn execute_command(&mut self, mut command: Box<dyn Command>) {
        command.execute(&mut self.doc);
        self.history.push(command);
    }

    pub fn undo_command(&mut self) {
        if let Some(mut command) = self.history.pop() {
            command.undo(&mut self.doc);
        }
    }

    // getters wouldn't be needed when you provide a Document for the functions, e.g.
    // execute_command, undo_command, ...
    pub fn get_document(&self) -> &Document {
        &self.doc
    }

    pub fn get_document_mut(&mut self) -> &mut Document {
        &mut self.doc
    }
}

fn main() {
    let mut processor = CommandProcessor::new(); // a Document is automatically created

    processor.execute_command(Box::new(AddTextCommand::new("Hello ")));
    processor.execute_command(Box::new(AddTextCommand::new("World!")));
    println!("Current: {}", processor.get_document().text); // Prints: Hello World!

    processor.undo_command();
    println!("Undo: {}", processor.get_document().text); // Prints: Hello 
}
