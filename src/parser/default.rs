use super::*;

impl Default for StructMemberConstant {
    fn default() -> Self {
        StructMemberConstant::No
    }
}

impl Default for SizeArithmetics {
    fn default() -> Self {
        SizeArithmetics::Usize(0)
    }
}

impl Default for BitArithmetic {
    fn default() -> Self {
        BitArithmetic::Value(0)
    }
}

impl Default for OverrideEndian {
    fn default() -> Self {
        Self::Default
    }
}