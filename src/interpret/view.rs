use super::*;

impl View {
    pub fn resolve_unknown_types(&mut self, types: &Types) -> Result<(), InterpretError> {
        for t in &mut self.types {
            if t.typ.typ.is_unknown() {
                let unknown = t.typ.typ.as_unknown().unwrap();
                let resolved = types.get_type(&unknown)?;
                if let Some(resolved) = resolved {
                    t.typ.typ = resolved.clone();
                } else {
                    return Err(InterpretError::UnknownType(unknown.clone()));
                }
            }
        }
        Ok(())
    }

    pub fn check_type(&mut self, types: &Types) -> Result<(), InterpretError> {
        for view_posibility in &self.types {
            // find count of type in self.types
            let count_of_type = self
                .types
                .iter()
                .filter(|t| view_posibility.typ.array_size == t.typ.array_size && match &view_posibility.typ.typ {
                    TypeVariant::Int(i) => {
                        t.typ.typ.is_int()
                            && t.typ.typ.as_int().unwrap().bytes == i.bytes
                            && t.typ.typ.as_int().unwrap().signed == i.signed
                    }
                    TypeVariant::Struct(s) => {
                        t.typ.typ.is_struct() && t.typ.typ.as_struct().unwrap().borrow().name() == s.borrow().name()
                    }
                    TypeVariant::View(v) => {
                        t.typ.typ.is_view() && t.typ.typ.as_view().unwrap().borrow().name == v.borrow().name
                    }
                    TypeVariant::Enum(e) => t.typ.typ.is_enum() && t.typ.typ.as_enum().unwrap().name == e.name,
                    TypeVariant::BitMask(b) => t.typ.typ.is_enum() && t.typ.typ.as_bit_mask().unwrap().name == b.name,
                    TypeVariant::Unknown(_) => panic!("cannot check type for unknown"),
                })
                .count();
            if count_of_type != 1 {
                return Err(InterpretError::ViewItemNotUniqueWithinView(view_posibility.typ.typ.code_view()));
            }
        }
        for t in &mut self.types {
            t.typ.typ.check_type(types)?;
        }
        if self.types.len() == 0 {
            return Err(InterpretError::ViewEmpty(self.name.clone()));
        }
        if !self.types
            .iter()
            .all(|t| t.constant.is_none()) &&
            !self.types
            .iter()
            .all(|t| t.constant.is_some() && 
                t.constant
                    .as_ref()
                    .unwrap()
                    .is_enum_member_ref()) &&
            !self.types
            .iter()
            .all(|t| t.constant.is_some() && 
                t.constant
                    .as_ref()
                    .unwrap()
                    .is_usize()) {
                return Err(InterpretError::VievConstantsMustBeAllEnumsOrAllIntsOrAllUndefined)
            }
        Ok(())
    }
    pub fn has_known_types(&self, known_types: &Vec<String>) -> bool {
        self.types
            .iter()
            .all(|ti| match &ti.typ.typ {
                TypeVariant::Struct(t) => known_types.contains(&t.borrow().name()),
                TypeVariant::Enum(_) => true,
                TypeVariant::View(t) => known_types.contains(&t.borrow().name),
                TypeVariant::Int(_) => true,
                TypeVariant::BitMask(_) => true,
                TypeVariant::Unknown(ref _u) => panic!("unexpected unknown type"),
            })
    }
}
