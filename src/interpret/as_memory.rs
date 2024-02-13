use super::*;

impl<TData: AsMemory + Default + Clone> AsMemory for DataView<TData> {
    fn as_memory(&self, others: &Vec<MemoryDeclaration>) -> Result<Memory, InterpretError> {
        self.data.as_memory(others)
    }
}

impl AsMemory for Struct {
    fn as_memory(&self, others: &Vec<MemoryDeclaration>) -> Result<Memory, InterpretError> {
        let structure = Rc::new(RefCell::new(StructMemory {
            name: self.name().to_string(),
            parsed: self.parsed.clone(),
            fields: Vec::new(),
        }));
        for member in &self.members {
            if let Some(c) = &member.constant {
                if c.is_usize() || c.is_enum_member_value() {
                    structure.borrow_mut().fields.push(Rc::new(StructMemberMemory {
                        name: member.name.data.clone(),
                        index: member.index,
                        memory: RefCell::new(member.typ.as_memory(others)?),
                        structure: structure.clone(),
                        parsed: self.parsed.clone()
                    }));
                    continue
                }
            }
            structure.borrow_mut().fields.push(Rc::new(StructMemberMemory {
                name: member.name.data.clone(),
                index: member.index,
                memory: RefCell::new(member.typ.as_memory(others)?),
                structure: structure.clone(),
                parsed: self.parsed.clone()
            }));
        }
        // resolve view reference keys
        for (i, f) in structure.borrow().fields.iter().enumerate() {
            if let Some(c) = &self.members[i].constant {
                match c {
                    StructMemberConstant::Usize(value) => if let Some(nm) = f.memory.borrow_mut().memory.as_native_mut() {
                        nm.typ.make_const(*value).map_err(|e| InterpretError::GenericError(e))?
                    } else {
                        return Err(InterpretError::CannotAsignUsizeCstToNonUnsignedMemory(*value))
                    },
                    StructMemberConstant::ViewReferenceKey(mr) => {
                        let index = self.get_member_index_by_name(&mr.member_name.data).unwrap();
                        let native_key = Rc::new(self.members[i].typ.as_memory(others)?.memory.as_native().unwrap().clone());
                        *f.memory.borrow_mut() = MemoryType::Native(
                            Native { 
                                typ: NativeType::ViewKeyReference(
                                        ViewKeyReference {
                                            native_key: native_key.clone(),
                                            key: f.clone(),
                                            view: structure.borrow().fields[index].clone()
                                        }
                                    ),
                                endian: native_key.endian.clone()
                            }).non_array_memory();
                    },
                    StructMemberConstant::ArrayDimension(mr) => {
                        let index = self.get_member_index_by_name(&mr.member_name.data).unwrap();
                        let native = Rc::new(self.members[i].typ.as_memory(others)?.memory.as_native().unwrap().clone());
                        *f.memory.borrow_mut() = MemoryType::Native(
                            Native {
                                typ: NativeType::ArrayDimensionReference(
                                    ArrayDimensionReference {
                                        origin: native.clone(),
                                        size: f.clone(),
                                        array: structure.borrow().fields[index].clone()
                                    }
                                ),
                                endian: native.endian.clone()
                            }).non_array_memory();
                    },
                    StructMemberConstant::Size(mr) => {
                        let index = self.get_member_index_by_name(&mr.member_name.data).unwrap();
                        let native = Rc::new(self.members[i].typ.as_memory(others)?.memory.as_native().unwrap().clone());
                        *f.memory.borrow_mut() = MemoryType::Native(
                            Native {
                                typ: NativeType::StructMemberSize(
                                    StructMemberSizeReference {
                                        native: native.clone(),
                                        origin: f.clone(),
                                        member: structure.borrow().fields[index].clone()
                                    }
                                ),
                                endian: native.endian.clone()
                            }).non_array_memory();
                    },
                    StructMemberConstant::EnumMemberValue(emv) => {
                        let value = others
                                .iter()
                                .find(|it: &&MemoryDeclaration| it.name == emv.enum_name.data)
                                .ok_or_else(|| InterpretError::UnknownEnum(emv.enum_name.code_view().clone()))?
                                .memory.memory
                                .as_enum().unwrap()
                                .constants
                                .iter().find(|it| it.name == emv.enum_member.data)
                                .ok_or_else(|| InterpretError::UnknownEnumMember(emv.enum_member.code_view().clone()))?
                                .value;
                        if let Some(nm) = f.memory.borrow_mut().memory.as_native_mut() {
                            nm.typ.make_const(value).map_err(|e| InterpretError::GenericError(e))?
                        } else {
                            return Err(InterpretError::CannotAsignUsizeCstToNonUnsignedMemory(value))
                        }
                    },
                    StructMemberConstant::SizeArithmetics(sa) => {
                        let native = Rc::new(self.members[i].typ.as_memory(others)?.memory.as_native().unwrap().clone());
                        for it in sa.iter() {
                            match &it.data {
                                parser::SizeArithmetics::MemberValueReference(mr) => {
                                    let index = self.get_member_index_by_name(&mr.member_name.data).unwrap();
                                    if !structure.borrow().fields[index].memory.borrow().memory.can_get_unsigned_value() {
                                        return Err(InterpretError::MemberValueNoUnsigned(it.code_view().clone()))
                                    }
                                },
                                _ => continue
                            }
                        }
                        *f.memory.borrow_mut() = MemoryType::Native(
                            Native {
                                typ: NativeType::StructMemberSizeArithmetics(
                                    StructMemberSizeArithmetics {
                                        native: native.clone(),
                                        arithmetics: sa.iter().map(|it| match &it.data {
                                            parser::SizeArithmetics::Plus => SizeArithmetics::Plus,
                                            parser::SizeArithmetics::Minus => SizeArithmetics::Minus,
                                            parser::SizeArithmetics::MemberSizeReference(mr) => {
                                                let index = self.get_member_index_by_name(&mr.member_name.data).unwrap();
                                                SizeArithmetics::StructMemberSizeReference(structure.borrow().fields[index].clone())
                                            },
                                            parser::SizeArithmetics::MemberValueReference(mr) => {
                                                let index = self.get_member_index_by_name(&mr.member_name.data).unwrap();
                                                SizeArithmetics::StructMemberValueReference(structure.borrow().fields[index].clone())
                                            },
                                            parser::SizeArithmetics::Usize(value) => SizeArithmetics::Usize(*value)
                                        }).collect()
                                    }
                                ), 
                            endian: native.endian.clone()
                        }).non_array_memory();
                    }
                } 
            }
        }
        for f in &structure.borrow().fields {
            if let Some(nm) = f.memory.borrow().memory.as_native() {
                if let NativeType::ViewKeyReference(vrk) = &nm.typ {
                    if vrk.view.memory.borrow().memory.as_view().unwrap().get_index_typename().size() > 4 {
                        return Err(InterpretError::ViewReferenceKeyIsTooBig(
                            self.members[f.index].constant
                                .as_ref()
                                .unwrap()
                                .as_view_reference_key()
                                .unwrap()
                                .member_name.code_view()
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
                8 => Ok(MemoryType::Native(Native { typ: NativeType::I8, endian: OverrideEndian::Default }).non_array_memory()),
                16 => Ok(MemoryType::Native(Native { typ: NativeType::I16, endian: OverrideEndian::Default }).non_array_memory()),
                32 => Ok(MemoryType::Native(Native { typ: NativeType::I32, endian: OverrideEndian::Default }).non_array_memory()),
                64 => Ok(MemoryType::Native(Native { typ: NativeType::I64, endian: OverrideEndian::Default }).non_array_memory()),
                _ => Err(InterpretError::UnknownIntSize(self.bytes)),
            }
        } else {
            match self.bytes {
                8 => Ok(MemoryType::Native(Native { typ: NativeType::U8, endian: OverrideEndian::Default }).non_array_memory()),
                16 => Ok(MemoryType::Native(Native { typ: NativeType::U16, endian: OverrideEndian::Default }).non_array_memory()),
                24 => Ok(MemoryType::Native(Native { typ: NativeType::U24, endian: OverrideEndian::Default }).non_array_memory()),
                32 => Ok(MemoryType::Native(Native { typ: NativeType::U32, endian: OverrideEndian::Default }).non_array_memory()),
                64 => Ok(MemoryType::Native(Native { typ: NativeType::U64, endian: OverrideEndian::Default }).non_array_memory()),
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
                                        return Err(InterpretError::UnknownEnum(v.enum_name.code_view()));
                                    }
                                    ViewPosibilityConstantMemory::EnumMemberRef(EnumMemberRefMemory {
                                        enum_typ: md.unwrap().memory.memory.as_enum().unwrap().clone(),
                                        index: md.unwrap().memory.memory.as_enum().unwrap().get_index(&v.enum_member.code_view())?
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

impl AsMemory for BitMask {
    fn as_memory(&self, others: &Vec<MemoryDeclaration>) -> Result<Memory, InterpretError> {
        let mut bit_mask = self.clone();
        bit_mask.native = self.native.typ.as_unknown().unwrap().as_memory(others)?.memory.as_native().unwrap().typ.no_swap();
        for mask in &mut bit_mask.bits {
            for bit_op in &mut mask.bits {
                if let Some(value) = bit_op.as_value_mut() {
                    let count = value.count_ones();
                    if count == 1 {
                        let index = value.trailing_zeros() as usize;
                        *value = index;
                    } else {
                        return Err(InterpretError::NotSingleBitValue(bit_op.code_view()))
                    }
                }
            }
        }
        return Ok(Memory {
            memory: MemoryType::BitMask(Rc::new(bit_mask)),
            array_size: ArraySize::No
        })
    }
}

impl AsMemory for TypeVariant {
    fn as_memory(&self, others: &Vec<MemoryDeclaration>) -> Result<Memory, InterpretError> {
        match self {
            TypeVariant::Struct(s) => s.borrow().as_memory(others),
            TypeVariant::View(v) => v.borrow().as_memory(others),
            TypeVariant::BitMask(b) => b.as_memory(others),
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
