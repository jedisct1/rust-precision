use super::timestamp::*;
pub(crate) struct CPUCounter;

#[cfg(asm)]
#[inline]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn cpucounter() -> u64 {
    let (low, high): (u64, u64);
    asm!("rdtscp" :  "={eax}" (low), "={edx}" (high) : : "ecx");
    (high << 32) | low
}


// https://github.com/google/benchmark/blob/v1.1.0/src/cycleclock.h#L116
#[cfg(asm)]
#[inline]
#[cfg(any(target_arch = "aarch64"))]
unsafe fn cpucounter() -> u64 {
    let (vtm): (u64);
    asm!("mrs %0, cntvct_el0" : "=r"(vtm));
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
