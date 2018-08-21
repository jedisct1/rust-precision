use super::timestamp::*;

pub(crate) struct CPUCounter;

extern "C" {
    fn cpucounter() -> u64;
}

impl CPUCounter {
    #[inline]
    pub fn current() -> Timestamp {
        Timestamp(unsafe { cpucounter() })
    }
}
