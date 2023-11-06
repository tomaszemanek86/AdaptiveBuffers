use super::*;

#[test]
fn generate_cpp_enum_deserializer_1() {
    test_dirs_init();

    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
        "
    enum Test : u32 {
        Value1 = 10,
        Value2 = 20
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

    let deserializer_h = read_deserializers();
}

#[test]
fn generate_cpp_view_deserializer_1() {
    test_dirs_init();

    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
        "
        view SmallNumbers {
            u8, u16
        }
        view Numbers {
            SmallNumbers, u64
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

    let deserializer_h = read_deserializers();
}

#[test]
fn generate_cpp_struct_deserializer_1() {
    test_dirs_init();

    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
        "
        view SmallNumbers {
            u8, u16
        }
        struct Data {
            u8_value: u8,
            small_numbers: SmallNumbers,
            u16_value_1: u16,
            u16_value_2: u16
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

    let deserializer_h = read_deserializers();
}

#[test]
fn generate_cpp_struct_deserializer_2() {
    test_dirs_init();

    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
        "
        view SmallNumbers {
            u8 = 80, u16 = 160
        }
        struct Data {
            small_numbers_key: u8 = small_numbers.key,
            small_numbers: SmallNumbers
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

    let deserializer_h = read_deserializers();
}
