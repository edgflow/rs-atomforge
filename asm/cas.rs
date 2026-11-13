//! What compare-and-exchange becomes in machine code. On this Mac:
//!   rustc -O --edition 2024 --crate-type=lib --emit asm asm/cas.rs
//! and without the CPU's single-instruction CAS (LSE), as on older ARM:
//!   rustc -O --edition 2024 --crate-type=lib --emit asm -C target-feature=-lse asm/cas.rs

use std::sync::atomic::{AtomicBool, Ordering};

#[unsafe(no_mangle)]
pub fn weak(lock: &AtomicBool) -> bool {
    lock.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok()
}

#[unsafe(no_mangle)]
pub fn strong(lock: &AtomicBool) -> bool {
    lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok()
}
