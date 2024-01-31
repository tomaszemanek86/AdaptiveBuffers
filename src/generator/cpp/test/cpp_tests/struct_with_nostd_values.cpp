#include <utest/utest.h>
#include "struct_with_nostd_values.h"

using namespace struct_with_nostd_values;

UTEST_MAIN();

UTEST(struct_with_nostd_values, serde) {
    uint8_t buffer[1024];
    NoStdValuesSer nostd_ser;
    nostd_ser.with_a(120951);
    ASSERT_EQ(nostd_ser.serialize(buffer), 6);
    NoStdValuesDe nostd_de(buffer);
    ASSERT_EQ(nostd_de.a(), 120951);
    ASSERT_EQ(nostd_de.b(), 1234);
}
