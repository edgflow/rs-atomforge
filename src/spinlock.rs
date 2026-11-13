//! The lock works now, but nothing stops a worker from touching the stats without it, or
//! from forgetting to unlock. So the lock owns the data, and the only way to the data is
//! through the lock: a guard that unlocks itself when it goes away.

use crate::sync::{AtomicBool, MutPtr, UnsafeCell, spin_loop};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::Ordering;

/// A spin lock that owns its data.
///
/// It can be shared between threads only if `T: Send`. An `Rc` must never reach two
/// threads, so a lock around one can't be shared:
///
/// ```compile_fail
/// fn shared<T: Sync>(_: &T) {}
/// shared(&atomforge::spinlock::SpinLock::new(std::rc::Rc::new(1)));
/// ```
pub struct SpinLock<T> {
    locked: AtomicBool,
    // The one way to mutate through a shared reference. It synchronizes nothing itself:
    // the lock is what makes its use correct.
    data: UnsafeCell<T>,
}

// SAFETY: sharing a `&SpinLock<T>` between threads hands out access to the `T` inside, one
// thread at a time: the lock lets only one guard exist, and Release/Acquire carries each
// owner's writes to the next. So the `T` moves from thread to thread, which is exactly what
// `T: Send` allows. (`Send` for `SpinLock<T>` itself is automatic.)
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        loop {
            // Acquire on success: the last owner's writes are visible to us.
            let taken = self.locked.compare_exchange_weak(
                false,
                true,
                Ordering::Acquire,
                Ordering::Relaxed,
            );
            if taken.is_ok() {
                break;
            }
            // Someone holds it. Wait with plain loads, which only read the cache line, and
            // try to take it again once it looks free.
            while self.locked.load(Ordering::Relaxed) {
                spin_loop();
            }
        }
        SpinLockGuard {
            lock: self,
            data: Some(self.data.get_mut()),
        }
    }
}

/// Proof that we hold the lock: while it exists, the data is ours alone.
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
    // Our access to the data: it starts when the lock is taken, and must end before the
    // lock is given back (see `drop`).
    data: Option<MutPtr<T>>,
}

impl<T> SpinLockGuard<'_, T> {
    fn access(&self) -> &MutPtr<T> {
        self.data.as_ref().expect("the access ends only in drop")
    }
}

impl<T> Deref for SpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // SAFETY: a guard exists only while its owner holds the lock, so nobody else can
        // be writing the data.
        unsafe { self.access().deref() }
    }
}

impl<T> DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: as above, and `&mut self` means this guard is not lent out elsewhere.
        unsafe { self.access().deref() }
    }
}

impl<T> Drop for SpinLockGuard<'_, T> {
    /// Release: every write made through the guard travels with this store to the next
    /// owner, whose Acquire sees them.
    fn drop(&mut self) {
        self.data = None; // our access ends here, while we still hold the lock...
        self.lock.locked.store(false, Ordering::Relaxed); // ...and only then is it free
    }
}
