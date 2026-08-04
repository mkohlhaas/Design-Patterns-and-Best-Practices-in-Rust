// ================================================== //
// 1. The Memento: A simple, immutable state wrapper. //
// ================================================== //

#[derive(Clone, Debug)]
pub struct EditorMemento {
    content: String,
}

// =========================================================================== //
// 2. The Originator: The active object that generates and consumes snapshots. //
// =========================================================================== //

pub struct TextEditor {
    content: String,
}

impl Default for TextEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn type_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn print_content(&self) {
        println!("Current Text: \"{}\"", self.content);
    }

    // Creates the Memento snapshot by cloning internal state data
    pub fn save(&self) -> EditorMemento {
        EditorMemento {
            content: self.content.clone(),
        }
    }

    // Restores internal state from a Memento
    pub fn restore(&mut self, memento: EditorMemento) {
        self.content = memento.content;
    }
}

// ============================================================================ //
// 3. The Caretaker: Manages the history array without touching the inner data. //
// ============================================================================ //

pub struct HistoryCaretaker {
    history: Vec<EditorMemento>,
}

impl Default for HistoryCaretaker {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoryCaretaker {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn save_state(&mut self, memento: EditorMemento) {
        self.history.push(memento);
    }

    pub fn undo(&mut self) -> Option<EditorMemento> {
        self.history.pop()
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    let mut editor = TextEditor::new();
    let mut caretaker = HistoryCaretaker::new();

    // Type some text and save the checkpoint
    editor.type_text("Hello, ");
    caretaker.save_state(editor.save());

    // Type more text and save a second checkpoint
    editor.type_text("World!");
    caretaker.save_state(editor.save());

    // Type garbage text that we will eventually want to undo
    editor.type_text(" This will be deleted.");
    editor.print_content(); // Out: "Hello, World! This will be deleted."

    // First undo: Revert to "Hello, World!"
    if let Some(memento) = caretaker.undo() {
        editor.restore(memento);
    }
    editor.print_content(); // Out: "Hello, World!"

    // Second undo: Revert to "Hello, "
    if let Some(memento) = caretaker.undo() {
        editor.restore(memento);
    }
    editor.print_content(); // Out: "Hello, "
}
