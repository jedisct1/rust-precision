#![allow(stable_features)]
#![cfg_attr(asm, feature(asm))]

//! Precision is a simple crate to perform measurements using hardware counters.
//!
//! It is especially useful for performing micro-benchmarks.
//!
//! Example
//! ```rust
//! extern crate precision;
//!
//! let p = precision::Precision::new(precision::Config::default()).unwrap();
//!
//! let start = p.now();
//!
//! let stop = p.now();
//! let elapsed1 = stop - start;
//!
//! let start = p.now();
//! let stop = p.now();
//! let elapsed2 = stop - start;
//!
//! let elapsed_total = elapsed1 + elapsed2;
//! let elapsed_total_secs = elapsed_total.as_secs_f64(&p);
//! let hw_ticks = elapsed_total.ticks();
//! ```

mod config;
mod cpucounter;
mod precision;
mod timestamp;

pub use self::config::*;
pub use self::precision::*;
pub use self::timestamp::*;

#[test]
fn test_simple() {
    use std::thread;
    use std::time::Duration;

    let p = Precision::new(Config::default()).unwrap();
    let start = p.now();
    thread::sleep(Duration::from_secs(2));
    let stop = p.now();
    let elapsed = stop - start;
    assert!(elapsed.as_secs_f64(&p) > 1.0 && elapsed.as_secs_f64(&p) < 40.0);
    assert!(elapsed.ticks() > 0);
}

#[test]
fn test_no_wall_time() {
    use std::thread;
    use std::time::Duration;

    let p = Precision::new(Config::default().wall_time(false)).unwrap();

    let start = p.now();
    thread::sleep(Duration::from_millis(1));
    let stop = p.now();

    let elapsed = stop - start;
    assert!(elapsed.ticks() > 0);

    let result = std::panic::catch_unwind(|| elapsed.as_millis(&p));
    assert!(result.is_err());
}
