use super::timestamp::*;
#[cfg(asm)]
use std::llvm_asm;

pub(crate) struct CPUCounter;

#[cfg(asm)]
#[inline]
unsafe fn cpucounter() -> u64 {
    let (low, high): (u64, u64);
    llvm_asm!("rdtscp" :  "={eax}" (low), "={edx}" (high) : : "ecx");
    (high << 32) | low
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
