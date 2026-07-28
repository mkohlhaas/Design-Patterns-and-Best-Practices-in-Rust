// A File System Hierarchy
//
// A classic example of the composite pattern is a computer file system where both File (Leaf) and
// Directory (Composite) can display their contents uniformly.

// 1. The Component Trait
trait FileSystemItem {
  fn print(&self, indent: &str);
}

// 2. The Leaf Node
struct File {
  name: String,
}

impl File {
  fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
    }
  }
}

impl FileSystemItem for File {
  fn print(&self, indent: &str) {
    println!("{}{}", indent, self.name);
  }
}

// 3. The Composite Node
struct Directory {
  name: String,
  // We use Box<dyn FileSystemItem> to store different types in the same vector
  children: Vec<Box<dyn FileSystemItem>>,
}

impl Directory {
  fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      children: Vec::new(),
    }
  }

  fn add(&mut self, item: Box<dyn FileSystemItem>) {
    self.children.push(item);
  }
}

impl FileSystemItem for Directory {
  fn print(&self, indent: &str) {
    // Print the directory name itself
    println!("{}{}/", indent, self.name);

    // Recursively print all child elements with an increased indentation
    let new_indent = format!("{}  ", indent);
    for child in &self.children {
      child.print(&new_indent);
    }
  }
}

fn main() {
  // Construct individual leaf nodes
  let file1 = Box::new(File::new("Cargo.toml"));
  let file2 = Box::new(File::new("main.rs"));
  let file3 = Box::new(File::new("README.md"));

  // Build the structural composite hierarchy
  let mut src_dir = Directory::new("src");
  src_dir.add(file2);

  let mut root_dir = Directory::new("project");
  root_dir.add(file1);
  root_dir.add(file3);
  root_dir.add(Box::new(src_dir)); // Nesting a composite into a composite

  // Execute the entry method uniformly over the whole tree structure
  root_dir.print("");
}
