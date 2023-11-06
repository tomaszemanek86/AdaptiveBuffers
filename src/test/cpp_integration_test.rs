use super::*;

#[test]
fn integration_test() {
    test_dirs_init();

    let _ = simple_logger::SimpleLogger::new().init();
    let source = String::from(
        "
        enum Command : u8 {
            Clean = 10,
            Go = 20
        }
        
        struct Cleaning {
            dollars_per_hour: u8,
            shift_hours: u8
        }
        
        struct Go {
            km_per_sec: u8
        }
        
        view Tasks {
            Cleaning = Command::Clean,
            Go = Command::Go
        }
        
        view SmallNumbers {
            u8, u16
        }
        
        view BigNumbers {
            u8, u16
        }
        
        view Numbers {
            SmallNumbers, BigNumbers
        }
        
        struct BigData {
            command: Command,
            u8_value: u8,
            numbers: Numbers,
            small_numbbers_key: u8 = small_numbers.key,
            u16_value_1: u16,
            small_numbers: SmallNumbers,
            u16_value_2: u16
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