# Precision

Precision is a simple crate to perform measurements using hardware counters.

It is especially useful for performing micro-benchmarks.

On x86, x86-64, AArch64, and RISC-V 64, the crate uses Rust inline assembly on stable Rust.

On s390x Linux, measurements use the hardware time-of-day counter.
Wall-time measurements use the calibration duration set by `Config::setup_duration`.
Cross-compiling for `s390x-unknown-linux-gnu` requires a C cross-compiler, such as `s390x-linux-gnu-gcc`, in addition to the Rust target.

## [API documentation](https://docs.rs/precision)

## Example

```rust
extern crate precision;

let p = precision::Precision::new(precision::Config::default()).unwrap();

let start = p.now();
let stop = p.now();
let elapsed1 = stop - start;

let start = p.now();
let stop = p.now();
let elapsed2 = stop - start;

let elapsed_total = elapsed1 + elapsed2;
let elapsed_total_secs = elapsed_total.as_secs_f64(&p);
let hw_ticks = elapsed_total.ticks();
```
