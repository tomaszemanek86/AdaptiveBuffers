#include <utest/utest.h>
#include "struct_with_views.h"

UTEST_MAIN();

UTEST(struct_with_views, serde) {
    uint8_t buffer[1024];
    NumberSeqSer number_seq_ser;

    number_seq_ser.with_num0().with_u8(1);
    number_seq_ser.with_num1().with_u16(20000);
    number_seq_ser.with_num2().with_u32(3000000);
    number_seq_ser.serialize(buffer);

    NumberSeqDe number_seq_de(buffer);
    ASSERT_EQ(number_seq_de.num0().u8(), 1);
    ASSERT_EXCEPTION(number_seq_de.num2(), std::runtime_error);

    ASSERT_EQ(number_seq_de.num1().u16(), 20000);
    //ASSERT_EQ(number_seq_de.num2().u32(), 3000000);
}
