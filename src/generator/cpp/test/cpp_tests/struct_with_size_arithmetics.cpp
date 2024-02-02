#include <utest/utest.h>
#include "struct_with_size_arithmetics.h"

using namespace struct_with_size_arithmetics;

UTEST_MAIN();

UTEST(struct_with_size_arithmetics, serde) {
    uint8_t buffer[1024];
    ABSizeSer absize_ser;
    absize_ser.with_a(5);
    absize_ser.with_b(6);
    auto size = absize_ser.serialize(buffer);
    ASSERT_EQ(size, 4);

    ABSizeDe absize_de(buffer);
    ASSERT_EQ(absize_de.a(), 5);
    ASSERT_EQ(absize_de.b(), 6);
    ASSERT_EQ(absize_de.size(), 3);
}
