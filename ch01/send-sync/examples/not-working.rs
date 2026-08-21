use std::thread;

use std::rc::Rc;

fn main() {
    let unsafe_config = Rc::new(String::from("db_host=localhost;"));
    let config_clone = Rc::clone(&unsafe_config);

    // ⚠️ This is NOT thread-safe and will not compile!
    thread::spawn(move || {
        println!("{}", config_clone);
    });
}
