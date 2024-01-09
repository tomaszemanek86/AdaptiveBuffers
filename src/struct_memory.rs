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
            .view
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
            return None
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

    pub fn get_groups(&self) -> std::vec::Vec<(usize, usize)> {
        let mut out = std::vec::Vec::default();
        let mut i0 = 0;
        for i in 0..self.fields.len() {
            if self.fields[i].memory.exact_size().is_none() {
                out.push((i0, i));
                i0 = i + 1;
            }
        }
        // for case all items are sized
        if out.len() == 0 {
            out.push((0, self.fields.len() - 1))
        }
        let last_index = self.fields.len() - 1;
        if out.last().unwrap().1 != last_index {
            out.push((i0, last_index))
        }
        out
    }
}
