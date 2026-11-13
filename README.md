# ATOMFORGE

The coordination core of a job runner, rebuilt from its incident report, one symptom at a
time: Rust atomics, memory ordering, compare-and-exchange, a spinlock from scratch, Loom,
and an honest benchmark against `std::sync::Mutex`.

Built in the video *I Caught My CPU Reordering Memory* on
[AI Edge Flow](https://www.youtube.com/@aiedgeflow). Every step of the video is a tag here,
from `ch1-step-0` to `ch7-step-2`.

## The incident

ATOMFORGE passed every test, then failed in production:

1. "Jobs finished" is wrong: 800,000 jobs ran, the counter says fewer.
2. Two workers end up inside the critical section at once.
3. A worker reads a config that is only half there.
4. Under load the machine freezes: every core busy, almost no progress.

## Run it

Every experiment uses atomics only, so none of them has undefined behavior. Run them in
release mode:

```bash
cargo run --release --example ch1_lost_jobs       # the counter
cargo run --release --example ch1_atomic_api      # the atomic toolbox
cargo run --release --example ch2_two_inside      # a lock made of one bool
cargo run --release --example ch3_cas_lock        # compare-and-exchange
cargo run --release --example ch3_guarded_jobs    # a correct lock that loses data
cargo run --release --example ch4_config          # the config and its ready flag
cargo run --release --example ch4_two_hands       # where only SeqCst helps
cargo run --release --example ch5_stats           # SpinLock<T>
cargo test --release                              # the lock under real threads
RUSTFLAGS="--cfg loom" cargo test --release --test loom_model   # Loom
cargo run --release --example ch7_contention      # SpinLock vs Mutex, 1 to 48 threads
cargo bench                                       # one thread, no contention
```

`asm/cas.rs` shows what compare-and-exchange becomes in machine code (commands inside).

## What we measured

On an Apple M2 Pro (ARM64, 8 performance and 4 efficiency cores), Rust 1.100.0, release
builds. Your numbers will differ: these experiments depend on the CPU, so run them.

| Experiment | Result |
|---|---|
| Counter with `load` then `store`, 8 × 100,000 | 219,222 of 800,000 |
| The same with `fetch_add` | 800,000 |
| Lock made of one bool: a second worker already inside | 796,624 times |
| Compare-and-exchange lock | 0 times |
| CAS lock, all `Relaxed`, guarding a counter | 20 of 20 runs lost jobs |
| The same with Acquire on lock, Release on unlock | 0 of 20 |
| Config read older than its ready flag, `Relaxed` | 16,751,658 of 54,063,644 reads |
| The same with Release/Acquire | 0 |
| Both schedulers went ahead, Release/Acquire | 1,997,948 of 2,000,000 rounds |
| The same with `SeqCst` | 0 |
| Loom, correct lock | passes (17 interleavings) |
| Loom, unlock with `Relaxed` | "Causality violation" |
| Alone: lock, add 1, unlock | spinlock 4.6 ns, mutex 7.0 ns |
| 48 threads, 4,800,000 increments | spinlock 3,004 ms (32 s CPU), mutex 71 ms (0.7 s CPU) |

## What this is, and isn't

- `SpinLock` is a lock, not a lock-free data structure.
- It is a teaching implementation, not a replacement for `std::sync::Mutex`: under
  contention it burns every core it can get, and the benchmark shows it.
- Passing stress tests proves nothing about the schedules that didn't happen. Loom explores
  many more, within its limits; it does not prove the hardware memory model either.
- `Relaxed` publishes nothing beyond the atomic itself; `SeqCst` is neither always slow nor
  always needed; compare-and-exchange doesn't solve ABA by itself.

## Your turn

Make the lock fair: a ticket lock, where threads are served in the order they arrived. Or
measure false sharing: put two hot counters on one cache line, then on two.

## License

MIT
