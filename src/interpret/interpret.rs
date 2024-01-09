use super::*;

impl Interpreter {
    pub fn put_token(&mut self, token: parser::SyntaxToken) -> Result<(), InterpretError> {
        match token {
            parser::SyntaxToken::Struct(t) => self.types.put_struct(t)?,
            parser::SyntaxToken::View(t) => self.types.put_view(t)?,
            parser::SyntaxToken::Enum(t) => self.types.put_enum(t)?,
            parser::SyntaxToken::RequiredVersion(v) => {
                self.required_version = Some([
                    v.data.version[0].value.unwrap(),
                    v.data.version[1].value.unwrap(),
                    v.data.version[2].value.unwrap(),
                ])
            }
            parser::SyntaxToken::Endian(e) => {
                if self.big_endian.is_some() {
                     return Err(
                        InterpretError::EndianOverrided(
                            self.big_endian.as_ref().unwrap().code_view.clone(),
                            e.code_view.clone()
                        )
                    )
                }
                self.big_endian = Some(e.convert(|b| b.big));
            },
        }
        Ok(())
    }

    pub fn interpret(mut self, tokens: Vec<parser::SyntaxToken>) -> Result<Self, InterpretError> {
        for te in tokens {
            self.put_token(te)?;
        }
        self.types = self.types.resolve_unknown_types()?;
        self.order = self.types.resolve_types_order()?;
        self.types.check_types(&self.types)?;
        Ok(self)
    }

    pub fn get_memory(&self) -> Result<Vec<MemoryDeclaration>, InterpretError> {
        let mut memory = Vec::new();
        for name in &self.order {
            let t = self.types.get_type(name)?;
            let m = t.unwrap().as_memory(&memory)?;
            memory.push(MemoryDeclaration {
                name: name.clone(),
                memory: m,
            });
        } 
        Ok(memory)
    }

    pub fn big_endian(&self) -> Result<bool, InterpretError> {
        match &self.big_endian {
            Some(b) => Ok(b.data),
            None => Err(InterpretError::EndianNotSet)
        }
    }
}
