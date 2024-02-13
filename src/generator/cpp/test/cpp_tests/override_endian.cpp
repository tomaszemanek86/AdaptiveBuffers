#include <utest/utest.h>
#include "override_endian.h"
#include <iostream>

using namespace override_endian;

UTEST_MAIN();

UTEST(override_endian, serde_second_0) {
    uint8_t buffer[1024];
    DifferentEndiansSer ser;
    ser.with_member_8(1);
    ser.with_member_8b(1);
    ser.with_member_8l(1);
    auto size = ser.serialize(buffer);
    ASSERT_EQ(size, 3*1 + 3*2);
    ASSERT_EQ(buffer[0], 1);
    ASSERT_EQ(buffer[1], 1);
    ASSERT_EQ(buffer[2], 128);
    ASSERT_EQ(*reinterpret_cast<uint16_t*>(&buffer[3]), 1);
    ASSERT_EQ(*reinterpret_cast<uint16_t*>(&buffer[5]), 1);
    ASSERT_EQ(*reinterpret_cast<uint16_t*>(&buffer[7]), 32768);
}
