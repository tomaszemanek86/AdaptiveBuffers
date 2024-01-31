use super::*;
use crate::interpret::InterpretError;

impl EnumMemory {
    pub fn get_index(&self, member: &CodeView) -> Result<usize, InterpretError> {
        for (i, cst) in self.constants.iter().enumerate() {
            if cst.name == member.view() {
                return Ok(i)
            }
        }
        Err(InterpretError::UnknownEnumMember(member.clone()))
    }
}
