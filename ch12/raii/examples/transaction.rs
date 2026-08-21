// Transactions are automatically rolled back if they go out of scope
// unless it is explicitly committed.

// =============== //
// The Transaction //
// =============== //

pub struct Transaction {
    // Represents your actual database connection or state
    is_active: bool,
}

impl Transaction {
    pub fn new() -> Self {
        println!("Starting a transaction.");
        Self { is_active: true }
    }

    pub fn execute(&self, query: &str) {
        println!("Executing '{}'.", query);
    }

    pub fn commit(&mut self) {
        if self.is_active {
            println!("Committing to disk.");
            self.is_active = false;
        }
    }

    pub fn rollback(&mut self) {
        if self.is_active {
            println!("Rolling back.");
            self.is_active = false;
        }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}

// ========================== //
// The RAII Transaction Guard //
// ========================== //

pub struct TransactionGuard<'a> {
    tx: &'a mut Transaction,
    completed: bool,
}

impl<'a> TransactionGuard<'a> {
    pub fn new(tx: &'a mut Transaction) -> Self {
        Self {
            tx,
            completed: false,
        }
    }

    pub fn execute(&self, query: &str) {
        self.tx.execute(query);
    }

    // Explicitly commit the transaction
    pub fn commit(mut self) {
        self.tx.commit();
        self.completed = true; // Prevents rollback in Drop
    }
}

// Automatically rollback if the guard is dropped without committing
impl<'a> Drop for TransactionGuard<'a> {
    fn drop(&mut self) {
        if !self.completed {
            println!("Transaction dropped without committing.");
            self.tx.rollback();
        }
    }
}

fn main() {
    {
        let mut db_connection = Transaction::new();
        let guard = TransactionGuard::new(&mut db_connection);

        guard.execute("INSERT INTO users VALUES ('Alice')");
        guard.commit();
    }

    println!("---");

    {
        let mut db_connection = Transaction::new();
        let guard = TransactionGuard::new(&mut db_connection);

        guard.execute("INSERT INTO users VALUES ('Bob')");

        // this time simulate an error (automatic rollback)
        println!("An error happened.");
    }
}
