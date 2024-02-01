#include <utest/utest.h>
#include "view_with_natives.h"

using namespace view_with_natives;

UTEST_MAIN();

UTEST(view_with_natives, serde) {
    uint8_t buffer[1024];
    NumbersSer numbers_ser;
    numbers_ser.with_u16(60000);
    numbers_ser.serialize(buffer);

    NumbersDe numbers_de(buffer);
    ASSERT_EQ(numbers_de.u16(), 60000);
    
    ASSERT_EXCEPTION(numbers_de.u8(), std::runtime_error);
    ASSERT_EXCEPTION(numbers_de.u32(), std::runtime_error);
    numbers_de.u16();
}
