#include <utest/utest.h>
#include "mask.h"

using namespace mask;

UTEST_MAIN();

UTEST(mask, serde) {
    uint8_t buffer[1024];
    MaskSer mask_ser;
    absize_ser.with_white();
    absize_ser.with_orange();
    absize_ser.with_gray();
    auto size = mask_ser.serialize(buffer);
    ASSERT_EQ(size, 1);

    MaskDe mask_de(buffer);
    ASSERT_TRUE(mask_de.white());
    ASSERT_TRUE(mask_de.orange());
    ASSERT_TRUE(mask_de.gray());
    ASSERT_TRUE(mask_de.green()); // mask orrange consist also of green
}
