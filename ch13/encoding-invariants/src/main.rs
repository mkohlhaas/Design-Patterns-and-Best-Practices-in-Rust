#![allow(unused)]

// ====================================================== //
// 1. Making Invalid States Unrepresentable (Using Enums) //
// ====================================================== //

// the bad way (loose types)
struct NetworkRequest1 {
    is_cached: bool,
    cache_key: Option<String>, // What if is_cached is true, but cache_key is None?
}

// the encoded way (using enums)
enum NetworkRequest2 {
    LiveRequest,
    CachedRequest { cache_key: String }, // Impossible to have a cached request without a key.
}

// ================================================ //
// 2. The Smart Constructor Pattern (Using Privacy) //
// ================================================ //

// In a separate module
pub struct NonEmptyString(String); // Private inner field

impl NonEmptyString {
    pub fn try_new(val: String) -> Result<Self, &'static str> {
        if val.is_empty() {
            Err("String cannot be empty!")
        } else {
            Ok(Self(val)) // Inside this type, the invariant "never empty" is locked in
        }
    }

    // Exposed getter guarantees safe reading
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ========================================== //
// 3. The Typestate Pattern (Using Ownership) //
// ========================================== //

struct DraftEmail {
    content: String,
}

struct ScheduledEmail {
    content: String,
    date: String,
}

impl DraftEmail {
    // This consumes the DraftEmail. It ceases to exist.
    pub fn schedule(self, date: String) -> ScheduledEmail {
        ScheduledEmail {
            content: self.content,
            date,
        }
    }
}

impl ScheduledEmail {
    pub fn send(&self) {
        /* Only scheduled emails can be sent */
        println!("I'll send you {}.", self.date)
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    let draft = DraftEmail {
        content: "Just an Email".into(),
    };

    // If a developer tries to call .send() on a DraftEmail, the compiler throws an error.
    // draft.send();

    let scheduled = draft.schedule("tomorrow".into());
    scheduled.send();
}
