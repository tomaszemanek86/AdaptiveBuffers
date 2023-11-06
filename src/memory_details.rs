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
}

impl MemoryDetails for NativeType {
    fn exact_size(&self) -> Option<usize> {
        match self {
            Self::Bool => Some(1),
            Self::U8 => Some(1),
            Self::U16 => Some(2),
            Self::U32 => Some(4),
            Self::U64 => Some(8),
            Self::I8 => Some(1),
            Self::I16 => Some(2),
            Self::I32 => Some(4),
            Self::I64 => Some(8),
            Self::Unknown => None,
            Self::ViewKeyReference(mr) => NativeType::from_max_number(mr.memory.borrow().memory.as_view().unwrap().types.len(), false).exact_size(),
            Self::ArrayDimensionReference(mr) => mr.exact_size(),
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
}
