#include <utest/utest.h>
#include "mask_u24.h"
#include <iostream>

using namespace mask_u24;

UTEST_MAIN();

UTEST(mask_u24, serde_second_0) {
    uint8_t buffer[1024];
    Mask24Ser mask_ser;
    mask_ser.with_second_0(true);
    auto size = mask_ser.serialize(buffer);
    ASSERT_EQ(size, 3);

    std::cout << "buffer[0] = " << std::to_string(buffer[0]) << std::endl;
    std::cout << "buffer[1] = " << std::to_string(buffer[1]) << std::endl;
    std::cout << "buffer[2] = " << std::to_string(buffer[2]) << std::endl;
    std::cout << "buffer[3] = " << std::to_string(buffer[3]) << std::endl;

    Mask24De mask_de(buffer);
    ASSERT_TRUE(mask_de.second_0());
}

UTEST(mask_u24, serde_second_1) {
    uint8_t buffer[1024];
    Mask24Ser mask_ser;
    mask_ser.with_second_1(true);
    auto size = mask_ser.serialize(buffer);
    ASSERT_EQ(size, 3);

    std::cout << "buffer[0] = " << std::to_string(buffer[0]) << std::endl;
    std::cout << "buffer[1] = " << std::to_string(buffer[1]) << std::endl;
    std::cout << "buffer[2] = " << std::to_string(buffer[2]) << std::endl;
    std::cout << "buffer[3] = " << std::to_string(buffer[3]) << std::endl;

    Mask24De mask_de(buffer);
    ASSERT_TRUE(mask_de.second_1());
}

UTEST(mask_u24, serde_second_2) {
    uint8_t buffer[1024];
    Mask24Ser mask_ser;
    mask_ser.with_second_2(true);
    auto size = mask_ser.serialize(buffer);
    ASSERT_EQ(size, 3);

    std::cout << "buffer[0] = " << std::to_string(buffer[0]) << std::endl;
    std::cout << "buffer[1] = " << std::to_string(buffer[1]) << std::endl;
    std::cout << "buffer[2] = " << std::to_string(buffer[2]) << std::endl;
    std::cout << "buffer[3] = " << std::to_string(buffer[3]) << std::endl;

    Mask24De mask_de(buffer);
    ASSERT_TRUE(mask_de.second_2());
}

UTEST(mask_u24, serde_firsts) {
    uint8_t buffer[1024];
    Mask24Ser mask_ser;
    mask_ser.with_firsts();
    auto size = mask_ser.serialize(buffer);
    ASSERT_EQ(size, 3);

    std::cout << "buffer[0] = " << std::to_string(buffer[0]) << std::endl;
    std::cout << "buffer[1] = " << std::to_string(buffer[1]) << std::endl;
    std::cout << "buffer[2] = " << std::to_string(buffer[2]) << std::endl;
    std::cout << "buffer[3] = " << std::to_string(buffer[3]) << std::endl;

    Mask24De mask_de(buffer);
    ASSERT_TRUE(mask_de.firsts());
}

UTEST(mask, serde) {
    uint8_t buffer[1024];
    Mask24Ser mask_ser;
    mask_ser.with_firsts()
            .with_second_0(true)
            .with_second_1(true)
            .with_second_2(true);
    auto size = mask_ser.serialize(buffer);
    ASSERT_EQ(size, 3);

    std::cout << "buffer[0] = " << std::to_string(buffer[0]) << std::endl;
    std::cout << "buffer[1] = " << std::to_string(buffer[1]) << std::endl;
    std::cout << "buffer[2] = " << std::to_string(buffer[2]) << std::endl;
    std::cout << "buffer[3] = " << std::to_string(buffer[3]) << std::endl;

    Mask24De mask_de(buffer);
    ASSERT_TRUE(mask_de.firsts());
    ASSERT_TRUE(mask_de.second_0());
    ASSERT_TRUE(mask_de.second_1());
    ASSERT_TRUE(mask_de.second_2());
}
