extern crate libc;

mod config;
mod cpucounter;
mod precision;
mod timestamp;

pub use self::config::*;
pub use self::precision::*;

#[test]
fn test_simple() {
    use std::thread;
    use std::time::Duration;

    let p = Precision::new(Config::default()).unwrap();
    let start = p.now();
    thread::sleep(Duration::from_secs(2));
    let stop = p.now();
    let elapsed = stop - start;
    assert!(elapsed.as_secs_f64(&p) > 1.0 && elapsed.as_secs_f64(&p) < 4.0);

    let start = p.now();
    let stop = p.now();
    let elapsed = stop - start;
    assert_eq!(elapsed.as_secs(&p), 0)
}
