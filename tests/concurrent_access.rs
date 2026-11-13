//! The SpinLock under real threads. Passing these says the lock works in the schedules we
//! happened to run, nothing more: the next chapter is about the ones we didn't.

use atomforge::spinlock::SpinLock;
use std::sync::Arc;
use std::thread;

#[test]
fn eight_threads_count_to_800_000() {
    let counter = Arc::new(SpinLock::new(0usize));
    let threads: Vec<_> = (0..8)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..100_000 {
                    *counter.lock() += 1;
                }
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
    assert_eq!(*counter.lock(), 800_000);
}

#[test]
fn many_threads_fill_one_vec() {
    let list = SpinLock::new(Vec::new());
    thread::scope(|s| {
        for t in 0..4 {
            let list = &list;
            s.spawn(move || {
                for i in 0..1_000 {
                    list.lock().push(t * 1_000 + i);
                }
            });
        }
    });
    let mut all = list.lock().clone();
    all.sort();
    assert_eq!(all, (0..4_000).collect::<Vec<_>>());
}

#[test]
fn the_guard_unlocks_at_the_end_of_its_scope() {
    let lock = SpinLock::new(0);
    {
        let mut guard = lock.lock();
        *guard += 1;
    } // unlocked here
    assert_eq!(*lock.lock(), 1); // would spin forever if it weren't
}

#[test]
fn the_guard_unlocks_when_its_thread_panics() {
    let lock = Arc::new(SpinLock::new(Vec::new()));
    let crashed = {
        let lock = Arc::clone(&lock);
        thread::spawn(move || {
            let mut list = lock.lock();
            list.push("written before the panic");
            panic!("a worker crashes while holding the lock");
        })
    };
    assert!(crashed.join().is_err());
    // Unwinding dropped the guard, so the lock is free, and the write made it.
    assert_eq!(*lock.lock(), ["written before the panic"]);
}
