use super::*;

impl<TData: AsMemory + Default + Clone> AsMemory for DataView<TData> {
    fn as_memory(&self, others: &Vec<MemoryDeclaration>) -> Result<Memory, InterpretError> {
        self.data.as_memory(others)
    }
}

impl AsMemory for Struct {
    fn as_memory(&self, others: &Vec<MemoryDeclaration>) -> Result<Memory, InterpretError> {
        let mut structure = Rc::new(RefCell::new(StructMemory {
            name: self.name.data.clone(),
            fields: Vec::new(),
        }));
        for member in &self.members {
            if let Some(c) = &member.constant {
                if c.is_usize() || c.is_enum_member_value() {
                    structure.borrow_mut().fields.push(Rc::new(StructMemberMemory {
                        name: member.name.data.clone(),
                        index: member.index,
                        memory: RefCell::new(member.typ.as_memory(others)?),
                        structure: structure.clone()
                    }));
                    continue
                }
            }
            structure.borrow_mut().fields.push(Rc::new(StructMemberMemory {
                name: member.name.data.clone(),
                index: member.index,
                memory: RefCell::new(member.typ.as_memory(others)?),
                structure: structure.clone()
            }));
        }
        // resolve view reference keys
        for (i, f) in structure.borrow().fields.iter().enumerate() {
            if let Some(c) = &self.members[i].constant {
                match c {
                    StructMemberConstant::Usize(value) => if let Some(nm) = f.memory.borrow_mut().memory.as_native_mut() {
                        nm.make_const(*value).map_err(|e| InterpretError::GenericError(e))?
                    } else {
                        return Err(InterpretError::CannotAsignUsizeCstToNonUnsignedMemory(*value))
                    },
                    StructMemberConstant::ViewReferenceKey(mr) => {
                        let index = self.get_member_index_by_name(&mr.member_name.data).unwrap();
                        let native_key = Rc::new(self.members[i].typ.as_memory(others)?.memory.as_native().unwrap().clone());
                        *f.memory.borrow_mut() = MemoryType::Native(NativeType::ViewKeyReference(
                                ViewKeyReference {
                                    native_key: native_key,
                                    key: f.clone(),
                                    view: structure.borrow().fields[index].clone()
                                }
                            )).non_array_memory();
        
                    },
                    StructMemberConstant::ArrayDimension(mr) => {
                        let index = self.get_member_index_by_name(&mr.member_name.data).unwrap();
                        let native = Rc::new(self.members[i].typ.as_memory(others)?.memory.as_native().unwrap().clone());
                        *f.memory.borrow_mut() = MemoryType::Native(NativeType::ArrayDimensionReference(
                                ArrayDimensionReference {
                                    origin: native,
                                    size: f.clone(),
                                    array: structure.borrow().fields[index].clone()
                                }
                            )).non_array_memory();
                    },
                    StructMemberConstant::Size(mr) => {
                        let index = self.get_member_index_by_name(&mr.member_name.data).unwrap();
                        let native = Rc::new(self.members[i].typ.as_memory(others)?.memory.as_native().unwrap().clone());
                        *f.memory.borrow_mut() = MemoryType::Native(NativeType::StructMemberSize(
                                StructMemberSizeReference {
                                    native: native,
                                    origin: f.clone(),
                                    member: structure.borrow().fields[index].clone()
                                }
                            )).non_array_memory();
                    },
                    StructMemberConstant::EnumMemberValue(emv) => {
                        let value = others
                                .iter()
                                .find(|it: &&MemoryDeclaration| it.name == emv.enum_name.data)
                                .ok_or_else(|| InterpretError::UnknownEnum(emv.enum_name.code_view.clone()))?
                                .memory.memory
                                .as_enum().unwrap()
                                .constants
                                .iter().find(|it| it.name == emv.enum_member.data)
                                .ok_or_else(|| InterpretError::UnknownEnumMember(emv.enum_member.code_view.clone()))?
                                .value;
                        if let Some(nm) = f.memory.borrow_mut().memory.as_native_mut() {
                            nm.make_const(value).map_err(|e| InterpretError::GenericError(e))?
                        } else {
                            return Err(InterpretError::CannotAsignUsizeCstToNonUnsignedMemory(value))
                        }
                    },
                    StructMemberConstant::SizeArithmetics(sa) => {
                        let native = Rc::new(self.members[i].typ.as_memory(others)?.memory.as_native().unwrap().clone());
                        *f.memory.borrow_mut() = MemoryType::Native(NativeType::StructMemberSizeArithmetics(
                            StructMemberSizeArithmetics {
                                native: native,
                                arithmetics: sa.iter().map(|it| match &it.data {
                                    parser::SizeArithmetics::Plus => SizeArithmetics::Plus,
                                    parser::SizeArithmetics::Minus => SizeArithmetics::Minus,
                                    parser::SizeArithmetics::MemberReference(mr) => {
                                        let index = self.get_member_index_by_name(&mr.member_name.data).unwrap();
                                        SizeArithmetics::StructMemberSizeReference(structure.borrow().fields[index].clone())
                                    }
                                    parser::SizeArithmetics::Usize(value) => SizeArithmetics::Usize(*value)
                                }).collect()
                            }
                        )).non_array_memory();
                        
                    }
                } 
            }
        }
        for f in &structure.borrow().fields {
            if let Some(nm) = f.memory.borrow().memory.as_native() {
                if let NativeType::ViewKeyReference(vrk) = nm {
                    if vrk.view.memory.borrow().memory.as_view().unwrap().get_index_typename().size() > 4 {
                        return Err(InterpretError::ViewReferenceKeyIsTooBig(
                            self.members[f.index].constant
                                .as_ref()
                                .unwrap()
                                .as_view_reference_key()
                                .unwrap()
                                .member_name.code_view.clone()
                            ));
                    }
                }
            }
        }
        Ok(MemoryType::Struct(structure).non_array_memory())
    }
}

impl AsMemory for Enum {
    fn as_memory(&self, others: &Vec<MemoryDeclaration>) -> Result<Memory, InterpretError> {
        let mut new_enum = EnumMemory {
            name: self.name.clone(),
            underlaying_type: self.underlaying_int.as_memory(others)?.memory.into_native(),
            constants: Vec::new(),
        };
        for constant in &self.constants {
            new_enum.constants.push(EnumConstantMemory {
                name: constant.name.clone(),
                value: constant.value,
            });
        }
        Ok(MemoryType::Enum(Rc::new(new_enum)).non_array_memory())
    }
}

impl AsMemory for Int {
    fn as_memory(&self, _others: &Vec<MemoryDeclaration>) -> Result<Memory, InterpretError> {
        if self.signed {
            match self.bytes {
                8 => Ok(MemoryType::Native(NativeType::I8).non_array_memory()),
                16 => Ok(MemoryType::Native(NativeType::I16).non_array_memory()),
                32 => Ok(MemoryType::Native(NativeType::I32).non_array_memory()),
                64 => Ok(MemoryType::Native(NativeType::I64).non_array_memory()),
                _ => Err(InterpretError::UnknownIntSize(self.bytes)),
            }
        } else {
            match self.bytes {
                8 => Ok(MemoryType::Native(NativeType::U8).non_array_memory()),
                16 => Ok(MemoryType::Native(NativeType::U16).non_array_memory()),
                24 => Ok(MemoryType::Native(NativeType::U24).non_array_memory()),
                32 => Ok(MemoryType::Native(NativeType::U32).non_array_memory()),
                64 => Ok(MemoryType::Native(NativeType::U64).non_array_memory()),
                _ => Err(InterpretError::UnknownIntSize(self.bytes)),
            }
        }
    }
}

impl AsMemory for View {
    fn as_memory(&self, others: &Vec<MemoryDeclaration>) -> Result<Memory, InterpretError> {
        let new_view = ViewMemory {
            name: self.name.clone(),
            types: self
                .types
                .iter()
                .enumerate()
                .map(|(i, vp)| 
                    Ok(ViewPosibilityMemory {
                        memory: vp.typ.as_memory(others)?.memory,
                        constant: {
                            if let Some(constant) = &vp.constant {
                                if let Some(v) = constant.as_enum_member_ref() {
                                    let md = others.iter().find(|md| md.name == v.enum_name.data);
                                    if md.is_none() {
                                        return Err(InterpretError::UnknownEnum(v.enum_name.code_view.clone()));
                                    }
                                    ViewPosibilityConstantMemory::EnumMemberRef(EnumMemberRefMemory {
                                        enum_typ: md.unwrap().memory.memory.as_enum().unwrap().clone(),
                                        index: md.unwrap().memory.memory.as_enum().unwrap().get_index(&v.enum_member.code_view)?
                                    })
                                } else if let Some(v) = constant.as_usize() {
                                    ViewPosibilityConstantMemory::Usize(v.data)
                                } else {
                                    ViewPosibilityConstantMemory::Default(i)
                                }
                            } else {
                                ViewPosibilityConstantMemory::Default(i)
                            }
                        }
                    })
                )
                .collect::<Result<Vec<ViewPosibilityMemory>, InterpretError>>()?
        };
        Ok(MemoryType::View(new_view).non_array_memory())
    }
}

impl AsMemory for TypeVariant {
    fn as_memory(&self, others: &Vec<MemoryDeclaration>) -> Result<Memory, InterpretError> {
        match self {
            TypeVariant::Struct(s) => s.borrow().as_memory(others),
            TypeVariant::View(v) => v.borrow().as_memory(others),
            TypeVariant::Enum(e) => e.as_memory(others),
            TypeVariant::Int(i) => i.as_memory(others),
            TypeVariant::Unknown(unknown) => Err(InterpretError::UnknownType(unknown.clone())),
        }
    }
}

impl AsMemory for Type {
    fn as_memory(&self, others: &Vec<MemoryDeclaration>) -> Result<Memory, InterpretError> {
        Ok(Memory {
            memory: self.typ.as_memory(others)?.memory,
            array_size: self.array_size.clone()
        })
    }
}
