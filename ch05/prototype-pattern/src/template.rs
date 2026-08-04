#![allow(unused)]

// Advanced Example: Managing Expensive Configurations
//
// The Rust Patterns Book highlights a highly practical scenario where a template engine compiles
// files upon startup. Recompiling these templates for every separate worker pool is inefficient.
// Instead, you can construct a base prototype and alter the settings for individual instances.

use std::collections::HashMap;

#[derive(Clone)]
struct TemplateEngine {
    // Cloning a HashMap duplicates its heap data safely
    templates: HashMap<String, String>,
    strict_mode: bool,
}

impl TemplateEngine {
    // Expensive baseline initialization
    fn init_base() -> Self {
        let mut templates = HashMap::new();
        templates.insert("header".to_string(), "<html>...".to_string());
        templates.insert("footer".to_string(), "...</html>".to_string());

        Self {
            templates,
            strict_mode: true,
        }
    }

    // Prototype modification method
    fn spawn_variant(&self, strict: bool) -> Self {
        let mut engine_copy = self.clone(); // Reuses already allocated template strings
        engine_copy.strict_mode = strict;
        engine_copy
    }
}

fn main() {
    // Heavy weight initialization happens once
    let master_prototype = TemplateEngine::init_base();

    // Fast, lightweight clones generated dynamically
    let dev_environment = master_prototype.spawn_variant(false);
    let prod_environment = master_prototype.clone();
}
