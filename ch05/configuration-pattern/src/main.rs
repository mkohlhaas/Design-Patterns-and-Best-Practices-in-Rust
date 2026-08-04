#![allow(unused)]

#[derive(Debug)]
pub struct CacheConfig {
    pub capacity: usize,
    pub ttl_seconds: u64,
    pub eviction_policy: String,
}

impl CacheConfig {
    // Convenient entry point to access the builder
    pub fn builder() -> CacheConfigBuilder {
        CacheConfigBuilder::default()
    }
}

// The Builder struct mirrors the target fields without using Options
#[derive(Debug)]
pub struct CacheConfigBuilder {
    capacity: usize,
    ttl_seconds: u64,
    eviction_policy: String,
}

// Provide meaningful production fallbacks via the Default trait
impl Default for CacheConfigBuilder {
    fn default() -> Self {
        Self {
            capacity: 1000,
            ttl_seconds: 3600,
            eviction_policy: "LRU".to_string(),
        }
    }
}

impl CacheConfigBuilder {
    // Chainable modifiers that consume and return Self
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn ttl_seconds(mut self, ttl_seconds: u64) -> Self {
        self.ttl_seconds = ttl_seconds;
        self
    }

    pub fn eviction_policy(mut self, policy: impl Into<String>) -> Self {
        self.eviction_policy = policy.into();
        self
    }

    // Zero-cost conversion since everything is already validated and allocated
    pub fn build(self) -> CacheConfig {
        CacheConfig {
            capacity: self.capacity,
            ttl_seconds: self.ttl_seconds,
            eviction_policy: self.eviction_policy,
        }
    }
}

fn main() {
    // Method 1: Using the explicit builder entry point
    let small_cache = CacheConfig::builder().capacity(100).build();

    // Method 2: Initializing the builder structure via the Default trait directly
    let custom_policy_cache = CacheConfigBuilder::default()
        .eviction_policy("FIFO")
        .ttl_seconds(60)
        .build();

    println!("Small Cache: {small_cache:?}");
    println!("Custom Cache: {custom_policy_cache:?}");
}
