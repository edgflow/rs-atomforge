//! Loom runs this closure again and again, once for every way the two threads can
//! interleave (up to its limits), and checks every access to the lock's data.
//!   RUSTFLAGS="--cfg loom" cargo test --release --test loom_model

#![cfg(loom)]

use atomforge::spinlock::SpinLock;
use loom::sync::Arc;
use loom::thread;

#[test]
fn two_threads_add_one_each() {
    loom::model(|| {
        let lock = Arc::new(SpinLock::new(0usize));
        let other = {
            let lock = Arc::clone(&lock);
            thread::spawn(move || *lock.lock() += 1)
        };
        *lock.lock() += 1;
        other.join().unwrap();
        assert_eq!(*lock.lock(), 2);
    });
}
