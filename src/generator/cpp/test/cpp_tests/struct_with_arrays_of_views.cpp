#include <utest/utest.h>
#include "struct_with_arrays_of_views.h"

UTEST_MAIN();

UTEST(struct_with_arrays_of_natives, serde) {
    uint8_t buffer[1024];

    NumbersSeqSer numbers_ser;
    numbers_ser.with_num0().get(0).with_u8(1);
    numbers_ser.with_num0().get(1).with_u16(2);
    numbers_ser.with_num1().get(0).with_u32(3);
    numbers_ser.with_num1().get(1).with_u8(4);
    numbers_ser.with_num2().get(0).with_u16(5);
    numbers_ser.with_num2().get(1).with_u32(6);
    numbers_ser.serialize(buffer);

    NumbersSeqDe numbers_de(buffer);
    ASSERT_EQ(numbers_de.num0().get(0).u8(), 1);
    ASSERT_EQ(numbers_de.num0().get(1).u16(), 2);
    ASSERT_EQ(numbers_de.num1().get(0).u32(), 3);
    ASSERT_EQ(numbers_de.num1().get(1).u8(), 4);
    ASSERT_EQ(numbers_de.num2().get(0).u16(), 5);
    ASSERT_EQ(numbers_de.num2().get(1).u32(), 6);
}
