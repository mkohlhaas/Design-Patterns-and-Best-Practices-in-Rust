// In classical OOP languages like Java, you can mark the template method as final so subclasses
// cannot modify the algorithm's structure. Rust traits allow structs to override any method,
// including your template method. To strictly enforce a non-overridable template method in Rust, you
// split the code into separate traits using a blanket implementation

// ========================== //
// 1. The Customization Trait //
// ========================== //

// Structs only implement these variable, format-specific steps.
trait DataMinerSteps {
  fn extract_data(&self);
  fn parse_data(&self);

  // Optional hook step
  fn hook(&self) {}
}

// ===================== //
// 2. The Template Trait //
// ===================== //

// Holds the public entry point for the algorithm workflow.
trait DataMiner {
  fn mine(&self, path: &str);
}

// ============================= //
// 3. The Blanket Implementation //
// ============================= //

// Contains the immutable workflow and the shared invariant steps.
impl<T: DataMinerSteps> DataMiner for T {
  fn mine(&self, path: &str) {
    // Enforced algorithmic order
    self.open_file(path);
    self.extract_data();
    self.parse_data();
    self.hook();
    self.close_file(path);
  }
}

// ================================= //
// Why this is safe from overriding? //
// ================================= //

// If a developer tries to break the design rules and write custom workflow logic for PdfMiner like this:

// impl DataMiner for PdfMiner {
//   fn mine(&self, _path: &str) {
//     println!("I am trying to bypass the template!");
//   }
// }

// The Rust compiler will reject it immediately because the blanket implementation already claims
// ownership over how DataMiner behaves for those types.

// ============================================================ //
// 4. Private extension trait to house the shared helper logic. //
// ============================================================ //

trait InvariantSteps {
  fn open_file(&self, path: &str);
  fn close_file(&self, path: &str);
}

// This prevents concrete structs from seeing, calling, or overriding them.
impl<T: DataMinerSteps> InvariantSteps for T {
  fn open_file(&self, path: &str) {
    println!("Opening file system handle for: {}", path);
  }

  fn close_file(&self, path: &str) {
    println!("Closing file handle and flushing buffers for: {}\n", path);
  }
}

// =========================== //
// 5. Concrete Implementations //
// =========================== //

struct PdfMiner;
impl DataMinerSteps for PdfMiner {
  fn extract_data(&self) {
    println!("Extracting raw bytes from PDF...");
  }

  fn parse_data(&self) {
    println!("Parsing PDF byte stream into text.");
  }
}

struct CsvMiner;
impl DataMinerSteps for CsvMiner {
  fn extract_data(&self) {
    println!("Reading rows from CSV...");
  }

  fn parse_data(&self) {
    println!("Parsing CSV rows into structured fields.");
  }

  fn hook(&self) {
    println!("CsvMiner Hook: Validating CSV headers.");
  }
}

fn main() {
  let pdf_worker = PdfMiner;
  let csv_worker = CsvMiner;

  println!("=== Processing PDF ===");
  pdf_worker.mine("report.pdf");

  println!("=== Processing CSV ===");
  csv_worker.mine("data.csv");
}
