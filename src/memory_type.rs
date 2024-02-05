use super::*;

impl MemoryType {
    pub fn non_array_memory(self) -> Memory {
        Memory {
            memory: self,
            array_size: ArraySize::No
        }
    }

    pub fn is_reference(&self) -> bool {
        if let Some(native) = self.as_native() {
            return native.typ.is_array_dimension_reference() || native.typ.is_view_key_reference();
        }
        false
    }

    pub fn referenced_view(&self) -> Option<Rc<StructMemberMemory>> {
        if let Some(native) = self.as_native() {
            if let Some(vkr) = native.typ.as_view_key_reference() {
                return Some(vkr.view.clone());
            }
        }
        None
    }

    pub fn can_get_unsigned_value(&self) -> bool {
        match self {
            MemoryType::Native(n) => n.typ.is_unsigned(),
            _ => false
        }
    }
}