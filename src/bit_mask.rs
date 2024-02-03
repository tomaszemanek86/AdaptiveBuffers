use super::*;

impl BitMask {
    pub fn check_type(&self) -> Result<(), InterpretError> {
        for mask in &self.bits {
            if mask.bits[0].is_and() {
                return Err(InterpretError::InvalidBitExpression(self.bits[0].bits[0].code_view()))
            }
            for (i, op) in mask.bits[0..mask.bits.len().wrapping_sub(1)].iter().enumerate() {
                if op.is_not() {
                    if !mask.bits[i + 1].is_value() {
                        return Err(InterpretError::InvalidBitExpression(mask.bits[i + 1].code_view()))
                    }
                }
                if op.is_value() {
                    if !mask.bits[i + 1].is_and() {
                        return Err(InterpretError::InvalidBitExpression(mask.bits[i + 1].code_view()))
                    }
                }
                if op.is_value() {
                    if !mask.bits[i + 1].is_value() && !mask.bits[i + 1].is_not() {
                        return Err(InterpretError::InvalidBitExpression(mask.bits[i + 1].code_view()))
                    }
                }
            }
            if !mask.bits.last().unwrap().is_value() {
                unsafe {
                    return Err(InterpretError::InvalidBitExpression(mask.bits.last().unwrap_unchecked().code_view()));
                }
            }
        }
        Ok(())
    }
}