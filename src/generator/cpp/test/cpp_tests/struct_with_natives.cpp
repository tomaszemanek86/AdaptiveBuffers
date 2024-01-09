#include <utest/utest.h>
#include "struct_with_natives.h"

UTEST_MAIN();

UTEST(struct_with_natives, serde) {
    uint8_t buffer[1024];
    DateSer xmass_ser;
    xmass_ser.with_year(2023);
    xmass_ser.with_month(12);
    xmass_ser.with_day(24);
    xmass_ser.serialize(buffer);
    DateDe xmass_de(buffer);
    ASSERT_EQ(xmass_de.day(), 24);
    ASSERT_EQ(xmass_de.month(), 12);
    ASSERT_EQ(xmass_de.year(), 2023);
}
