use super::*;

#[test]
fn generate_cpp_struct_1() {
    test_dirs_init();

    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
        "
    struct Test {
        packet_size: u8 = $size,
        member_u8: u8,
        member_u16: u16
    }",
    );

    generate_cpp(
        generate_memory(source),
        &Args {
            protofile: String::from(""),
            language: Language::Cpp,
            output_dir: String::from(TEST_DIR),
        },
    );

    let data_h = read_data();
    assert!(data_h.contains("#pragma once"));
    assert!(data_h.contains("stdint.h"));
    assert!(data_h.contains("struct Test {"));
    assert!(data_h.contains("uint8_t member_u8;"));
    assert!(data_h.contains("uint16_t member_u16;"));
    assert!(data_h.contains("};"));
}

#[test]
fn generate_enum_1() {
    test_dirs_init();

    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
        "
        enum TestEnum: u8 {
            TestValue1 = 100,
            TestValue2 = 200
        }
    ",
    );

    generate_cpp(
        generate_memory(source),
        &Args {
            protofile: String::from(""),
            language: Language::Cpp,
            output_dir: String::from(TEST_DIR),
        },
    );

    let data_h = read_data();
    assert!(data_h.contains("#pragma once"));
    assert!(data_h.contains("stdint.h"));
    assert!(data_h.contains("enum class TestEnum : uint8_t {"));
    assert!(data_h.contains("TestValue1 = 100;"));
    assert!(data_h.contains("TestValue2 = 200;"));
    assert!(data_h.contains("};"));
}

#[test]
fn generate_view_1() {
    test_dirs_init();

    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
        "
        view TestView {
            u8, u16
        }
    ",
    );

    generate_cpp(
        generate_memory(source),
        &Args {
            protofile: String::from(""),
            language: Language::Cpp,
            output_dir: String::from(TEST_DIR),
        },
    );

    let data_h = read_data();
    assert!(data_h.contains("#pragma once"));
    assert!(data_h.contains("stdint.h"));
    assert!(data_h.contains("union TestView {"));
    assert!(data_h.contains("uint8_t u8;"));
    assert!(data_h.contains("uint16_t u16;"));
}
