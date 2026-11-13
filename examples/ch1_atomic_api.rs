//! The atomic toolbox on one value. Each call returns what was there before it, so we can
//! see exactly what it did.

use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

fn main() {
    let x = AtomicUsize::new(0);

    x.store(10, Relaxed);
    println!("store(10)            x = {}", x.load(Relaxed));
    let was = x.swap(7, Relaxed);
    println!("swap(7)              was {was:>2}, x = {}", x.load(Relaxed));
    let was = x.fetch_add(5, Relaxed);
    println!("fetch_add(5)         was {was:>2}, x = {}", x.load(Relaxed));
    let was = x.fetch_sub(2, Relaxed);
    println!("fetch_sub(2)         was {was:>2}, x = {}", x.load(Relaxed));
    let was = x.fetch_or(0b1_0000, Relaxed);
    println!("fetch_or(0b1_0000)   was {was:>2}, x = {}", x.load(Relaxed));
    let was = x.fetch_and(0b1_1100, Relaxed);
    println!("fetch_and(0b1_1100)  was {was:>2}, x = {}", x.load(Relaxed));

    // Compare and exchange: "if x is still 24, make it 20". It is, so it does.
    let hit = x.compare_exchange(24, 20, Relaxed, Relaxed);
    println!("cas(24, 20)          {hit:?}, x = {}", x.load(Relaxed));
    // x is 20 now, not 24: nothing changes, and we learn what x really is.
    let miss = x.compare_exchange(24, 99, Relaxed, Relaxed);
    println!("cas(24, 99)          {miss:?}, x = {}", x.load(Relaxed));
}
