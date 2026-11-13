//! The lock never lets two workers in. Now it guards something: the jobs counter, read and
//! written back inside the lock, so the lock is all that keeps an update from being lost.
//! (The counter is atomic, so there is no data race: just ordinary loads and stores.)

use atomforge::cas_lock::CasLock;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::thread;

const WORKERS: usize = 8;
const JOBS: usize = 100_000;
const RUNS: usize = 20;

fn run() -> usize {
    let lock = CasLock::new();
    let jobs = AtomicUsize::new(0);
    thread::scope(|s| {
        for _ in 0..WORKERS {
            s.spawn(|| {
                for _ in 0..JOBS {
                    lock.lock();
                    let done = jobs.load(Relaxed);
                    jobs.store(done + 1, Relaxed);
                    lock.unlock();
                }
            });
        }
    });
    jobs.load(Relaxed)
}

fn main() {
    let results: Vec<usize> = (0..RUNS).map(|_| run()).collect();
    let short = results.iter().filter(|&&n| n != WORKERS * JOBS).count();
    let worst = results.iter().min().unwrap();
    println!("runs:                {RUNS} × {} jobs", WORKERS * JOBS);
    println!("runs that lost jobs: {short}");
    println!("worst run:           {worst}");
}
