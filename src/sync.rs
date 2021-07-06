//! Wrapper for working with both `std` and `parking_lot` interchangeably.
//!
//! None of these functions should panic when used correctly as they're used in FFI.

#[cfg(not(feature = "parking-lot"))]
mod sync {
    pub use std::sync::{Condvar, Mutex, MutexGuard};
    use std::ptr;

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

pub(crate) use sync::*;
