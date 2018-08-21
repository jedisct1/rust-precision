use super::timestamp::*;

use std::mem;
pub(crate) struct CPUCounter;

impl CPUCounter {
    #[inline]
    pub fn current() -> Timestamp {
        // rdtscp; shlq	$32, %rdx; addq	%rdx, %rax; ret
        let rdtscp: &'static [u8] = &[
            0x0f, 0x01, 0xf9, 0x48, 0xc1, 0xe2, 0x20, 0x48, 0x01, 0xd0, 0xc3,
        ];
        let rdtscp_fn: unsafe extern "C" fn() -> u64 = unsafe { mem::transmute(rdtscp.as_ptr()) };
        Timestamp(unsafe { rdtscp_fn() })
    }
}
