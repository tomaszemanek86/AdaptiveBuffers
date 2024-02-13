use super::*;

mod cpp;
mod utils;
mod to_string;

pub enum GeneratorError {
    InternalError(String),
}

pub fn generate(mi: MemoryImage, args: &Args) -> Result<(), GeneratorError> {
    let big_endian_on_machine = match args.endian.as_str() {
        "big" => true,
        "little" => false,
        _ => panic!("endian can be big or little")
    };
    match args.language {
        Language::Cpp => cpp::generate(&mi.memory_decl, &EndianSettings {
            protocol_big: mi.big_endian,
            machine_big: big_endian_on_machine
        }, args),
        _ => {
            return Err(GeneratorError::InternalError(format!(
                "Language {} not supported",
                args.language.to_string()
            )))
        }
    }
    Ok(())
}
