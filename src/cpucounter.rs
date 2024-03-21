#[cfg(asm)]
#[allow(unused_imports)]
use core::arch::asm;

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
))]
use wasm_bindgen::prelude::*;

use super::timestamp::*;

pub(crate) struct CPUCounter;

#[cfg(asm)]
#[inline]
#[cfg(target_arch = "x86_64")]
unsafe fn cpucounter() -> u64 {
    let (low, high): (u64, u64);
    asm!("rdtscp", out("eax") low, out("edx") high, out("ecx") _);
    (high << 32) | low
}

#[cfg(asm)]
#[inline]
#[cfg(target_arch = "x86")]
unsafe fn cpucounter() -> u64 {
    let (low, high): (u32, u32);
    asm!("rdtscp", out("eax") low, out("edx") high, out("ecx") _);
    ((high as u64) << 32) | (low as u64)
}

#[cfg(asm)]
#[inline]
#[cfg(target_arch = "aarch64")]
unsafe fn cpucounter() -> u64 {
    let vtm: u64;
    asm!("mrs {}, cntvct_el0", out(reg) vtm);
    vtm
}

#[cfg(asm)]
#[inline]
#[cfg(target_arch = "riscv64")]
unsafe fn cpucounter() -> u64 {
    let time: u64;
    asm!("rdtime {}", out(reg) time);
    time
}

#[cfg(all(not(asm), not(any(target_arch = "wasm32", target_arch = "wasm64"))))]
extern "C" {
    fn cpucounter() -> u64;
}

#[cfg(any(
    target_os = "wasix",
    all(target_os = "wasi", not(feature = "wasi-abi2"))
))]
#[inline]
fn cpucounter() -> u64 {
    use wasix::{clock_time_get, CLOCKID_MONOTONIC, CLOCKID_REALTIME};
    unsafe { clock_time_get(CLOCKID_MONOTONIC, 1_000_000) }
        .or_else(|_| unsafe { clock_time_get(CLOCKID_REALTIME, 1_000_000) })
        .expect("Clock not available")
}

#[cfg(all(target_os = "wasi", feature = "wasi-abi2"))]
fn cpucounter() -> u64 {
    let nsec = wasi_abi2::clocks::monotonic_clock::now();
    nsec
}

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
))]
#[wasm_bindgen]
extern "C" {
    #[allow(non_camel_case_types)]
    type performance;

    #[wasm_bindgen(static_method_of = performance)]
    pub fn now() -> f64;
}

#[cfg(all(
    any(target_arch = "wasm32", target_arch = "wasm64"),
    target_os = "unknown"
))]
fn cpucounter() -> u64 {
    (performance::now() * 1_000_000.0) as u64
}

impl CPUCounter {
    #[inline]
    pub fn current() -> Timestamp {
        #[allow(unused_unsafe)]
        Timestamp(unsafe { cpucounter() })
    }
}
