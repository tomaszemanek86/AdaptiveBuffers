use super::*;

impl StructMemberMemory {
    pub fn get_array_size_reference(&self) -> Option<Rc<StructMemberMemory>> {
        for m in &self.structure.borrow().fields {
            if let Some(nt) = m.memory.borrow().memory.as_native() {
                if let Some(adr) = nt.as_array_dimension_reference() {
                    if adr.array.name == self.name {
                        return Some(m.clone())
                    }
                }
            }
        }
        None
    }

    pub fn get_view_key_reference<'a>(&self) -> Option<ViewKeyReference> {
        for m in &self.structure.borrow().fields {
            if let Some(nt) = m.memory.borrow().memory.as_native() {
                if let Some(vkr) = nt.as_view_key_reference() {
                    if vkr.key.name == self.name {
                        return Some(vkr.clone())
                    }
                }
            }
        }
        None
    }
}
