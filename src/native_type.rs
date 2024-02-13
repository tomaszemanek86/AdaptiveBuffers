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

    pub fn size(&self) -> usize {
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
            Self::NoSwapU8 => 1,
            Self::NoSwapU16 => 2,
            Self::NoSwapU24 => 3,
            Self::NoSwapU32 => 4,
            Self::NoSwapU64 => 8,
            Self::ConstU8(_) => 1,
            Self::ConstU16(_) => 2,
            Self::ConstU24(_) => 3,
            Self::ConstU32(_) => 4,
            Self::ConstU64(_) => 8,
            Self::Unknown(_) => panic!("cannot get bytes from unknow native type"),
            Self::ViewKeyReference(mr) => mr.key.memory.borrow().memory.as_native().unwrap().typ.size(),
            Self::ArrayDimensionReference(mr) => mr.size.memory.borrow().memory.as_native().unwrap().typ.size(),
            Self::StructMemberSize(m) => m.origin.memory.borrow().memory.as_native().unwrap().typ.size(),
            NativeType::StructMemberSizeArithmetics(m) => m.native.exact_size().unwrap(),
        }
    }

    pub fn make_const(&mut self, value: usize) -> Result<(), String> {
        match self {
            Self::U8 => *self = Self::ConstU8(u8::try_from(value).or(Err(format!("Cannot convert {} to u8", value)))?),
            Self::U16 => *self = Self::ConstU16(u16::try_from(value).or(Err(format!("Cannot convert {} to u16", value)))?),
            Self::U24 => *self = Self::ConstU24(u32::try_from(value).or(Err(format!("Cannot convert {} to u24", value)))?),
            Self::U32 => *self = Self::ConstU32(u32::try_from(value).or(Err(format!("Cannot convert {} to u32", value)))?),
            Self::U64 => *self = Self::ConstU64(u64::try_from(value).or(Err(format!("Cannot convert {} to u64", value)))?),
            Self::Unknown(_) => return Err("unexpcted".into()),
            _ => panic!("cannot make const")
        }
        Ok(())
    }

    pub fn is_unsigned(&self) -> bool {
        match self {
            Self::U8 => true,
            Self::U16 => true,
            Self::U24 => true,
            Self::U32 => true,
            Self::U64 => true,
            _ => false
        }
    }

    pub fn no_swap(&self) -> Native {
        match self {
            Self::U8 => Native { typ: Self::NoSwapU8, endian: OverrideEndian::Default },
            Self::U16 => Native { typ: Self::NoSwapU16, endian: OverrideEndian::Default },
            Self::U24 => Native { typ: Self::NoSwapU24, endian: OverrideEndian::Default },
            Self::U32 => Native { typ: Self::NoSwapU32, endian: OverrideEndian::Default },
            Self::U64 => Native { typ: Self::NoSwapU64, endian: OverrideEndian::Default },
            _ => panic!("cannot make no-swap native type")
        }
    }
}

impl Default for NativeType {
    fn default() -> Self {
        Self::Bool
    }
}
