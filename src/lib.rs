//! ATOMFORGE: the coordination core of a job runner. Worker threads take jobs, count what
//! they finish, publish a shared config, and update shared stats. It passed every test, and
//! then failed in production. This crate rebuilds it, one symptom at a time.

pub mod broken_lock;
pub mod cas_lock;
pub mod counter;
pub mod spinlock;
mod sync;
