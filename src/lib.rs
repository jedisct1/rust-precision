#![doc = include_str!("../README.md")]

#![allow(stable_features)]
#![cfg_attr(asm, feature(asm))]

mod config;
mod cpucounter;
mod precision;
mod timestamp;

pub use self::config::*;
pub use self::precision::*;
pub use self::timestamp::*;

#[cfg(not(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
)))]
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

#[cfg(not(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
)))]
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
