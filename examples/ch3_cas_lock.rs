//! The same probe on the compare-and-exchange lock: is it ever two workers inside?
//! `inside` counts who is in the critical section; nothing else is touched in there.

use atomforge::cas_lock::CasLock;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::thread;

const WORKERS: usize = 8;
const ROUNDS: usize = 100_000;

fn main() {
    let lock = CasLock::new();
    let inside = AtomicUsize::new(0);
    let collisions = AtomicUsize::new(0);
    thread::scope(|s| {
        for _ in 0..WORKERS {
            s.spawn(|| {
                for _ in 0..ROUNDS {
                    lock.lock();
                    if inside.fetch_add(1, Relaxed) > 0 {
                        collisions.fetch_add(1, Relaxed); // someone else is in here too
                    }
                    inside.fetch_sub(1, Relaxed);
                    lock.unlock();
                }
            });
        }
    });
    println!("lock taken:            {} times", WORKERS * ROUNDS);
    println!("another worker inside: {} times", collisions.load(Relaxed));
}
