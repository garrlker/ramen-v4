//! Mainly a wrapper for working with both `std` and `parking_lot` interchangeably.
//!
//! None of these functions should panic when used correctly as they're used in FFI.

#[cfg(not(feature = "parking-lot"))]
mod sync {
    use std::ptr;
    pub use std::sync::{Condvar, Mutex, MutexGuard};

    #[inline]
    pub fn cvar_notify_one(cvar: &Condvar) {
        cvar.notify_one();
    }

    pub fn cvar_wait<T>(cvar: &Condvar, guard: &mut MutexGuard<T>) {
        // HACK: Since the signature in `std` sucks and *consumes* the guard,
        // we "move it out" for the duration of the wait (`ptr` avoids dropping).
        unsafe {
            let guard_copy = ptr::read(guard);
            let result = cvar.wait(guard_copy).expect("cvar mutex poisoned (this is a bug)");
            ptr::write(guard, result);
        }
    }

    pub fn mutex_lock<T>(mtx: &Mutex<T>) -> MutexGuard<T> {
        mtx.lock().expect("mutex poisoned (this is a bug)")
    }
}

#[cfg(feature = "parking-lot")]
mod sync {
    pub use parking_lot::{Condvar, Mutex, MutexGuard};

    #[inline]
    pub fn cvar_notify_one(cvar: &Condvar) {
        let _ = cvar.notify_one();
    }

    #[inline]
    pub fn cvar_wait<T>(cvar: &Condvar, guard: &mut MutexGuard<T>) {
        cvar.wait(guard);
    }

    #[inline]
    pub fn mutex_lock<T>(mtx: &Mutex<T>) -> MutexGuard<T> {
        mtx.lock()
    }
}

use std::{cell::UnsafeCell, ops, ptr, sync::Once};

/// Minimal lazily initialized type, similar to the one in `once_cell`.
///
/// Thread safe initialization, immutable-only access.
pub struct LazyCell<T, F = fn() -> T> {
    // Invariant: Written to at most once on first access.
    init: UnsafeCell<Option<F>>,
    ptr: UnsafeCell<*const T>,

    // Synchronization primitive for initializing `init` and `ptr`.
    once: Once,
}

unsafe impl<T, F> Send for LazyCell<T, F> where T: Send {}
unsafe impl<T, F> Sync for LazyCell<T, F> where T: Sync {}

impl<T, F> LazyCell<T, F> {
    pub const fn new(init: F) -> Self {
        Self {
            init: UnsafeCell::new(Some(init)),
            ptr: UnsafeCell::new(ptr::null()),
            once: Once::new(),
        }
    }
}

impl<T, F: FnOnce() -> T> LazyCell<T, F> {
    pub fn get(&self) -> &T {
        self.once.call_once(|| unsafe {
            if let Some(f) = (&mut *self.init.get()).take() {
                let pointer = Box::into_raw(Box::new(f()));
                ptr::write(self.ptr.get(), pointer);
            }
        });

        // SAFETY: A call to `call_once` initialized the pointer
        unsafe { &**self.ptr.get() }
    }
}

impl<T, F: FnOnce() -> T> ops::Deref for LazyCell<T, F> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

pub(crate) use sync::*;
