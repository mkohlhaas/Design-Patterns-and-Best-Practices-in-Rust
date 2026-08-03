### The Strategy Pattern

The Strategy pattern in Rust is a behavioral design pattern that allows you to
define a family of interchangeable algorithms, put each one into a separate
type, and make them interchangeable

## Comparison of Rust Strategy Formats

| Implementation Type | Memory Overhead | Dispatch Cost | Runtime Swappable? | Best Used For |
|---|---|---|---|---|
| Static (<T: Trait>) | None (Zero-Cost) | None (Inlined) | ❌ No | Performance-critical workflows, configurations fixed at application startup. |
| Dynamic (Box<dyn T>) | Pointer allocation | Vtable lookup | Yes | Interactive user choices,插件 architectures, runtime configuration changes. |
| Functional (Fn / FnMut) | Minimal to none | None to minor | ❌ No (unless boxed) | Simple, single-method algorithms lacking complex internal configuration. |
