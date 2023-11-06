use super::*;
use crate::interpret::InterpretError;

impl EnumMemory {
    pub fn get_index(&self, member: &CodeView) -> Result<usize, InterpretError> {
        match self.constants
            .iter()
            .enumerate()
            .find(|(_, c)| c.name == member.view()) {
                Some((i, c)) => Ok(i),
                None => Err(InterpretError::UnknownEnumMember(member.clone()))
            }
    }
}
