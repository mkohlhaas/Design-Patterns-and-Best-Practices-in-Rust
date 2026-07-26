//! Samsa - A simple publish/subscribe microservice
//!
//! This crate demonstrates architectural patterns through building
//! a working message broker system.

// Core error handling
pub use error::{Result, SamsaError};

pub use broker::Broker;
pub use consumer::Consumer;
pub use message::{Event, Message};
pub use producer::Producer;

mod broker;
mod consumer;
mod error;
mod message;
mod producer;
mod storage;
