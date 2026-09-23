#include <stdint.h>

#if defined(__x86_64__) || defined(__amd64__)
uint64_t cpucounter(void)
{
    uint64_t low, high;
    __asm__ __volatile__("rdtscp"
                         : "=a"(low), "=d"(high)
                         :
                         : "%ecx");
    return (high << 32) | low;
}
#elif defined(__i386__)
uint64_t cpucounter(void)
{
    uint32_t low, high;
    __asm__ __volatile__("rdtscp"
                         : "=a"(low), "=d"(high)
                         :
                         : "%ecx");
    return ((uint64_t) high << 32) | low;
}
#elif defined(__aarch64__)
uint64_t cpucounter(void)
{
    uint64_t virtual_timer_value;
    __asm__ __volatile__("mrs %0, cntvct_el0"
                         : "=r"(virtual_timer_value));
    return virtual_timer_value;
}
#elif defined(__powerpc__)
uint64_t cpucounter(void)
{
    return __builtin_ppc_get_timebase();
}
#elif defined(__s390x__)
uint64_t cpucounter(void)
{
    uint64_t clock;
    __asm__ __volatile__("stckf %0" : "=Q"(clock) : : "cc");
    return clock;
}
#elif defined(__riscv)
uint64_t cpucounter(void)
{
    uint64_t time;
    __asm__ __volatile__("rdtime %0"
                         : "=r"(time));
    return time;
}
#endif
