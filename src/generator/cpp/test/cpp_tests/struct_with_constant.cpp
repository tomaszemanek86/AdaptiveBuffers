#include <utest/utest.h>
#include "struct_with_constant.h"

using namespace struct_with_constant;

UTEST_MAIN();

UTEST(struct_with_constant, serde) {
    uint8_t buffer[1024];
    MessageSer msg_ser;
    msg_ser.with_value(288);
    ASSERT_EQ(msg_ser.serialize(buffer), 4);
    MessageDe msg_de(buffer);
    ASSERT_EQ(msg_de.byte0(), 224);
    ASSERT_EQ(msg_de.byte1(), 3);
    ASSERT_EQ(msg_de.value(), 288);
}
