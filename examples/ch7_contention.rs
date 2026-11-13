//! Symptom 4: "under load the machine freezes". The same work, 4,800,000 increments, split
//! across more and more threads, with our SpinLock and with std's Mutex. Wall time is how
//! long we waited; CPU time is how much of the machine it burned.

use atomforge::spinlock::SpinLock;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;

const TOTAL: usize = 4_800_000;

/// CPU time this process has used so far, in seconds (user + system).
fn cpu_seconds() -> f64 {
    // SAFETY: getrusage only writes into the struct we hand it.
    let usage = unsafe {
        let mut usage: libc::rusage = std::mem::zeroed();
        libc::getrusage(libc::RUSAGE_SELF, &mut usage);
        usage
    };
    let secs = |t: libc::timeval| t.tv_sec as f64 + t.tv_usec as f64 / 1e6;
    secs(usage.ru_utime) + secs(usage.ru_stime)
}

/// Runs `work` on `threads` threads; returns (wall ms, CPU ms).
fn measure(threads: usize, work: impl Fn() + Sync) -> (f64, f64) {
    let (cpu, wall) = (cpu_seconds(), Instant::now());
    thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(&work);
        }
    });
    (
        wall.elapsed().as_secs_f64() * 1e3,
        (cpu_seconds() - cpu) * 1e3,
    )
}

fn main() {
    println!("threads   spin wall   spin CPU  | mutex wall  mutex CPU");
    for threads in [1, 2, 4, 8, 12, 24, 48] {
        let per_thread = TOTAL / threads;
        let spin = SpinLock::new(0usize);
        let (spin_wall, spin_cpu) = measure(threads, || {
            for _ in 0..per_thread {
                *spin.lock() += 1;
            }
        });
        let mutex = Mutex::new(0usize);
        let (mutex_wall, mutex_cpu) = measure(threads, || {
            for _ in 0..per_thread {
                *mutex.lock().unwrap() += 1;
            }
        });
        assert_eq!(*spin.lock(), *mutex.lock().unwrap());
        println!(
            "{threads:>7} {spin_wall:>8.0} ms {spin_cpu:>8.0} ms  | {mutex_wall:>7.0} ms \
             {mutex_cpu:>7.0} ms"
        );
    }
}
