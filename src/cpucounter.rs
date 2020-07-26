use super::timestamp::*;
#[cfg(asm)]
use std::llvm_asm;

pub(crate) struct CPUCounter;

#[cfg(asm)]
#[inline]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn cpucounter() -> u64 {
    let (low, high): (u64, u64);
    asm!("rdtscp", out("eax") low, out("edx") high, out("ecx") _);
    (high << 32) | low
}

#[cfg(asm)]
#[inline]
#[cfg(any(target_arch = "aarch64"))]
unsafe fn cpucounter() -> u64 {
    let (vtm): (u64);
    asm!("mrs {}, cntvct_el0", out(reg) vtm);
    vtm
}

#[cfg(not(asm))]
extern "C" {
    fn cpucounter() -> u64;
}

impl CPUCounter {
    #[inline]
    pub fn current() -> Timestamp {
        Timestamp(unsafe { cpucounter() })
    }
}
