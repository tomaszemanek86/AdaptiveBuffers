use super::*;

impl ViewPosibilityConstantMemory {
    pub fn get_value(&self) -> usize {
        match self {
            ViewPosibilityConstantMemory::Default(v) => *v,
            ViewPosibilityConstantMemory::Usize(v) => *v,
            ViewPosibilityConstantMemory::EnumMemberRef(v) => v.get_value(),
        }
    }
}