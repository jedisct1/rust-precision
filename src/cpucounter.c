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
#elif defined(__aarch64__)
uint64_t cpucounter(void)
{
    uint64_t virtual_timer_value;
    __asm__ __volatile__("mrs %0, cntvct_el0"
                         : "=r"(virtual_timer_value));
    return virtual_timer_value;
}
#endif