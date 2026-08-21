In Rust, structs or variables named with "Guard" are part of the RAII (Resource
Acquisition Is Initialization) guard pattern. They lock a resource when created
and automatically clean it up or unlock it via the Drop trait when they go out
of scope.

### Common Standard Library Guards

* MutexGuard: Returned by mutex.lock() to provide safe, exclusive access to data behind a mutex and unlock it when dropped.
* RwLockReadGuard & RwLockWriteGuard: Returned by read/write locks to manage shared or exclusive concurrent access safely.

### Common Ecosystem and Custom Guards

* ScopeGuard: Used to run a deferred closure or cleanup task when leaving a local scope (commonly from the scopeguard crate).
* Custom RAII/Lock Guards: Frequently written in concurrent, graphics, or systems code to track things like active transaction states, GPU render passes, or safety constraints that must reset at the end of a block.
