//! One thread, no contention: what taking and giving back the lock costs, and nothing else.
//!   cargo bench

use atomforge::spinlock::SpinLock;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::sync::Mutex;

fn uncontended(c: &mut Criterion) {
    let spin = SpinLock::new(0u64);
    c.bench_function("spinlock: lock, add 1, unlock", |b| {
        b.iter(|| *black_box(&spin).lock() += 1)
    });
    let mutex = Mutex::new(0u64);
    c.bench_function("mutex: lock, add 1, unlock", |b| {
        b.iter(|| *black_box(&mutex).lock().unwrap() += 1)
    });
}

criterion_group!(benches, uncontended);
criterion_main!(benches);
