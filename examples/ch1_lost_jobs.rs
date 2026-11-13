//! Eight workers, 100,000 jobs each, one shared counter.

use atomforge::counter::JobCounter;
use std::thread;

const WORKERS: usize = 8;
const JOBS: usize = 100_000;

fn main() {
    let counter = JobCounter::new();
    thread::scope(|s| {
        for _ in 0..WORKERS {
            s.spawn(|| {
                for _ in 0..JOBS {
                    counter.finish();
                }
            });
        }
    });
    println!("jobs run:     {}", WORKERS * JOBS);
    println!("counter says: {}", counter.finished());
}
