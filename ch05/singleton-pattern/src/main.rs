use std::sync::{LazyLock, Mutex};

// 1. Define the data structure
pub struct DatabaseConnection {
    pub url: String,
    pub active_queries: usize,
}

// 2. Define the global, thread-safe Singleton using LazyLock and Mutex
pub static DB_INSTANCE: LazyLock<Mutex<DatabaseConnection>> = LazyLock::new(|| {
    println!("Initializing database connection pool...");
    Mutex::new(DatabaseConnection {
        url: String::from("postgres://localhost:5432"),
        active_queries: 0,
    })
});

fn main() {
    // 3. Access the singleton safely across your application
    {
        let mut db = DB_INSTANCE.lock().unwrap();
        db.active_queries += 1;
        println!("Connection 1: Queries active = {}", db.active_queries);
    } // The Mutex lock drops here, freeing it for other tasks/threads

    {
        let db = DB_INSTANCE.lock().unwrap();
        println!("Connection 2: URL is {}", db.url);
    }
}
