use super::*;

impl<T: MemoryDetails> MemoryDetails for std::rc::Rc<T> {
    fn exact_size(&self) -> Option<usize> {
        self.as_ref().exact_size()
    }

    fn max_size(&self) -> Option<usize> {
        self.as_ref().max_size()
    }

    fn buffer_size(&self) -> Option<usize> {
        self.as_ref().buffer_size()
    }

    fn submembers(&self) -> usize {
        self.as_ref().submembers()
    }

    fn context_size(&self) -> usize {
        self.as_ref().context_size()
    }
}

impl<T: MemoryDetails> MemoryDetails for RefCell<T> {
    fn exact_size(&self) -> Option<usize> {
        self.borrow().exact_size()
    }

    fn max_size(&self) -> Option<usize> {
        self.borrow().max_size()
    }

    fn buffer_size(&self) -> Option<usize> {
        self.borrow().buffer_size()
    }

    fn submembers(&self) -> usize {
        self.borrow().submembers()
    }

    fn context_size(&self) -> usize {
        self.borrow().context_size()
    }
}

impl MemoryDetails for MemoryDeclaration {
    fn exact_size(&self) -> Option<usize> {
        self.memory.exact_size()
    }

    fn max_size(&self) -> Option<usize> {
        self.memory.max_size()
    }

    fn buffer_size(&self) -> Option<usize> {
        self.memory.buffer_size()
    }

    fn submembers(&self) -> usize {
        self.memory.submembers()
    }

    fn context_size(&self) -> usize {
        self.memory.context_size()
    }
}

impl MemoryDetails for Memory {
    fn exact_size(&self) -> Option<usize> {
        self.memory.exact_size()
    }

    fn max_size(&self) -> Option<usize> {
        self.memory.max_size()
    }

    fn buffer_size(&self) -> Option<usize> {
        self.memory.buffer_size()
    }

    fn submembers(&self) -> usize {
        self.memory.submembers()
    }

    fn context_size(&self) -> usize {
        self.memory.context_size()
    }
}

impl MemoryDetails for MemoryType {
    fn exact_size(&self) -> Option<usize> {
        match self {
            MemoryType::Struct(s) => s.exact_size(),
            MemoryType::Enum(e) => e.exact_size(),
            MemoryType::View(v) => v.exact_size(),
            MemoryType::Native(n) => n.exact_size(),
        }
    }

    fn max_size(&self) -> Option<usize> {
        match self {
            MemoryType::Struct(s) => s.max_size(),
            MemoryType::Enum(e) => e.max_size(),
            MemoryType::View(v) => v.max_size(),
            MemoryType::Native(n) => n.max_size(),
        }
    }

    fn buffer_size(&self) -> Option<usize> {
        match self {
            MemoryType::Struct(s) => s.buffer_size(),
            MemoryType::Enum(e) => e.buffer_size(),
            MemoryType::View(v) => v.buffer_size(),
            MemoryType::Native(n) => n.buffer_size(),
        }
    }

    fn submembers(&self) -> usize {
        match self {
            MemoryType::Native(t) => t.submembers(),
            MemoryType::Struct(t) => t.submembers(),
            MemoryType::View(t) => t.submembers(),
            MemoryType::Enum(t) => t.submembers(),
        }
    }

    fn context_size(&self) -> usize {
        match self {
            MemoryType::Native(t) => t.context_size(),
            MemoryType::Struct(t) => t.context_size(),
            MemoryType::View(t) => t.context_size(),
            MemoryType::Enum(t) => t.context_size(),
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
            Self::ConstU8(_) => Some(1),
            Self::ConstU16(_) => Some(2),
            Self::ConstU24(_) => Some(3),
            Self::ConstU32(_) => Some(4),
            Self::ConstU64(_) => Some(8),
            Self::I8 => Some(1),
            Self::I16 => Some(2),
            Self::I32 => Some(4),
            Self::I64 => Some(8),
            Self::Unknown => None,
            Self::ViewKeyReference(mr) => mr.native_key.exact_size(),
            Self::ArrayDimensionReference(mr) => mr.origin.exact_size(),
            Self::StructMemberSize(m) => m.native.exact_size(),
        }
    }

    fn max_size(&self) -> Option<usize> {
        self.exact_size()
    }

    fn buffer_size(&self) -> Option<usize> {
        self.exact_size().and_then(|bytes| Some(bytes + 1))
    }

    fn submembers(&self) -> usize {
        1
    }

    fn context_size(&self) -> usize {
        cpp_ptr_size() * 2 + NativeType::U32.exact_size().unwrap()
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

    fn max_size(&self) -> Option<usize> {
        self.fields.iter().fold(Some(0), |sum, m| {
            if let Some(size1) = sum {
                if let Some(size2) = m.memory.borrow().max_size() {
                    return Some(size1 + size2);
                }
            }
            None
        })
    }

    fn buffer_size(&self) -> Option<usize> {
        self.fields.iter().fold(Some(0), |sum, m| {
            if let Some(size1) = sum {
                if let Some(size2) = m.memory.borrow().buffer_size() {
                    return Some(size1 + size2);
                }
            }
            None
        }).and_then(|bytes| Some(bytes + self.fields.len()))
    }

    fn submembers(&self) -> usize {
        self.fields.iter().map(|f| f.submembers()).sum()
    }

    fn context_size(&self) -> usize {
        self.fields.iter().map(|t| t.memory.context_size()).sum::<usize>()
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

    fn max_size(&self) -> Option<usize> {
        if self.types.iter().all(|v| v.memory.max_size().is_some()) {
            Some(
                self.types
                    .iter()
                    .map(|v| v.memory.max_size().unwrap())
                    .max()
                    .unwrap(),
            )
        } else {
            None
        }
    }

    fn buffer_size(&self) -> Option<usize> {
        if self.types.iter().all(|v| v.memory.buffer_size().is_some()) {
            Some(
                self.types
                    .iter()
                    .map(|v| v.memory.buffer_size().unwrap())
                    .max()
                    .unwrap(),
            )
        } else {
            None
        }.and_then(|bytes| Some(bytes + 3)) // u16 for type identification + bool for is set
    }

    fn submembers(&self) -> usize {
        self.types.iter().map(|t| t.memory.submembers()).max().unwrap()
    }

    fn context_size(&self) -> usize {
        self.types.iter().map(|t| t.memory.context_size()).max().unwrap()
    }
}

impl MemoryDetails for ViewPosibilityMemory {
    fn exact_size(&self) -> Option<usize> {
        self.memory.exact_size()
    }

    fn max_size(&self) -> Option<usize> {
        self.memory.max_size()
    }

    fn buffer_size(&self) -> Option<usize> {
        self.memory.buffer_size()
    }

    fn submembers(&self) -> usize {
        self.memory.submembers()
    }

    fn context_size(&self) -> usize {
        cpp_ptr_size() * 2 + NativeType::U32.exact_size().unwrap()
    }
}

impl MemoryDetails for EnumMemory {
    fn exact_size(&self) -> Option<usize> {
        self.underlaying_type.exact_size()
    }

    fn max_size(&self) -> Option<usize> {
        self.underlaying_type.max_size()
    }

    fn buffer_size(&self) -> Option<usize> {
        self.underlaying_type.buffer_size().and_then(|bytes| Some(bytes + 1))
    }

    fn submembers(&self) -> usize {
        self.underlaying_type.submembers()
    }

    fn context_size(&self) -> usize {
        cpp_ptr_size() * 2 + NativeType::U32.exact_size().unwrap()
    }
}

impl MemoryDetails for StructMemberMemory {
    fn exact_size(&self) -> Option<usize> {
        self.memory.borrow().exact_size()
    }

    fn max_size(&self) -> Option<usize> {
        self.memory.borrow().max_size()
    }

    fn buffer_size(&self) -> Option<usize> {
        self.memory.borrow().buffer_size()
    }

    fn submembers(&self) -> usize {
        self.memory.submembers()
    }

    fn context_size(&self) -> usize {
        self.memory.context_size()
    }
}
