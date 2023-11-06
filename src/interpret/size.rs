use super::*;

impl Size for Type {
    fn size(&self) -> Option<usize> {
        match self {
            Type::Struct(t) => t.borrow().size(),
            Type::Enum(t) => t.size(),
            Type::Variant(t) => t.borrow().size(),
            Type::View(t) => t.borrow().size(),
            Type::Int(t) => t.size(),
            Type::Unknown(t) => None,
        }
    }
}

impl Size for Enum {
    fn size(&self) -> Option<usize> {
        Some(self.underlaying_int.bytes as usize)
    }
}

impl Size for Struct {
    fn size(&self) -> Option<usize> {
        self.members.iter().fold(Some(0), |sum, m| {
            if let Some(size1) = sum {
                if let Some(size2) = m.typ.size() {
                    return Some(size1 + size2)
                }
            }
            None
        })
    }
}

impl Size for Variant {
    fn size(&self) -> Option<usize> {
        self.variants.iter().fold(None, |size_i, t| {
            if let Some(size1) = size_i {
                if let Some(size2) = t.typ.size() {
                    if size1 == size2 {
                        return Some(size1)
                    }
                }
            }
            None
        })
    }
}

impl Size for View {
    fn size(&self) -> Option<usize> {
        self.types.iter().fold(Some(0), |size_i, t| {
            if let Some(size1) = size_i {
                if let Some(size2) = t.size() {
                    return Some(size1)
                }
            }
            None
        })
    }
}

impl Size for Int {
    fn size(&self) -> Option<usize> {
        Some(self.bytes as usize)
    }
}
