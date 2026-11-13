//! Symptom 3: "a worker read a config that was only half there". The publisher writes the
//! config, then raises the flag that says which version is ready. A worker reads the flag,
//! then the config it promises. The config is a version number here, so a stale read shows.

use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

const VERSIONS: u64 = 50_000_000;

fn main() {
    let config = AtomicU64::new(0);
    let ready = AtomicU64::new(0);
    let (mut reads, mut stale) = (0u64, 0u64);
    thread::scope(|s| {
        s.spawn(|| {
            for v in 1..=VERSIONS {
                config.store(v, Ordering::Relaxed); // write the config...
                // ...then publish it: everything written before this store travels with it.
                ready.store(v, Ordering::Release);
            }
        });
        loop {
            // Acquire pairs with the Release: seeing version `v` here means seeing all that
            // was written before it was published.
            let promised = ready.load(Ordering::Acquire);
            let got = config.load(Ordering::Relaxed);
            reads += 1;
            if got < promised {
                stale += 1; // the flag said `promised`, the config was older
            }
            if promised == VERSIONS {
                break;
            }
        }
    });
    println!("reads:                    {reads}");
    println!("config older than flag:   {stale}");
}
