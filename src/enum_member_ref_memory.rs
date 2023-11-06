use super::*;

impl EnumMemberRefMemory {
    pub fn get_value(&self) -> usize {
        self.enum_typ.constants[self.index].value
    }
}