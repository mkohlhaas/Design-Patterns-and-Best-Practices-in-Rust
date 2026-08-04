// Root module: src/lib.rs

pub mod database {
    // 1. The query parser cannot see the connection manager
    pub mod parser {
        pub fn parse_query() {
            // ERROR: `connection_manager` is private outside of `network`
            // crate::database::network::pool::connection_manager();
        }
    }

    // 2. The network module and its children can see it
    pub mod network {

        pub mod pool {
            // This function is visible ONLY inside `database::network` and its children
            pub(in crate::database::network) fn connection_manager() {
                println!("Managing network connections...");
            }
        }

        pub mod client {
            pub fn connect() {
                // This works! `client` is a child of `network`
                super::pool::connection_manager();
            }
        }

        pub fn initialize_network() {
            // This works! `initialize_network` is inside `network`
            pool::connection_manager();
        }
    }
}

fn main() {}
