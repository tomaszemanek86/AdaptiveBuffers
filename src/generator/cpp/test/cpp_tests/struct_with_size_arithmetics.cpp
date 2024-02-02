#include <utest/utest.h>
#include "struct_with_size_arithmetics.h"

using namespace struct_with_size_arithmetics;

UTEST_MAIN();

UTEST(struct_with_size_arithmetics, serde) {
    uint8_t buffer[1024];
    ABSizeSer absize_ser;
    absize_ser.with_a(5);
    absize_ser.with_b(6);
    absize_ser.with_c().get(0).set_data(180);
    absize_ser.with_c().get(1).set_data(170);
    absize_ser.with_c().get(2).set_data(160);
    absize_ser.with_c().get(3).set_data(150);
    auto size = absize_ser.serialize(buffer);
    ASSERT_EQ(size, 20);

    ABSizeDe absize_de(buffer);
    ASSERT_EQ(absize_de.a(), 5);
    ASSERT_EQ(absize_de.b(), 6);
    ASSERT_EQ(absize_de.c().get(0).get_data(), 180);
    ASSERT_EQ(absize_de.c().get(1).get_data(), 170);
    ASSERT_EQ(absize_de.c().get(2).get_data(), 160);
    ASSERT_EQ(absize_de.c().get(3).get_data(), 150);
    ASSERT_EQ(absize_de.size(), 17 + 100 - 50 + 6/*b.value*/);
}
