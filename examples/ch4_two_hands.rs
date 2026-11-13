//! Release/Acquire fixed the config. Is it enough everywhere? Two schedulers must never run
//! the same job. Each raises its hand, then looks at the other's: if the other hand is
//! down, it goes ahead. Can both see the other's hand down, and both go ahead?

use std::hint::spin_loop;
use std::sync::atomic::{AtomicU64, Ordering, Ordering::*};
use std::thread;

const ROUNDS: u64 = 2_000_000;

/// Rounds in which both schedulers went ahead. `raise` and `look` are the orderings of the
/// hand-raising store and of the look at the other hand.
#[inline(always)]
fn both_went_ahead(raise: Ordering, look: Ordering) -> u64 {
    let (hand_a, hand_b) = (AtomicU64::new(0), AtomicU64::new(0));
    let (saw_a, saw_b) = (AtomicU64::new(0), AtomicU64::new(0));
    let (start, finished) = (AtomicU64::new(0), AtomicU64::new(0));
    let mut both = 0;
    thread::scope(|s| {
        for (mine, other, saw) in [(&hand_a, &hand_b, &saw_a), (&hand_b, &hand_a, &saw_b)] {
            let (start, finished) = (&start, &finished);
            s.spawn(move || {
                for round in 1..=ROUNDS {
                    while start.load(Acquire) != round {
                        spin_loop();
                    }
                    mine.store(1, raise); // my hand goes up...
                    saw.store(other.load(look), Relaxed); // ...then I look at yours
                    finished.fetch_add(1, AcqRel);
                }
            });
        }
        for round in 1..=ROUNDS {
            hand_a.store(0, Relaxed);
            hand_b.store(0, Relaxed);
            start.store(round, Release);
            while finished.load(Acquire) != 2 * round {
                spin_loop();
            }
            if saw_a.load(Relaxed) == 0 && saw_b.load(Relaxed) == 0 {
                both += 1; // each saw the other's hand down: both went ahead
            }
        }
    });
    both
}

fn main() {
    let relaxed = both_went_ahead(Relaxed, Relaxed);
    println!("Relaxed:          both went ahead in {relaxed} of {ROUNDS} rounds");
    let release_acquire = both_went_ahead(Release, Acquire);
    println!("Release/Acquire:  both went ahead in {release_acquire} of {ROUNDS} rounds");
    let seq_cst = both_went_ahead(SeqCst, SeqCst);
    println!("SeqCst:           both went ahead in {seq_cst} of {ROUNDS} rounds");
}
