use super::*;

impl StructMemberConstantMemory {
    pub fn code_view(&self) -> CodeView {
        match self {
            Self::ViewMemberKey(v) => v.code_view.clone(),
        }
    }
}