use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 1. SETUP THREAD-SAFE SHARED DATA
    // We wrap the configuration string in an Arc (Atomic Reference Counted) so it can be shared read-only.
    // Arc implements both Send and Sync because the underlying counter uses atomic operations.
    let shared_config = Arc::new(String::from("db_host=localhost;port=8080;"));

    // We wrap the status counter in a Mutex inside an Arc.
    // This allows us to safely mutate data across threads.
    // Mutex makes a type Sync by ensuring only one thread can access it at a time.
    let shared_status = Arc::new(Mutex::new(String::from("Initializing")));

    // 2. SPAWN WORKER THREADS TO READ DATA
    let mut thread_handles = vec![];

    for thread_id in 1..=3 {
        // Clone the Arc pointers (not the underlying data).
        // This increments the atomic reference counts.
        let config_clone = Arc::clone(&shared_config);
        let status_clone = Arc::clone(&shared_status);

        // thread::spawn requires the closure to be `Send`.
        // Because Arc is Send, this compiles perfectly!
        let handle = thread::spawn(move || {
            // Read-only access via Arc (Sync behavior)
            println!("Thread #{} reading config: {}", thread_id, config_clone);

            // Simulating work
            thread::sleep(Duration::from_millis(50));

            // Mutable access via Mutex (Send/Sync behavior)
            // .lock() blocks the thread until it wins sole access to the data
            if thread_id == 2 {
                let mut status_guard = status_clone.lock().unwrap();
                *status_guard = String::from("Running (Active)");
                println!("Thread #2 updated the system status!");
            }
        });

        thread_handles.push(handle);
    }

    // 3. WAIT FOR ALL THREADS TO FINISH
    for handle in thread_handles {
        handle.join().unwrap();
    }

    // 4. PRINT FINAL STATUS
    // The main thread locks the mutex one last time to read the final result
    let final_status = shared_status.lock().unwrap();
    println!("Final Server Status: {}", *final_status);
}
