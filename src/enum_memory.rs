use super::*;
use crate::interpret::InterpretError;

impl EnumMemory {
    pub fn get_index(&self, member: &CodeView) -> Result<usize, InterpretError> {
        match self.constants
            .iter()
            .find(|c| c.name == member.view()) {
                Some(c) => Ok(c.value),
                None => Err(InterpretError::UnknownEnumMember(member.clone()))
            }
    }
}
