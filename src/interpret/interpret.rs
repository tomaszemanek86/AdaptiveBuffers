use super::*;

impl Interpreter {
    pub fn put_token(&mut self, token: parser::SyntaxToken) -> Result<(), InterpretError> {
        match token {
            parser::SyntaxToken::Struct(structure) => self.structs.put_struct(structure)?,
            _ => todo!()
        }
        Ok(())
    }

    pub fn get_types(&mut self) -> Result<Vec<Type>, InterpretError> {
        let mut types = Vec::<Type>::default();
        let _ = self.structs.validate_user_defined_types()?;
        Ok(types)
    }
}