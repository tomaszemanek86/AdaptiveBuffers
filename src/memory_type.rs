use super::*;

impl MemoryType {
    pub fn non_array_memory(self) -> Memory {
        Memory {
            memory: self,
            array_size: ArraySize::No
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
            MemoryType::Native(NativeType::ViewKeyReference(member)) => Some(member.view.clone()),
            _ => None
        }
    }
}