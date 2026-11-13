//! The stats are several numbers that must change together, so we need a lock. The first
//! try is made of one atomic bool: wait while it is taken, then take it.

use std::hint::spin_loop;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Default)]
pub struct BrokenLock {
    locked: AtomicBool,
}

impl BrokenLock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        // Wait until nobody holds it...
        while self.locked.load(Ordering::Relaxed) {
            spin_loop();
        }
        // ...then take it.
        self.locked.store(true, Ordering::Relaxed);
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Relaxed);
    }
}
