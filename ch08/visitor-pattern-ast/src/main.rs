// ======================================================= //
// 1. Define the AST using standard Rust enums and structs //
// ======================================================= //

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Binary {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Op {
    Add,
    Multiply,
}

// ======================================================== //
// 2. Define the Visitor Trait with default traversal logic //
// ======================================================== //

pub trait Visitor<'ast> {
    // The entry point for visiting any expression node

    fn visit_expr(&mut self, expr: &'ast Expr) {
        // the expr argument is a reference to an Expr that must live at least as long as the 'ast lifetime
        match expr {
            Expr::Number(val) => self.visit_number(*val),
            Expr::Binary { left, op, right } => self.visit_binary(left, op, right),
        }
    }

    fn visit_number(&mut self, _val: i32) {
        // Default: do nothing
    }

    fn visit_binary(&mut self, left: &'ast Expr, _op: &'ast Op, right: &'ast Expr) {
        // Default: walk down the child nodes recursively
        self.visit_expr(left);
        self.visit_expr(right);
    }
}

// ================================================================ //
// 3. Implement a concrete Visitor (e.g., an Interpreter/Evaluator) //
// ================================================================ //

pub struct Evaluator {
    // Visitors usually maintain internal mutable state
    pub result: i32,
}

impl<'ast> Visitor<'ast> for Evaluator {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        // Overriding the entry point to compute values on stack unwind
        match expr {
            Expr::Number(val) => self.result = *val,
            Expr::Binary { left, op, right } => {
                // Visit left child
                self.visit_expr(left);
                let left_val = self.result;

                // Visit right child
                self.visit_expr(right);
                let right_val = self.result;

                // Apply operation
                self.result = match op {
                    Op::Add => left_val + right_val,
                    Op::Multiply => left_val * right_val,
                };
            }
        }
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    // Build AST for: (2 + 3) * 4
    let ast = Expr::Binary {
        left: Box::new(Expr::Binary {
            left: Box::new(Expr::Number(2)),
            op: Op::Add,
            right: Box::new(Expr::Number(3)),
        }),
        op: Op::Multiply,
        right: Box::new(Expr::Number(4)),
    };

    let mut evaluator = Evaluator { result: 0 };
    evaluator.visit_expr(&ast);

    println!("Evaluation Result: {}", evaluator.result); // Outputs: 20
}
