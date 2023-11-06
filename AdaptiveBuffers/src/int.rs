use super::*;

impl Int {
    pub fn check_value(&self, value: usize) -> bool {
        let max_value = match self.bytes {
            8 => 0xFF,
            16 => 0xFFFF,
            32 => 0xFFFFFFFF,
            64 => 0xFFFFFFFFFFFFFFFF,
            _ => unreachable!(),
        };
        if value > max_value {
            return false;
        }
        true
    }

    pub fn max_value(&self) -> usize {
        match self {
            Self { signed: false, bytes: 1 } => u8::MAX as usize,
            Self { signed: false, bytes: 2 } => u16::MAX as usize,
            Self { signed: false, bytes: 4 } => u32::MAX as usize,
            Self { signed: false, bytes: 8 } => u64::MAX as usize,
            Self { signed: true, bytes: 1 } => i8::MAX as usize,
            Self { signed: true, bytes: 2 } => i16::MAX as usize,
            Self { signed: true, bytes: 4 } => i32::MAX as usize,
            Self { signed: true, bytes: 8 } => i64::MAX as usize,
            _ => unreachable!(),
        }
    }
}
