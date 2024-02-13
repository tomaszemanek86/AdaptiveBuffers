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
    ASSERT_EQ(*reinterpret_cast<uint16_t*>(&buffer[3]), 3);
    ASSERT_EQ(*reinterpret_cast<uint16_t*>(&buffer[5]), 3);
    ASSERT_EQ(*reinterpret_cast<uint16_t*>(&buffer[7]), 49152);

    DifferentEndiansDe de(buffer);
    ASSERT_EQ(de.member_16(), 3);
    ASSERT_EQ(de.member_16b(), 3);
    ASSERT_EQ(de.member_16l(), 3);
    ASSERT_EQ(de.member_8(), 1);
    ASSERT_EQ(de.member_8b(), 1);
    ASSERT_EQ(de.member_8l(), 1);
}
