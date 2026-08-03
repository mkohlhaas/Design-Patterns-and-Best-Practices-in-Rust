// ============================================================ //
// 1. The Trait defines the contract and the algorithm skeleton //
// ============================================================ //

trait DataMiner {
  // The Template Method: This controls the fixed workflow
  fn mine(&self, path: &str) {
    self.open_file(path);
    self.extract_data();
    self.parse_data();
    self.hook(); // Optional lifecycle hook
    self.close_file();
  }

  // Shared default behavior for invariant steps
  fn open_file(&self, path: &str) {
    println!("Opening: {}", path);
  }
  fn close_file(&self) {
    println!("Closing file handle.");
  }

  // Abstract methods: Concrete types must implement these
  fn extract_data(&self);
  fn parse_data(&self);

  // Hook: Optional step with an empty default implementation
  fn hook(&self) {}
}

// ========================== //
// 2. Concrete Implementation //
// ========================== //

// A. Concrete Implementation for PDF files
struct PdfMiner;
impl DataMiner for PdfMiner {
  fn extract_data(&self) {
    println!("Extracting PDF byte stream.");
  }

  fn parse_data(&self) {
    println!("Parsing PDF text blocks.");
  }
}

// B. Concrete Implementation for CSV files
struct CsvMiner;
impl DataMiner for CsvMiner {
  fn extract_data(&self) {
    println!("Reading CSV rows.");
  }

  fn parse_data(&self) {
    println!("Parsing CSV string fields.");
  }

  // Overriding the optional hook
  fn hook(&self) {
    println!("CSV Hook: Checking for headers.");
  }
}

fn main() {
  let pdf = PdfMiner;
  pdf.mine("report.pdf");

  let csv = CsvMiner;
  csv.mine("data.csv");
}
