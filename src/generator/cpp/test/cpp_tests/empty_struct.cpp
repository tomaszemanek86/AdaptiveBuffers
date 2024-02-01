#include <utest/utest.h>
#include "empty_struct.h"

using namespace empty_struct;

UTEST_MAIN();

UTEST(empty_struct, serde) {
    uint8_t buffer[1024];

    EmptySer empty_ser;
    ASSERT_EQ(empty_ser.size(), 0);

    EmptyDe empty_de(buffer);
    ASSERT_TRUE(empty_de._deserialized());
}
