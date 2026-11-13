//! Symptom 1: "800,000 jobs ran; the counter says fewer."

use std::sync::atomic::{AtomicUsize, Ordering};

/// How many jobs the workers have finished.
#[derive(Debug, Default)]
pub struct JobCounter {
    done: AtomicUsize,
}

impl JobCounter {
    pub const fn new() -> Self {
        Self {
            done: AtomicUsize::new(0),
        }
    }

    /// One more job done. Every load and every store here is atomic.
    pub fn finish(&self) {
        let done = self.done.load(Ordering::Relaxed);
        self.done.store(done + 1, Ordering::Relaxed);
    }

    pub fn finished(&self) -> usize {
        self.done.load(Ordering::Relaxed)
    }
}
