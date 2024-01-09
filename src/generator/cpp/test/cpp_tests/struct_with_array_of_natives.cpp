#include <utest/utest.h>
#include "struct_with_array_of_natives.h"

UTEST_MAIN();

UTEST(struct_with_natives, serde) {
    uint8_t buffer[1024];
    DimensionSer dim_ser;
    
    dim_ser.with_whl().get(0).set_data(10);
    dim_ser.with_whl().get(1).set_data(20);
    dim_ser.with_whl().get(2).set_data(30);
    dim_ser.serialize(buffer);

    DimensionDe dim_de(buffer);
    ASSERT_EQ(dim_de.whl().get(0).get_data(), 10);
    ASSERT_EQ(dim_de.whl().get(1).get_data(), 20);
    ASSERT_EQ(dim_de.whl().get(2).get_data(), 30);
}
