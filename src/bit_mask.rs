use super::*;

impl BitMask {
    pub fn check_type(&self) -> Result<(), InterpretError> {
        if self.bits[0].bits[0].is_and() {
            return InterpretError::InvalidBitExpression(self.bits[0].bits[0].code_view())
        }
        for mask in &self.bits {
            // check not only befor value
            let mut not = false;
            for op in 1..mask.bits.len() {
                i
            }
        }
        Ok(())
    }
}