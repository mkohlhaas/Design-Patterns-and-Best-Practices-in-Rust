// Blogging workflow—moving from Draft, to Pending Review, to Published—reimagined using the
// Typestate pattern.

// ================================== //
// 1. Define the unique state structs //
// ================================== //

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

pub struct PublishedPost {
    content: String,
}

// ================================================= //
// 2. Implement behavior specific to the Draft state //
// ================================================= //

impl Default for DraftPost {
    fn default() -> Self {
        Self::new()
    }
}

impl DraftPost {
    pub fn new() -> Self {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Consumes the DraftPost and returns a PendingReviewPost
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

// ========================================================== //
// 3. Implement behavior specific to the Pending Review state //
// ========================================================== //

impl PendingReviewPost {
    // Consumes the PendingReviewPost and returns a PublishedPost
    pub fn approve(self) -> PublishedPost {
        PublishedPost {
            content: self.content,
        }
    }

    // If rejected, you could transition back to a Draft
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

// ===================================================== //
// 4. Implement behavior specific to the Published state //
// ===================================================== //

impl PublishedPost {
    // Only the Published state exposes the content method!
    pub fn content(&self) -> &str {
        &self.content
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    {
        // A post starts as a draft
        let mut post = DraftPost::new();
        post.add_text("Rust is awesome.");

        // COMPILER ERROR SAMPLES:
        // post.content(); // Error: no method `content` on `DraftPost`
        // post.approve(); // Error: no method `approve` on `DraftPost`

        // Transition to Pending Review (post variable is consumed and dead)
        let pending_post = post.request_review();

        // pending_post.content(); // Error: no method `content` on `PendingReviewPost`

        // Transition to Published (pending_post variable is consumed and dead)
        let published_post = pending_post.approve();

        // Now we can safely read the content
        assert_eq!(published_post.content(), "Rust is awesome.");
    }

    {
        let mut post = DraftPost::new();

        post.add_text("Rust is awesome.");
        let post = post.request_review();
        let post = post.approve();
        assert_eq!(post.content(), "Rust is awesome.");
    }

    {
        let mut post = DraftPost::new();

        post.add_text("Rust is awesome.");
        let post = post.request_review();
        let post = post.reject();
        let post = post.request_review();
        let post = post.approve();
        assert_eq!(post.content(), "Rust is awesome.");
    }
}
