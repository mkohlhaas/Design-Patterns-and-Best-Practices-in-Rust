// 1. A custom struct that OWNS its data.
// By making `content` a `String`, the struct is the sole custodian of that memory.
struct LogReport {
    content: String,
}

// 2. This function CONSUMES ownership (Takes by value)
// Look at the signature: `text: String` (no `&`).
// This function tells the caller: "Give me this data. I own it now, and I will destroy it when I'm done."
fn clean_and_transform(text: String) -> String {
    println!("[Transform] Cleaning up the raw string memory...");
    let trimmed = text.trim();
    trimmed.to_lowercase() // Returns a brand new owned String
} // `text` goes out of scope here and its memory is freed if we didn't return anything.

// 3. This function BORROWS the data immutably (Read-only loan)
// Look at the signature: `&LogReport`.
// It tells the caller: "I just need to look at your data temporarily. You keep ownership."
fn render_report(report: &LogReport) {
    println!("[Render] Printing report: '{}'", report.content);
} // The loan ends here. The caller still owns the report.

// 4. This function BORROWS the data mutably (Exclusive, editable loan)
// Look at the signature: `&mut LogReport`.
// It tells the caller: "I need sole access to change this data temporarily."
fn append_timestamp(report: &mut LogReport) {
    println!("[Modify] Adding timestamp metadata to the report...");
    report.content.push_str(" [Timestamp: 2026-08-21]");
} // The exclusive loan ends here.

fn main() {
    // --- STEP 1: Allocation ---
    // `raw_input` enters the scope. It is the sole owner of the heap memory holding "  USER_DATA  ".
    let raw_input = String::from("  USER_DATA  ");

    // --- STEP 2: The Move (Transferring Ownership) ---
    // We pass `raw_input` into the clean function by value.
    // `raw_input` is MOVED. It no longer exists in `main`.
    let cleaned_text = clean_and_transform(raw_input);

    // UNCOMMENTING THE LINE BELOW WILL CAUSE A COMPILE ERROR:
    // println!("{}", raw_input);
    // Error: use of moved value: `raw_input`

    // --- STEP 3: Creating a Struct that Takes Ownership ---
    // `cleaned_text` is moved into the `LogReport` struct.
    // Now, the variable `my_report` is the sole owner of that memory.
    let mut my_report = LogReport {
        content: cleaned_text,
    };

    // --- STEP 4: Immutable Borrowing (Loaning) ---
    // We pass a reference `&my_report`. We can do this as many times as we want concurrently.
    render_report(&my_report);
    render_report(&my_report); // Works perfectly because reading doesn't change or destroy data.

    // --- STEP 5: Mutable Borrowing (Exclusive Loaning) ---
    // We pass a mutable reference `&mut my_report`.
    // While this loan is active, no one else can read or write to `my_report`.
    append_timestamp(&mut my_report);

    // The mutable loan ended above, so we are allowed to read it again now.
    render_report(&my_report);
} // <-- IMPORTANT: `my_report` goes out of scope here.
// Because it is the sole owner of the data, Rust automatically calls `drop` on it.
// The heap memory holding the text is safely freed right here. No garbage collector required.
