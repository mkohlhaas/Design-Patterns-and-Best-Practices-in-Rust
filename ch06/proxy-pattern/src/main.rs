// Lazy Initialization and Caching Proxy

// 1. The common interface defined as a Trait
trait Database {
    fn fetch_data(&mut self, query: &str) -> String;
}

// 2. The Real Object that does the heavy work
struct RealDatabase;

impl RealDatabase {
    fn new() -> Self {
        println!("Connecting to a heavy database...");
        RealDatabase
    }
}

impl Database for RealDatabase {
    fn fetch_data(&mut self, query: &str) -> String {
        println!("Executing query: {}", query);
        format!("Result for '{}'", query)
    }
}

// 3. The Proxy Object handling caching and lazy initialization
struct DatabaseProxy {
    real_db: Option<RealDatabase>,
    cache: std::collections::HashMap<String, String>,
}

impl DatabaseProxy {
    fn new() -> Self {
        DatabaseProxy {
            real_db: None, // Database is not initialized yet
            cache: std::collections::HashMap::new(),
        }
    }
}

impl Database for DatabaseProxy {
    fn fetch_data(&mut self, query: &str) -> String {
        // Check if the result is already cached
        if let Some(cached_result) = self.cache.get(query) {
            println!("Returning cached data for: {}", query);
            return cached_result.clone();
        }

        // Lazy initialization: Connect only when a real query is made
        if self.real_db.is_none() {
            self.real_db = Some(RealDatabase::new());
        }

        // Delegate the work to the real database object
        let result = self.real_db.as_mut().unwrap().fetch_data(query);

        // Cache the result for future requests
        self.cache.insert(query.to_string(), result.clone());
        result
    }
}

// 4. Client Code
fn main() {
    // The client interacts only with the Trait interface
    let mut db_service = DatabaseProxy::new();

    // First call: Triggers lazy initialization and queries the database
    db_service.fetch_data("SELECT * FROM users");

    // Second call: Fetches directly from the proxy cache
    db_service.fetch_data("SELECT * FROM users");
}
