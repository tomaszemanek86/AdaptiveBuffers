#include <utest/utest.h>
#include "struct_with_size_reference.h"

using namespace struct_with_size_reference;

UTEST_MAIN();

UTEST(struct_with_size_reference, serde) {
    MainSer main_ser;
    main_ser.with_volume().with_w(10);
    main_ser.with_volume().with_h(20);
    main_ser.with_volume().with_l(30);
    auto buffer = main_ser.serialize();

    MainDe main_de(buffer.data());
    ASSERT_EQ(main_de.volume_size(), 7);
    ASSERT_EQ(main_de.volume().w(), 10);
    ASSERT_EQ(main_de.volume().h(), 20);
    ASSERT_EQ(main_de.volume().l(), 30);
}
