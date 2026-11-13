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

    /// One more job done: read, add one and write back, as one indivisible step.
    ///
    /// `Relaxed` is enough: this counter stands alone. Nothing else is published through
    /// it, so the only promise we need is that no increment is lost.
    pub fn finish(&self) {
        self.done.fetch_add(1, Ordering::Relaxed);
    }

    pub fn finished(&self) -> usize {
        self.done.load(Ordering::Relaxed)
    }
}
