mod cpp_generated_data_test;
mod cpp_generated_deserializers_test;
mod cpp_generated_serializers_test;
mod memory;

use super::*;

static TEST_DIR: &str = "test_cpp_generator";

fn delete_test_dir_if_exists() {
    // check if directory exists
    if !std::path::Path::new(TEST_DIR).exists() {
        return;
    }
    let _ = std::fs::remove_dir_all(TEST_DIR);
}

fn create_test_dir() {
    let _ = std::fs::create_dir(TEST_DIR);
}

fn test_dirs_init() {
    delete_test_dir_if_exists();
    create_test_dir();
}

fn read_data() -> String {
    std::fs::read_to_string(format!("{}/data.h", TEST_DIR)).unwrap()
}

fn read_serializers() -> String {
    std::fs::read_to_string(format!("{}/serializers.h", TEST_DIR)).unwrap()
}

fn read_deserializers() -> String {
    std::fs::read_to_string(format!("{}/deserializers.h", TEST_DIR)).unwrap()
}
