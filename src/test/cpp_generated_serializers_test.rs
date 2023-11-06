use super::*;

#[test]
fn generate_cpp_enum_serializer() {
    test_dirs_init();

    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from("
    enum Car : u8 {
        Skoda = 1,
        Volvo = 2,
        Porshe = 3
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

    let serializer_h = read_serializers();
    assert!(serializer_h.contains("#include \"data.h\""));
    assert!(serializer_h.contains("#include \"serializer_base.h\""));
    assert!(serializer_h.contains("class CarSerializer {"));
    assert!(serializer_h.contains("public:"));
    assert!(serializer_h.contains("CarSerializer(void* buffer);"));
    assert!(serializer_h.contains("void with_car(Car value);"));
    assert!(serializer_h.contains("uint16_t serialize(void* buffer);"));
    assert!(serializer_h.contains("private:"));
    assert!(serializer_h.contains("void* _buffer;"));
}

#[test]
fn generate_cpp_viewserializer() {
    test_dirs_init();

    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
    "
    view SmallNumbers {
        u8, u16
    }

    view BigNumbers {
        u32, u64
    }

    view Numbers {
        SmallNumbers, BigNumbers
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

    let serializer_h = read_serializers();
    assert!(serializer_h.contains("#include \"data.h\""));
    assert!(serializer_h.contains("#include \"serializer_base.h\""));
    assert!(serializer_h.contains("class SmallNumbersSerializer {"));
    assert!(serializer_h.contains("class BigNumbersSerializer {"));
    assert!(serializer_h.contains("class NumbersSerializer {"));
    assert!(serializer_h.contains("public:"));
    assert!(serializer_h.contains("private:"));
    assert!(serializer_h.contains("SmallNumbersSerializer(void* buffer);"));
    assert!(serializer_h.contains("BigNumbersSerializer(void* buffer);"));
    assert!(serializer_h.contains("NumbersSerializer(void* buffer);"));
    assert!(serializer_h.contains("void with_u8(uint8_t value);"));
    assert!(serializer_h.contains("void with_u16(uint16_t value);"));
    assert!(serializer_h.contains("void with_u32(uint32_t value);"));
    assert!(serializer_h.contains("void with_u64(uint64_t value);"));
    assert!(serializer_h.contains("void* _buffer = nullptr;"));
}

#[test]
fn generate_cpp_struct_serializer() {
    test_dirs_init();

    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
    "
    view SmallNumbers {
        u8, u16
    }

    view BigNumbers {
        u32, u64
    }

    view Numbers {
        SmallNumbers, BigNumbers
    }
    
    struct Volume {
        width: u16,
        height: u16,
        length: u16,
        numbers_key: u8 = numbers.key,
        numbers: Numbers
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
    
    let serializer_h = read_serializers();
    assert!(serializer_h.contains("class VolumeSerializer {"));
    assert!(serializer_h.contains("VolumeSerializer(void* buffer);"));
    assert!(serializer_h.contains("void with_width(uint16_t value);"));
    assert!(serializer_h.contains("void with_height(uint16_t value);"));
    assert!(serializer_h.contains("void with_length(uint16_t value);"));
    assert!(serializer_h.contains("NumbersSerializer with_numbers();"));
}

#[test]
fn generate_cpp_complex_struct_serializer() {
    test_dirs_init();

    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
        "
    enum Command : u8 {
        DoA = 20,
        DoB = 30
    }

    view Numbers {
        u8, u16
    }

    struct StructA {
        numbers_id: u8 = numbers.key,
        numbers: Numbers,
        u8_array_size: u8 = u8_array.dimension,
        u8_array: [u8, 10max]
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

    let serializer_h = read_serializers();
}


#[test]
fn generate_cpp_struct_with_array_serializer() {
    test_dirs_init();

    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
        "
    struct StructA {
        values_dimension: u16 = values.dimension,
        values: [u8]
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
}