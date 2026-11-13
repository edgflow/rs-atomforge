//! The lock's building blocks: from std normally, from Loom under `--cfg loom`. The lock's
//! code is the same in both worlds, so Loom checks the very code we ship.

#[cfg(loom)]
pub(crate) use loom::cell::{MutPtr, UnsafeCell};
#[cfg(loom)]
pub(crate) use loom::hint::spin_loop;
#[cfg(loom)]
pub(crate) use loom::sync::atomic::AtomicBool;

#[cfg(not(loom))]
pub(crate) use std::hint::spin_loop;
#[cfg(not(loom))]
pub(crate) use std::sync::atomic::AtomicBool;

/// std's `UnsafeCell` with Loom's interface: `get_mut` hands out a pointer, and the access
/// lasts as long as that pointer. Loom watches that lifetime; here it is a plain pointer.
#[cfg(not(loom))]
#[derive(Debug)]
pub(crate) struct UnsafeCell<T>(std::cell::UnsafeCell<T>);

#[cfg(not(loom))]
impl<T> UnsafeCell<T> {
    pub(crate) fn new(value: T) -> Self {
        Self(std::cell::UnsafeCell::new(value))
    }

    pub(crate) fn get_mut(&self) -> MutPtr<T> {
        MutPtr(self.0.get())
    }
}

#[cfg(not(loom))]
pub(crate) struct MutPtr<T>(*mut T);

#[cfg(not(loom))]
impl<T> MutPtr<T> {
    /// # Safety
    /// The caller must have the right to the value right now: here, by holding the lock.
    // Like Loom's `MutPtr::deref`: `&mut T` from `&self` is sound only because the caller
    // holds the lock, which is why this function is `unsafe`.
    #[allow(clippy::mut_from_ref)]
    pub(crate) unsafe fn deref(&self) -> &mut T {
        unsafe { &mut *self.0 }
    }
}
