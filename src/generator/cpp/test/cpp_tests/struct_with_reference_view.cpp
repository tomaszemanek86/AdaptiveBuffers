#include <utest/utest.h>
#include "struct_with_reference_view.h"

using namespace struct_with_reference_view;

UTEST_MAIN();

UTEST(struct_with_reference_view, serde) {
    uint8_t buffer[1024];
    KnownNumberSer known_number_ser;
    known_number_ser.with_number().with_u16(12500);
    known_number_ser.serialize(buffer);

    KnownNumberDe known_number_de(buffer);
    ASSERT_EQ(known_number_de.number_key(), 1);
    ASSERT_EQ(known_number_de.number().u16(), 12500);
}
