use super::*;

impl Types {
    pub fn put_struct(&mut self, typ: DataView<parser::Struct>) -> Result<(), InterpretError> {
        self.types.insert(typ.name.data.clone(), TypeVariant::from_struct(typ)?);
        Ok(())
    }
    pub fn put_enum(&mut self, typ: DataView<parser::Enum>) -> Result<(), InterpretError> {
        if self.types.contains_key(typ.name.as_str()) {
            return Err(InterpretError::EnumAlreadyExists(typ));
        }
        self.types.insert(typ.name.clone(), TypeVariant::from_enum(typ)?);
        Ok(())
    }
    pub fn put_view(&mut self, typ: DataView<parser::View>) -> Result<(), InterpretError> {
        if self.types.contains_key(typ.name.as_str()) {
            return Err(InterpretError::ViewAlreadyExists(typ.code_view()));
        }
        self.types.insert(typ.name.clone(), TypeVariant::from_view(typ)?);
        Ok(())
    }
    pub fn put_bit_mask(&mut self, typ: DataView<BitMask>) -> Result<(), InterpretError> {
        if self.types.contains_key(typ.name.as_str()) {
            return Err(InterpretError::BitMaskAlreadyExists(typ.name.clone(), typ.code_view()));
        }
        self.types.insert(typ.name.clone(), TypeVariant::from_bit_mask(typ)?);
        Ok(())
    }
    pub fn resolve_unknown_types(self) -> Result<Self, InterpretError> {
        for (_name, t) in &self.types {
            t.resolve_unknown_types(&self)?
        }
        Ok(self)
    }
    pub fn resolve_types_order(&self) -> Result<Vec<String>, InterpretError> {
        let mut independent = Vec::<String>::default();
        let mut dependent = Vec::<String>::default();
        // first make all independet types - enums, types that consist only from native types
        for (k, typ) in &self.types {
            if typ.has_known_types(&independent) {
                independent.push(k.clone())
            } else {
                dependent.push(k.clone())
            }
        }
        let mut dep_len = dependent.len();
        while !dependent.is_empty() {
            for i in 0..dependent.len() {
                if self
                    .get_type(dependent[i].as_str())?
                    .unwrap()
                    .has_known_types(&independent)
                {
                    independent.push(dependent.remove(i));
                    break; // It can be removed only one element in cycle, then iterator is invalid
                }
            }
            if dep_len == dependent.len() {
                return Err(InterpretError::CyclicalReference(dependent));
            }
            dep_len = dependent.len();
        }
        Ok(independent)
    }
    pub fn check_types(&self, types: &Types) -> Result<(), InterpretError> {
        for (_name, typ) in &self.types {
            let _ = typ.check_type(types)?;
        }
        Ok(())
    }
}
