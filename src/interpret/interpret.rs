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
            parser::SyntaxToken::Endian(e) => self.big_endian = Some(e.data.big),
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
}
