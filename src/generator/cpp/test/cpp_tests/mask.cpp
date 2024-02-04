#include <utest/utest.h>
#include "mask.h"
#include <iostream>
using namespace mask;

UTEST_MAIN();

UTEST(mask, set_get_bit) {
    auto value = abf::set_u8_bit((uint8_t)0, 5, true);
    ASSERT_EQ(abf::get_u8_bit(5), value);
}

UTEST(mask, serde_1) {
    uint8_t buffer[1024];
    ColorMaskSer mask_ser;
    mask_ser.with_blue(true)
            .with_green(true)
            .with_red();
    auto size = mask_ser.serialize(buffer);
    ASSERT_EQ(size, 1);
    ColorMaskDe mask_de(buffer);
    ASSERT_FALSE(mask_de.blue()); // reds puts blue down
    ASSERT_TRUE(mask_de.green());
    ASSERT_TRUE(mask_de.red()); // set red mas set on white
    ASSERT_TRUE(mask_de.white());
}

UTEST(mask, serde_2) {
    uint8_t buffer[1024];
    ColorMaskSer mask_ser;
    mask_ser.with_white();
    auto size = mask_ser.serialize(buffer);
    ASSERT_EQ(size, 1);
    ColorMaskDe mask_de(buffer);
    ASSERT_TRUE(mask_de.white()); // reds puts blue down
}
