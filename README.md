# Precision

Precision is a simple crate to perform measurements using hardware counters.

It is especially useful for performing micro-benchmarks.

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
let elapsed_total_secs = elapsed_total_as_secs_f64();
let hw_ticks = elapsed_total.ticks();
```
