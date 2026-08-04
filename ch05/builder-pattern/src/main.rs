#![allow(unused)]

// ///////////////////////////////// //
// 1. Consuming Builder (Owned self) //
// ///////////////////////////////// //

// The target struct
#[derive(Debug)]
pub struct Server {
    host: String,
    port: u16,
    timeout: Option<u32>,
}

// NOTE: idiomatic Rust: the type you're building provides the entry point to its builder
impl Server {
    pub fn builder(host: String, port: u16) -> ServerBuilder {
        ServerBuilder::new(host, port)
    }
}

// The builder struct
pub struct ServerBuilder {
    host: String,
    port: u16,
    timeout: Option<u32>,
}

impl ServerBuilder {
    // 1. Initialize builder with mandatory fields
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            timeout: None, // Defaults to None
        }
    }

    // 2. Setter method consumes and returns `Self`
    pub fn timeout(mut self, timeout: u32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    // 3. Finalizer consumes the builder to produce the final object
    pub fn build(self) -> Server {
        Server {
            host: self.host,
            port: self.port,
            timeout: self.timeout,
        }
    }
}

// ////////////////////////////////////////////////////// //
// 2. Non-Consuming Builder (Mutable Reference &mut self) //
// ////////////////////////////////////////////////////// //

impl ServerBuilder {
    pub fn timeout1(&mut self, timeout: u32) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build1(&self) -> Server {
        Server {
            // Error: cannot move out of `self.host` which is behind a shared reference
            host: self.host.clone(), // Requires cloning because `self` is a reference
            port: self.port,
            timeout: self.timeout,
        }
    }
}

fn main() {
    // Elegant, readable fluent interface

    {
        // let server = ServerBuilder::new("127.0.0.1".to_string(), 8080)
        let server = Server::builder("127.0.0.1".to_string(), 8080)
            .timeout(30)
            .build();

        println!("{:?}", server);
    }

    {
        let server = Server::builder("127.0.0.1".to_string(), 8080)
            .timeout1(30)
            .build1();

        println!("{:?}", server);
    }
}
