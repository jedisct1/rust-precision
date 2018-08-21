#include <stdint.h>

uint64_t cpucounter(void)
{
    uint64_t low, high;
    __asm__ __volatile__ ("rdtscp" : "=a" (low), "=d" (high) : : "%ecx");
    return (high << 32) + low;
}
