use std::ops::Deref;

use super::*;

impl Struct {
    pub fn create(s: parser::Struct) -> Result<Rc<Struct>, InterpretError> {
        // Check if member is self
        if s.members.iter().any(|member| if let parser::Typ::UserDefined(user_defined) = member.typ {
            user_defined == s.name
        } else {
            false
        }) {
            return Err(InterpretError::StructureContainsItself(s))
        }
        // create structure
        Ok(Rc::new(Struct { members: s.members.iter().map(|member| {
            return Member { name: member.name.clone(), typ: match &member.typ {
                parser::Typ::Int(int) => Cell::new(Type::Int(int.clone())),
                parser::Typ::UserDefined(ud) => Cell::new(Type::UserDefined(ud.clone())),
                _ => unimplemented!()
            } }
        }).collect() }))
    }
}

impl Structs {
    pub fn put_struct(&mut self, s: parser::Struct) -> Result<(), InterpretError> {
        if self.structs.iter().any(|it| *it.0 == s.name) {
            return Err(InterpretError::StructureRedefined(s))
        }
        self.structs.insert(s.name.clone(), Struct::create(s)?);
        Ok(())
    }
    pub fn validate_user_defined_types(&mut self) -> Result<(), InterpretError> {
        for s1 in &self.structs {
            for s1_member in &s1.1.members {
                let mut typ = s1_member.typ.into_inner();
                typ = if let Type::UserDefined(ud) = typ {
                    if let Some(ref_sruct) = self.structs.iter().find(|it| it.0 == *s1_member.typ.get_mut()) {

                    } else {
                        return Err(InterpretError::UnknownType(ud))
                    }
                } else {
                    typ
                }
                
            }
            for s2 in &self.structs {

            }
        }
        todo!()
    }
}