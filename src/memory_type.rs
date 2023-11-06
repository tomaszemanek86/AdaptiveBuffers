use super::*;

impl MemoryType {
    pub fn non_array_memory(self) -> Memory {
        Memory {
            memory: self,
            max_array_size: None
        }
    }

    pub fn is_reference(&self) -> bool {
        match self {
            MemoryType::Native(NativeType::ArrayDimensionReference(_)) => true,
            MemoryType::Native(NativeType::ViewKeyReference(_)) => true,
            _ => false
        }
    }

    pub fn referenced_view(&self) -> Option<Rc<StructMemberMemory>> {
        match self {
            MemoryType::Native(NativeType::ViewKeyReference(member)) => Some(member.clone()),
            _ => None
        }
    }

    pub fn referenced_array(&self) -> Option<Rc<StructMemberMemory>> {
        match self {
            MemoryType::Native(NativeType::ViewKeyReference(member)) => Some(member.clone()),
            _ => None
        }
    }
}