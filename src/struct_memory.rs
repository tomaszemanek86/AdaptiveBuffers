use super::*;

impl StructMemory {
    pub fn get_referenced_view_index(&self, member_index: usize) -> usize {
        if !self.is_view_reference(member_index) {
            panic!("not a view key reference")
        }
        self.fields[member_index].memory
            .borrow()
            .memory
            .as_native()
            .unwrap()
            .as_view_key_reference()
            .unwrap()
            .index
    }

    pub fn is_view_reference(&self, member_index: usize) -> bool {
        if self.fields[member_index].memory.borrow().memory.is_native() {
            if self.fields[member_index].memory.borrow().memory.as_native().unwrap().is_view_key_reference() {
                return true
            }
        }
        false
    }

    pub fn get_view_key_reference_member_index(&self, member_index: usize) -> Option<usize> {
        if !self.is_view(member_index) {
            panic!("not a view")
        }
        for (i, _f) in  self.fields.iter().enumerate() {
            if  self.is_view_reference(i) && 
                self.get_referenced_view_index(i) == member_index 
            {
                return Some(i);
            }
        }
        None
    }

    pub fn is_view(&self, member_index: usize) -> bool {
        if self.fields[member_index].memory.borrow().memory.is_view() {
            return true
        }
        false
    }
}
