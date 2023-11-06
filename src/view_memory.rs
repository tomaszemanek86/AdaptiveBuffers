use super::*;

impl ViewMemory {
    pub fn get_index_typename(&self) -> NativeType {
        let mut max_value = 0;
        for t in &self.types {
            match &t.constant {
                ViewPosibilityConstantMemory::Default(v) => max_value = max_value.max(*v),
                ViewPosibilityConstantMemory::Usize(v) => max_value = max_value.max(*v),
                ViewPosibilityConstantMemory::EnumMemberRef(emr) => max_value = max_value.max(emr.get_value()),
            }
        }
        NativeType::from_max_number(max_value, false)
    }
}
