use super::*;

impl Enum {
    pub fn check_type(&self) -> Result<(), InterpretError> {
        for constant in self.constants.iter() {
            if self
                .constants
                .iter()
                .filter(|it| it.name == constant.name)
                .count()
                != 1
            {
                return Err(InterpretError::EnumConstantNotUnique(
                    constant.code_view.clone(),
                ));
            }
            if self
                .constants
                .iter()
                .filter(|it| it.value == constant.value)
                .count()
                != 1
            {
                return Err(InterpretError::EnumConstantValueNotUnique(
                    constant.code_view.clone(),
                ));
            }
            if !self.underlaying_int.check_value(constant.value) {
                return Err(InterpretError::EnumConstantValueNotFitting(
                    constant.code_view.clone(),
                ));
            }
        }
        Ok(())
    }
}
