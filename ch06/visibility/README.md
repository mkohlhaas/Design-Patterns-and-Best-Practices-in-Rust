## Common Visibility Modifiers Comparison
Rust allows you to restrict visibility to specific ancestor modules using several syntax options outlined in [Rust By Example](https://doc.rust-lang.org/rust-by-example/mod/visibility.html):

| Modifier | Visibility Scope |
|---|---|
| fn item() | Visible only within the current module (default/private). |
| pub(self) fn item() | Exactly identical to default private behavior. |
| pub(super) fn item() | Visible only to the parent module. |
| pub(crate) fn item() | Visible across the entire current crate. |
| pub(in path) fn item() | Visible only within a defined ancestor module path. |
| pub fn item() | Visible globally to any code importing the crate. |
