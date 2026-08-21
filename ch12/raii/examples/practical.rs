#[derive(Debug)]
struct DatabaseConnection {
    is_active: bool,
}

// 1. Define the Guard struct holding a reference to the resource
#[derive(Debug)]
struct ConnectionGuard<'a> {
    connection: &'a mut DatabaseConnection,
}

// 2. Implement the Drop trait to automate cleanup
impl<'a> Drop for ConnectionGuard<'a> {
    fn drop(&mut self) {
        self.connection.is_active = false;
        println!("Guard dropped: Connection automatically closed safely.");
    }
}

// ⚠️ THIS WILL NOT COMPILE (create and lock in one step)
// fn create_and_lock() -> ConnectionGuard<'static> {
//     let mut db = DatabaseConnection { is_active: true };
//
//     // Error: db does not live long enough
//     ConnectionGuard {
//         connection: &mut db,
//     }
// }

// The lifetime 'a ties the guard's validity to the caller's resource
fn lock_connection<'a>(db: &'a mut DatabaseConnection) -> ConnectionGuard<'a> {
    db.is_active = true;
    ConnectionGuard { connection: db }
}

fn main() {
    // 1. create
    // The resource lives here in the outer scope
    let mut db = DatabaseConnection { is_active: false };
    println!("Database is active: {:?}", db.is_active);

    {
        // 2. lock
        // The guard safely borrows 'db'
        let guard = lock_connection(&mut db);
        println!("Database is active: {:?}", guard.connection.is_active);
    } // guard drops here, unlocking 'db' safely

    println!("Database is active: {:?}", db.is_active);
}
