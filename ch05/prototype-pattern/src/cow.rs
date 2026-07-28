// How to use Cow to manage prototype configurations or strings without unnecessary memory overhead.

use std::borrow::Cow;

struct UserSession<'a> {
  // Cow can hold either a cheap reference (&str) or an owned string (String)
  role: Cow<'a, str>,
}

fn main() {
  // 1. Create a prototype using a cheap stack reference
  let standard_blueprint = UserSession {
    role: Cow::Borrowed("Guest"),
  };

  // 2. "Clone" the prototype for User 1 (Read-only)
  let session_one = UserSession {
    role: standard_blueprint.role.clone(), // No allocation! Just copies a pointer.
  };

  // 3. "Clone" and modify for User 2 (Write)
  let mut session_two = UserSession {
    role: standard_blueprint.role.clone(), // Still just a pointer reference.
  };

  // Modifying triggers the implicit clone into heap memory via .to_mut()
  session_two.role.to_mut().push_str("_Admin");

  // Verification
  println!("Blueprint: {}", standard_blueprint.role); // Guest
  println!("Session 1: {}", session_one.role); // Guest (Zero allocation)
  println!("Session 2: {}", session_two.role); // Guest_Admin (Allocated on heap)
}
