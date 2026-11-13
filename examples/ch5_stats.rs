//! The stats, guarded at last: three numbers that change together, from eight workers.

use atomforge::spinlock::SpinLock;
use std::thread;

#[derive(Debug, Default)]
struct Stats {
    jobs: u64,
    bytes: u64,
    slowest_us: u64,
}

const WORKERS: u64 = 8;
const JOBS: u64 = 100_000;

fn main() {
    let stats = SpinLock::new(Stats::default());
    thread::scope(|s| {
        for worker in 0..WORKERS {
            let stats = &stats;
            s.spawn(move || {
                for job in 0..JOBS {
                    let mut st = stats.lock(); // unlocked again when `st` goes away
                    st.jobs += 1;
                    st.bytes += 512;
                    st.slowest_us = st.slowest_us.max(worker * 1_000 + job % 1_000);
                }
            });
        }
    });
    let st = stats.lock();
    println!(
        "jobs: {}  bytes: {}  slowest: {} µs",
        st.jobs, st.bytes, st.slowest_us
    );
}
