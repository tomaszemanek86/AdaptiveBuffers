#include <utest/utest.h>
#include "struct_with_sized_array_of_natives.h"

UTEST_MAIN();

UTEST(struct_with_natives, serde) {
    uint8_t buffer[1024];
    DimensionSer dim_ser;
    
    dim_ser.with_whl().get(0).set_data(57);
    dim_ser.with_whl().get(1).set_data(58);
    dim_ser.serialize(buffer);

    DimensionDe dim_de(buffer);
    ASSERT_EQ(dim_de.whl().get(0).get_data(), 57);
    ASSERT_EQ(dim_de.whl().get(1).get_data(), 58);
    ASSERT_EXCEPTION(dim_de.whl().get(2), std::runtime_error);
}
