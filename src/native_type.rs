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

    pub fn size(&self) -> u8 {
        match self {
            Self::Bool => 1,
            Self::I8 => 1,
            Self::I16 => 2,
            Self::I32 => 4,
            Self::I64 => 8,
            Self::U8 => 1,
            Self::U16 => 2,
            Self::U24 => 3,
            Self::U32 => 4,
            Self::U64 => 8,
            Self::Unknown => panic!("cannot get bytes from unknow native type"),
            Self::ViewKeyReference(mr) => mr.key.memory.borrow().memory.as_native().unwrap().size(),
            Self::ArrayDimensionReference(mr) => mr.size.memory.borrow().memory.as_native().unwrap().size(),
        }
    }

    pub fn typename(&self, l: Language) -> &str {
        match l {
            Language::Cpp => {
                match self {
                    NativeType::Bool => "bool",
                    NativeType::U8 => "uint8_t",
                    NativeType::U16 => "uint16_t",
                    NativeType::U24 => "uint24_t",
                    NativeType::U32 => "uint32_t",
                    NativeType::U64 => "uint64_t",
                    NativeType::I8 => "int8_t",
                    NativeType::I16 => "int16_t",
                    NativeType::I32 => "int32_t",
                    NativeType::I64 => "int64_t",
                    NativeType::Unknown => panic!("unknown type"),
                    NativeType::ViewKeyReference(_) => todo!(),
                    NativeType::ArrayDimensionReference(_) => todo!(),
                }
            },
            _ => panic!("unexpected language")
        }
    }
}
