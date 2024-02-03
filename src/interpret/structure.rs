use core::panic;

use self::parser::SizeArithmetics;

use super::*;

impl Struct {
    pub fn check_type(&self, types: &Types) -> Result<(), InterpretError> {
        self.check_unique_member_names()?;
        self.check_references(types)?;
        Ok(())
    }
    fn check_unique_member_names(&self) -> Result<(), InterpretError> {
        for member in &self.members {
            if self
                .members
                .iter()
                .filter(|m| m.name.data == member.name.data)
                .count()
                != 1
            {
                return Err(InterpretError::StructMemberNotUnique);
            }
        }
        Ok(())
    }
    fn check_references(&self, types: &Types) -> Result<(), InterpretError> {
        for member in &self.members {
            if let Some(c) = &member.constant {
                if !member.typ.typ.is_int() {
                    return Err(InterpretError::StructMemberConstantCanBeApliedOnlyForInt(
                        member.name.code_view(),
                    ));
                }
                match c {
                    StructMemberConstant::Usize(_value) => continue,
                    StructMemberConstant::ViewReferenceKey(mr) => {
                        if let Some(t) = types.get_type(&mr.member_name.data)? {
                            if let Some(i) = self.get_member_index_by_name(&mr.member_name.data) {
                                if !self.members[i].typ.typ.is_view() {
                                    return Err(InterpretError::MemberReferenceDoesntPointToView(mr.member_name.code_view()))
                                }
                            } else {
                                return Err(InterpretError::UnknownStructMemberReference(mr.member_name.code_view()));
                            }
                            if let Some(view) = t.as_view() {
                                // find max value of view constant
                                let mut max_value = 0;
                                for t in &view.borrow().types {
                                    if let Some(c) = &t.constant {
                                        match c {
                                            parser::ViewConstantValue::Usize(value) => 
                                                max_value = max_value.max(value.data),
                                            parser::ViewConstantValue::EnumMemberRef(e) => 
                                                max_value = max_value.max(
                                                    types.get_enum_member_value(&e.enum_name, &e.enum_member)?),
                                        }
                                    }
                                }
                                if member.typ.typ.as_int().unwrap().max_value() < max_value {
                                    return Err(InterpretError::ViewReferenceTypeTooSmall(
                                        member.name.code_view(),
                                    ));
                                }
                            } else {
                                return Err(InterpretError::UnknownStructMemberReference(mr.member_name.code_view()));
                            }
                        }
                    },
                    StructMemberConstant::ArrayDimension(mr) => {
                        if let Some(i) = self.get_member_index_by_name(&mr.member_name.data) {
                            if self.members[i].typ.array_size.is_no() {
                                return Err(InterpretError::MemberReferenceDoesntPointToArray(mr.member_name.code_view()))
                            }
                        } else {
                            return Err(InterpretError::UnknownStructMemberReference(mr.member_name.code_view()));
                        }
                    },
                    StructMemberConstant::Size(mr) => {
                        if self.get_member_index_by_name(&mr.member_name.data).is_none() {
                            return Err(InterpretError::UnknownStructMemberReference(mr.member_name.code_view()));
                        }
                    },
                    StructMemberConstant::EnumMemberValue(emr) => {
                        types.get_enum_member_value(&emr.enum_name.data, &emr.enum_member.data)?;
                    },
                    StructMemberConstant::SizeArithmetics(sa) => {
                        let mut expect_operator = sa[0].is_operator();
                        for s in sa {
                            if expect_operator {
                                if !s.is_operator() {
                                    return Err(InterpretError::ExpectedOperator(s.code_view()));
                                }
                            } else {
                                match &s.data {
                                    SizeArithmetics::MemberSizeReference(_) => {
                                        if self.get_member_index_by_name(&s.data.as_member_size_reference().unwrap().member_name).is_none() {
                                            return Err(InterpretError::UnknownStructMemberReference(s.code_view()));
                                        }
                                    },
                                    SizeArithmetics::MemberValueReference(_) => {
                                        if self.get_member_index_by_name(&s.data.as_member_value_reference().unwrap().member_name).is_none() {
                                            return Err(InterpretError::UnknownStructMemberReference(s.code_view()));
                                        }
                                    },
                                    SizeArithmetics::Plus | SizeArithmetics::Minus => 
                                        return Err(InterpretError::ExpectedMemberSize(s.code_view())),
                                    _ => ()
                                }
                                
                            }
                            expect_operator = !expect_operator;
                        }
                    }
                }
            }
        }
        Ok(())
    }
    pub fn resolve_members_with_unknown_types(
        &mut self,
        types: &Types,
    ) -> Result<(), InterpretError> {
        for member in &mut self.members {
            if let TypeVariant::Unknown(u) = &member.typ.typ {
                match types.get_type(&u.data) {
                    Ok(t) => match t {
                        None => return Err(InterpretError::UnknownType(u.clone())),
                        Some(t) => member.typ = Type {
                            typ: t,
                            array_size: member.typ.array_size.clone()
                        },
                    },
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(())
    }
    pub fn has_known_types(&self, known_types: &Vec<String>) -> bool {
        self.members
            .iter()
            .all(|mi| {
                if let Some(cst) = &mi.constant {
                    if let Some(emv) = cst.as_enum_member_value() {
                        return known_types.contains(&emv.enum_name.data);
                    }
                };
                match &mi.typ.typ {
                    TypeVariant::Struct(t) => known_types.contains(&t.borrow().name),
                    TypeVariant::Enum(_) => true,
                    TypeVariant::View(t) => known_types.contains(&t.borrow().name),
                    TypeVariant::Int(_) => return true,
                    TypeVariant::Unknown(_) => panic!("unexpected unknown type"),
                }
            })
    }
    pub fn get_member_index_by_name(&self, name: &str) -> Option<usize> {
        self.members.iter().position(|m| m.name.data == name)
    }
}
