use super::*;

impl NativeType {
    pub fn from_max_number(max_number: usize, signed: bool) -> NativeType {
        if signed {
            if max_number < i8::MAX as usize {
                return Self::I8;
            }
            if max_number < i16::MAX as usize {
                return Self::I16;
            }
            if max_number < i32::MAX as usize {
                return Self::I32;
            }
            Self::I64
        } else {
            if max_number < u8::MAX as usize {
                return Self::U8;
            }
            if max_number < u16::MAX as usize {
                return Self::U16;
            }
            if max_number < u32::MAX as usize {
                return Self::U32;
            }
            Self::U64
        }
    }

    pub fn bytes(&self) -> u8 {
        match self {
            Self::Bool => 1,
            Self::I8 => 1,
            Self::I16 => 2,
            Self::I32 => 4,
            Self::I64 => 8,
            Self::U8 => 1,
            Self::U16 => 2,
            Self::U32 => 4,
            Self::U64 => 8,
            Self::Unknown => panic!("cannot get bytes from unknow native type"),
            Self::ViewKeyReference(mr) => mr.memory.borrow().memory.as_native().unwrap().bytes(),
            Self::ArrayDimensionReference(mr) => mr.memory.borrow().memory.as_native().unwrap().bytes(),
        }
    }
}
