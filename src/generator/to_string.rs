use super::*;

impl ToString for GeneratorError {
    fn to_string(&self) -> String {
        match self {
            GeneratorError::InternalError(e) => format!("Internal error: {}", e),
        }
    }
}
