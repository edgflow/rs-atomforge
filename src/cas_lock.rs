//! The fix for two workers inside: check and set as one indivisible step. Of all the
//! workers that see `false`, exactly one turns it into `true`; the others learn they lost.

use std::hint::spin_loop;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Default)]
pub struct CasLock {
    locked: AtomicBool,
}

impl CasLock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        // "If it is still false, make it true." Err means someone else got there first, or,
        // with the weak version, that the CPU gave up for a moment: we retry either way.
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Relaxed);
    }
}
