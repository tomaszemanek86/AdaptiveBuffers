#include <utest/utest.h>
#include "no_type.h"
#include <limits>

using namespace no_type;

UTEST_MAIN();

UTEST(bswap, u8) {
    uint8_t value = 10;
    ASSERT_EQ(abf::bswap8(abf::bswap8(value)), value);
}

UTEST(bswap, u16) {
    uint16_t value = 45885;
    ASSERT_EQ(abf::bswap16(abf::bswap16(value)), value);
}

UTEST(bswap, u32) {
    uint32_t value = 45885;
    ASSERT_EQ(abf::bswap32(abf::bswap32(value)), value);
}

UTEST(bswap, u64) {
    uint64_t value = 7845885;
    ASSERT_EQ(abf::bswap64(abf::bswap64(value)), value);
}
