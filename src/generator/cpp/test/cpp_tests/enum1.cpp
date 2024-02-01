#include <utest/utest.h>
#include "enum1.h"

using namespace enum1;

UTEST_MAIN();

UTEST(enum1, serde) {
    uint8_t buffer[1024];
    CarSer car_ser;
    
    car_ser.set_data(Car::Audi);
    car_ser.serialize(buffer);

    CarDe car_de(buffer);
    ASSERT_EQ(car_de.get_data(), Car::Audi);
}
