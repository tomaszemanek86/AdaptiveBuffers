use super::*;

mod cpp;
mod utils;
mod to_string;

pub enum GeneratorError {
    InternalError(String),
}

pub struct Writer {
    begin_spaces: usize,
    buffer: String,
    filename: String,
}

pub fn generate(memory: &Vec<MemoryDeclaration>, args: &Args) -> Result<(), GeneratorError> {
    match args.language {
        Language::Cpp => cpp::generate(memory, args),
        _ => {
            return Err(GeneratorError::InternalError(format!(
                "Language {} not supported",
                args.language.to_string()
            )))
        }
    }
    Ok(())
}
