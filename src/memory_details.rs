use super::*;

impl<T: MemoryDetails> MemoryDetails for std::rc::Rc<T> {
    fn exact_size(&self) -> Option<usize> {
        self.as_ref().exact_size()
    }
}

impl<T: MemoryDetails> MemoryDetails for RefCell<T> {
    fn exact_size(&self) -> Option<usize> {
        self.borrow().exact_size()
    }
}

impl MemoryDetails for MemoryDeclaration {
    fn exact_size(&self) -> Option<usize> {
        self.memory.exact_size()
    }
}

impl MemoryDetails for Memory {
    fn exact_size(&self) -> Option<usize> {
        self.memory.exact_size()
    }
}

impl MemoryDetails for MemoryType {
    fn exact_size(&self) -> Option<usize> {
        match self {
            MemoryType::Struct(s) => s.exact_size(),
            MemoryType::Enum(e) => e.exact_size(),
            MemoryType::View(v) => v.exact_size(),
            MemoryType::Native(n) => n.exact_size(),
            MemoryType::BitMask(b) => b.exact_size(),
        }
    }
}

impl MemoryDetails for NativeType {
    fn exact_size(&self) -> Option<usize> {
        match self {
            Self::Bool => Some(1),
            Self::U8 => Some(1),
            Self::U16 => Some(2),
            Self::U24 => Some(3),
            Self::U32 => Some(4),
            Self::U64 => Some(8),
            Self::NoSwapU8 => Some(1),
            Self::NoSwapU16 => Some(2),
            Self::NoSwapU24 => Some(3),
            Self::NoSwapU32 => Some(4),
            Self::NoSwapU64 => Some(8),
            Self::ConstU8(_) => Some(1),
            Self::ConstU16(_) => Some(2),
            Self::ConstU24(_) => Some(3),
            Self::ConstU32(_) => Some(4),
            Self::ConstU64(_) => Some(8),
            Self::I8 => Some(1),
            Self::I16 => Some(2),
            Self::I32 => Some(4),
            Self::I64 => Some(8),
            Self::Unknown(_) => None,
            Self::ViewKeyReference(mr) => mr.native_key.exact_size(),
            Self::ArrayDimensionReference(mr) => mr.origin.exact_size(),
            Self::StructMemberSize(m) => m.native.exact_size(),
            NativeType::StructMemberSizeArithmetics(m) => m.native.exact_size(),
        }
    }
}

impl MemoryDetails for Native {
    fn exact_size(&self) -> Option<usize> {
        self.typ.exact_size()
    }
}

impl MemoryDetails for StructMemory {
    fn exact_size(&self) -> Option<usize> {
        self.fields.iter().fold(Some(0), |sum, m| {
            if let Some(size1) = sum {
                if let Some(size2) = m.memory.borrow().exact_size() {
                    return Some(size1 + size2);
                }
            }
            None
        })
    }
}

impl MemoryDetails for ViewMemory {
    fn exact_size(&self) -> Option<usize> {
        self.types.iter().fold(Some(0), |size_i, t| {
            if let Some(size1) = size_i {
                if let Some(size2) = t.memory.exact_size() {
                    if size1 == size2 {
                        return Some(size1);
                    }
                }
            }
            None
        })
    }
}

impl MemoryDetails for ViewPosibilityMemory {
    fn exact_size(&self) -> Option<usize> {
        self.memory.exact_size()
    }
}

impl MemoryDetails for EnumMemory {
    fn exact_size(&self) -> Option<usize> {
        self.underlaying_type.exact_size()
    }
}

impl MemoryDetails for StructMemberMemory {
    fn exact_size(&self) -> Option<usize> {
        self.memory.borrow().exact_size()
    }
}

impl MemoryDetails for BitMask {
    fn exact_size(&self) -> Option<usize> {
        self.native.exact_size()
    }
}
