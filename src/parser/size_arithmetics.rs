use super::*;

impl SizeArithmetics {
    pub fn is_operator(&self) -> bool {
        self.is_plus() || self.is_minus()
    }
}