use super::*;

impl TypeVariant {
    pub fn from_parser_typ(typ: &parser::TypVariant, array_size: ArraySize) -> Type {
        match typ {
            parser::TypVariant::Int(int) => Type {
                typ: TypeVariant::Int(int.clone()),
                array_size: array_size
            },
            parser::TypVariant::Unknown(unknown) => Type {
                typ: TypeVariant::Unknown(unknown.clone()),
                array_size: array_size
            },
            parser::TypVariant::UnknownType => panic!("unexpected unknown type"),
        }
    }
    pub fn from_struct(s: DataView<parser::Struct>) -> Result<TypeVariant, InterpretError> {
        // Check if member is self
        if s.members.iter().any(|member| {
            if let parser::TypVariant::Unknown(user_defined) = &member.typ.typ {
                user_defined.data == s.name.data
            } else {
                false
            }
        }) {
            return Err(InterpretError::TypeContainsItself(s));
        }
        // create structure
        Ok(TypeVariant::Struct(Rc::new(RefCell::new(DataView::new(
            Struct {
                name: s.name.clone(),
                members: s
                    .members
                    .iter()
                    .enumerate()
                    .map(|(i, member)| {
                        return StructMember {
                            name: member.name.clone(),
                            index: i,
                            typ: Self::from_parser_typ(&member.typ.typ, member.typ.array_size.clone()),
                            constant: match &member.constant {
                                    parser::StructMemberConstant::No => None,
                                    parser::StructMemberConstant::Usize(value) => Some(StructMemberConstant::Usize(*value)),
                                    parser::StructMemberConstant::ViewMemberKey(mr) => {
                                        Some(StructMemberConstant::ViewReferenceKey(mr.clone()))
                                    },
                                    parser::StructMemberConstant::ArrayDimension(mr) => {
                                        Some(StructMemberConstant::ArrayDimension(mr.clone()))
                                    }
                                    parser::StructMemberConstant::Size(mr) => {
                                        Some(StructMemberConstant::Size(mr.clone()))
                                    }
                            },
                        };
                    })
                    .collect(),
            },
            s.code_view,
        )))))
    }
    pub fn from_enum(e: DataView<parser::Enum>) -> Result<TypeVariant, InterpretError> {
        let new_enum = DataView::new(
            Enum {
                name: e.data.name.clone(),
                underlaying_int: e.data.underlaying_int.clone(),
                constants: e
                    .data
                    .constants
                    .into_iter()
                    .map(|constant| {
                        DataView::new(
                            EnumConstant {
                                name: constant.name.clone(),
                                value: constant.typ.value.unwrap(),
                            },
                            constant.code_view,
                        )
                    })
                    .collect::<Vec<DataView<EnumConstant>>>(), // Call collect here
            },
            e.code_view,
        );
        Ok(TypeVariant::Enum(Rc::new(new_enum)))
    }
    pub fn from_view(v: DataView<parser::View>) -> Result<TypeVariant, InterpretError> {
        Ok(TypeVariant::View(Rc::new(RefCell::new(DataView::new(
            View {
                name: v.data.name.clone(),
                types: v
                    .data
                    .types
                    .iter()
                    //.map(|t| Self::from_parser_typ(&t.data))
                    .map(|t| ViewPosibility { 
                        typ: Self::from_parser_typ(&t.data.typ.typ, t.data.typ.array_size.clone()), 
                        constant: t.data.constant.clone(),
                    })
                    .collect(),
            },
            v.code_view,
        )))))
    }
    pub fn has_known_types(&self, known_types: &Vec<String>) -> bool {
        match self {
            TypeVariant::Struct(structure) => structure.borrow().has_known_types(known_types),
            TypeVariant::View(view) => view.borrow().has_known_types(known_types),
            TypeVariant::Enum(_) => return true,
            TypeVariant::Int(_) => return true,
            TypeVariant::Unknown(_) => panic!("unexpected unknoqn type"),
        }
    }
    pub fn check_type(&self, types: &Types) -> Result<(), InterpretError> {
        match self {
            TypeVariant::Struct(s) => s.borrow().check_type(types),
            TypeVariant::Enum(e) => e.check_type(),
            TypeVariant::View(v) => v.borrow_mut().check_type(types),
            TypeVariant::Int(_i) => Ok(()),
            TypeVariant::Unknown(_unknown) => panic!("cannot check type for unknown"),
        }
    }
    pub fn resolve_unknown_types(&self, types: &Types) -> Result<(), InterpretError> {
        match self {
            TypeVariant::Struct(s) => s.borrow_mut().resolve_members_with_unknown_types(types),
            TypeVariant::View(v) => v.borrow_mut().resolve_unknown_types(types),
            TypeVariant::Enum(_e) => Ok(()),
            TypeVariant::Int(_) => Ok(()),
            TypeVariant::Unknown(_unknown) => panic!("cannot resolve unknown types for unknown"),
        }
    }
    pub fn code_view(&self) -> CodeView {
        match self {
            TypeVariant::Struct(s) => s.borrow().code_view.clone(),
            TypeVariant::View(v) => v.borrow().code_view.clone(),
            TypeVariant::Enum(e) => e.code_view.clone(),
            TypeVariant::Int(i) => i.code_view.clone(),
            TypeVariant::Unknown(_unknown) => panic!("cannot get code view for unknown"),
        }
    }
}
