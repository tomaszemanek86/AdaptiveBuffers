use super::*;

impl CppMemoryDetail for MemoryDeclaration {
    fn name(&self) -> String {
        self.memory.memory.name()
    }

    fn user_value_serializable(&self) -> bool {
        self.memory.directly_deserializable()
    }

    fn directly_serializable(&self) -> bool {
        self.memory.directly_serializable()
    }

    fn directly_deserializable(&self) -> bool {
        self.memory.directly_deserializable()
    }

    fn serializer_typename(&self, protocol_endian: &EndianSettings) -> String {
        self.memory.serializer_typename(protocol_endian)    
    }

    fn deserializer_typename(&self, protocol_endian: &EndianSettings) -> String {
        self.memory.deserializer_typename(protocol_endian)
    }

    fn native_typename(&self) -> String {
        self.memory.native_typename()
    }

    fn bytes(&self) -> Option<u32> {
        self.memory.bytes()
    }

    fn default_constructible_deserializer(&self) -> bool {
        self.memory.default_constructible_deserializer()
    }
}

impl CppMemoryDetail for Memory {
    fn name(&self) -> String {
        self.memory.name()
    }

    fn user_value_serializable(&self) -> bool {
        match self.array_size {
            ArraySize::No => self.memory.user_value_serializable(),
            ArraySize::Dyn => true,
            ArraySize::Exact(_) => true,
        }
    }

    fn directly_deserializable(&self) -> bool {
        match self.array_size {
            ArraySize::No => self.memory.directly_deserializable(),
            ArraySize::Dyn => false,
            ArraySize::Exact(_) => false,
        }
    }

    fn directly_serializable(&self) -> bool {
        match self.array_size {
            ArraySize::No => self.memory.directly_serializable(),
            ArraySize::Dyn => false,
            ArraySize::Exact(_) => false,
        }
    }

    fn serializer_typename(&self, protocol_endian: &EndianSettings) -> String {
        match self.array_size {
            ArraySize::No => self.memory.serializer_typename(protocol_endian),
            ArraySize::Dyn => format!("abf::DynArraySerializer<{}>", self.memory.serializer_typename(protocol_endian)),
            ArraySize::Exact(s) => format!("abf::ArraySerializer<{}, {}>", self.memory.serializer_typename(protocol_endian), s),
        }
    }

    fn deserializer_typename(&self, protocol_endian: &EndianSettings) -> String {
        match self.array_size {
            ArraySize::No => self.memory.deserializer_typename(protocol_endian),
            ArraySize::Dyn => format!("abf::DynArrayDeserializer<{}>", self.memory.deserializer_typename(protocol_endian)),
            ArraySize::Exact(s) => format!("abf::ArrayDeserializer<{}, {}>", self.memory.deserializer_typename(protocol_endian), s),
        }
    }

    fn native_typename(&self) -> String {
        self.memory.native_typename()
    }

    fn bytes(&self) -> Option<u32> {
        match self.array_size {
            ArraySize::No => return self.memory.bytes(),
            ArraySize::Dyn => return None,
            ArraySize::Exact(s) => {
                if let Some(b) = self.memory.bytes() {
                    return Some(s * b)
                } else {
                    return None
                }
            }
        }
    }

    fn default_constructible_deserializer(&self) -> bool {
        match self.array_size {
            ArraySize::No => self.memory.default_constructible_deserializer(),
            ArraySize::Dyn => false,
            ArraySize::Exact(_) => false,
        }
    }
}

impl CppMemoryDetail for MemoryType {
    fn name(&self) -> String {
        match &self {
            MemoryType::Native(m) => m.name(),
            MemoryType::Struct(m) => m.borrow().name(),
            MemoryType::View(m) => m.name(),
            MemoryType::Enum(m) => m.name(),
            MemoryType::BitMask(m) => m.name(),
        }
    }

    fn user_value_serializable(&self) -> bool {
        match &self {
            MemoryType::Native(m) => m.user_value_serializable(),
            MemoryType::Struct(m) => m.borrow().user_value_serializable(),
            MemoryType::View(m) => m.user_value_serializable(),
            MemoryType::Enum(m) => m.user_value_serializable(),
            MemoryType::BitMask(m) => m.user_value_serializable(),
        }
    }

    fn directly_deserializable(&self) -> bool {
        match &self {
            MemoryType::Native(m) => m.directly_deserializable(),
            MemoryType::Struct(m) => m.borrow().directly_deserializable(),
            MemoryType::View(m) => m.directly_deserializable(),
            MemoryType::Enum(m) => m.directly_deserializable(),
            MemoryType::BitMask(m) => m.directly_deserializable(),
        }
    }

    fn directly_serializable(&self) -> bool {
        match &self {
            MemoryType::Native(m) => m.directly_serializable(),
            MemoryType::Struct(m) => m.borrow().directly_serializable(),
            MemoryType::View(m) => m.directly_serializable(),
            MemoryType::Enum(m) => m.directly_serializable(),
            MemoryType::BitMask(m) => m.directly_serializable(),
        }
    }

    fn serializer_typename(&self, protocol_endian: &EndianSettings) -> String {
        match &self {
            MemoryType::Native(m) => m.serializer_typename(protocol_endian),
            MemoryType::Struct(m) => m.borrow().serializer_typename(protocol_endian),
            MemoryType::View(m) => m.serializer_typename(protocol_endian),
            MemoryType::Enum(m) => m.serializer_typename(protocol_endian),
            MemoryType::BitMask(m) => m.serializer_typename(protocol_endian),
        }
    }

    fn deserializer_typename(&self, protocol_endian: &EndianSettings) -> String {
        match &self {
            MemoryType::Native(m) => m.deserializer_typename(protocol_endian),
            MemoryType::Struct(m) => m.borrow().deserializer_typename(protocol_endian),
            MemoryType::View(m) => m.deserializer_typename(protocol_endian),
            MemoryType::Enum(m) => m.deserializer_typename(protocol_endian),
            MemoryType::BitMask(m) => m.deserializer_typename(protocol_endian),
        }
    }

    fn native_typename(&self) -> String {
        match &self {
            MemoryType::Native(m) => m.native_typename(),
            MemoryType::Struct(m) => m.borrow().native_typename(),
            MemoryType::View(m) => m.native_typename(),
            MemoryType::Enum(m) => m.native_typename(),
            MemoryType::BitMask(m) => m.native_typename(),
        }
    }

    fn bytes(&self) -> Option<u32> {
        match &self {
            MemoryType::Native(m) => m.bytes(),
            MemoryType::Struct(m) => m.borrow().bytes(),
            MemoryType::View(m) => m.bytes(),
            MemoryType::Enum(m) => m.bytes(),
            MemoryType::BitMask(m) => m.bytes(),
        }
    }

    fn default_constructible_deserializer(&self) -> bool {
        match &self {
            MemoryType::Native(m) => m.default_constructible_deserializer(),
            MemoryType::Struct(m) => m.borrow().default_constructible_deserializer(),
            MemoryType::View(m) => m.default_constructible_deserializer(),
            MemoryType::Enum(m) => m.default_constructible_deserializer(),
            MemoryType::BitMask(m) => m.default_constructible_deserializer(),
        }
    }
}

fn resolve_copy_struct(endian_settings: &EndianSettings, override_endian: &OverrideEndian) -> &'static str {

    let target_protocol_endian_big = if endian_settings.protocol_big {
        !override_endian.is_little_endian()
    } else {
        !override_endian.is_big_endian()
    };

    if target_protocol_endian_big {
        if endian_settings.protocol_big {
            return "abf::Copy"
        } else {
            return "abf::ByteSwapCopy"
        }
    } else {
        if endian_settings.protocol_big {
            return "abf::ByteSwapCopy"
        } else {
            return "abf::Copy"
        }
    }
}

impl CppMemoryDetail for Native {
    fn name(&self) -> String {
        match &self.typ {
            NativeType::Bool => "b".into(),
            NativeType::U8 => "u8".into(),
            NativeType::U16 => "u16".into(),
            NativeType::U24 => "u24".into(),
            NativeType::U32 => "u32".into(),
            NativeType::U64 => "u64".into(),
            NativeType::NoSwapU8 => "u8".into(),
            NativeType::NoSwapU16 => "u16".into(),
            NativeType::NoSwapU24 => "u24".into(),
            NativeType::NoSwapU32 => "u32".into(),
            NativeType::NoSwapU64 => "u64".into(),
            NativeType::ConstU8(_) => "cu8".into(),
            NativeType::ConstU16(_) => "cu16".into(),
            NativeType::ConstU24(_) => "cu24".into(),
            NativeType::ConstU32(_) => "cu32".into(),
            NativeType::ConstU64(_) => "cu64".into(),
            NativeType::I8 => "i8".into(),
            NativeType::I16 => "i16".into(),
            NativeType::I32 => "i32".into(),
            NativeType::I64 => "i64".into(),
            NativeType::Unknown(_) => panic!("unknown type"),
            NativeType::ViewKeyReference(m) => m.native_key.name(),
            NativeType::ArrayDimensionReference(r) => r.origin.as_ref().name(),
            NativeType::StructMemberSize(m) => m.origin.name(),
            NativeType::StructMemberSizeArithmetics(m) => m.native.name(),
        }
    }
    fn user_value_serializable(&self) -> bool {
        match &self.typ {
            NativeType::Unknown(_) => panic!("unknown type"),
            NativeType::ViewKeyReference(_) => false,
            NativeType::ArrayDimensionReference(_) => false,
            NativeType::ConstU8(_) => false,
            NativeType::ConstU16(_) => false,
            NativeType::ConstU24(_) => false,
            NativeType::ConstU32(_) => false,
            NativeType::ConstU64(_) => false,
            NativeType::StructMemberSize(_) => false,
            NativeType::StructMemberSizeArithmetics(_) => false,
            _ => true
        }
    }
    fn directly_serializable(&self) -> bool {
        true
    }
    fn directly_deserializable(&self) -> bool {
        true
    }
    fn serializer_typename(&self, protocol_endian: &EndianSettings) -> String {
        let copy = resolve_copy_struct(protocol_endian, &self.endian);
        match &self.typ {
            NativeType::Bool => format!("abf::NativeSerializer<bool, 1, {}>", copy),
            NativeType::U8 => format!("abf::NativeSerializer<uint8_t, 1, {}>", copy),
            NativeType::U16 => format!("abf::NativeSerializer<uint16_t, 2, {}>", copy),
            NativeType::U24 => format!("abf::NativeSerializer<uint32_t, 3, {}>", copy),
            NativeType::U32 => format!("abf::NativeSerializer<uint32_t, 4, {}>", copy),
            NativeType::U64 => format!("abf::NativeSerializer<uint64_t, 8, {}>", copy),
            NativeType::NoSwapU8 => format!("abf::NativeNoSwapSerializer<uint8_t, 1>"),
            NativeType::NoSwapU16 => format!("abf::NativeNoSwapSerializer<uint16_t, 2>"),
            NativeType::NoSwapU24 => format!("abf::NativeNoSwapSerializer<uint32_t, 3>"),
            NativeType::NoSwapU32 => format!("abf::NativeNoSwapSerializer<uint32_t, 4>"),
            NativeType::NoSwapU64 => format!("abf::NativeNoSwapSerializer<uint64_t, 8>"),
            NativeType::ConstU8(v) => format!("abf::ConstantSerializer<abf::NativeSerializer<uint8_t, 1, {}>, uint8_t, {}>", copy, v),
            NativeType::ConstU16(v) => format!("abf::ConstantSerializer<abf::NativeSerializer<uint16_t, 2, {}>, uint16_t, {}>", copy, v),
            NativeType::ConstU24(v) => format!("abf::ConstantSerializer<abf::NativeSerializer<uint32_t, 3, {}>, uint32_t, {}>", copy, v),
            NativeType::ConstU32(v) => format!("abf::ConstantSerializer<abf::NativeSerializer<uint32_t, 4, {}>, uint32_t, {}>", copy, v),
            NativeType::ConstU64(v) => format!("abf::ConstantSerializer<abf::NativeSerializer<uint64_t, 8, {}>, uint64_t, {}>", copy, v),
            NativeType::I8 => format!("abf::NativeSerializer<int8_t, 1, {}>", copy),
            NativeType::I16 => format!("abf::NativeSerializer<int16_t, 2, {}>", copy),
            NativeType::I32 => format!("abf::NativeSerializer<int32_t, 4, {}>", copy),
            NativeType::I64 => format!("abf::NativeSerializer<int64_t, 8, {}>", copy),
            NativeType::Unknown(_) => panic!("unknown type"),
            NativeType::ViewKeyReference(m) => format!("abf::ViewKeySerializer<{}, {}, {}>", m.native_key.native_typename(), m.native_key.bytes().unwrap(), copy),
            NativeType::ArrayDimensionReference(r) => format!("abf::LazySerializer<{}>", r.origin.as_ref().serializer_typename(protocol_endian)),
            NativeType::StructMemberSize(m) => format!("abf::LazySerializer<{}>", m.native.serializer_typename(protocol_endian)),
            NativeType::StructMemberSizeArithmetics(m) => m.native.serializer_typename(protocol_endian),
        }
    }
    fn deserializer_typename(&self, protocol_endian: &EndianSettings) -> String {
        let copy = resolve_copy_struct(protocol_endian, &self.endian);
        match &self.typ {
            NativeType::Bool => format!("abf::NativeDeserializer<bool, 1, {}>", copy),
            NativeType::U8 => format!("abf::NativeDeserializer<uint8_t, 1, {}>", copy),
            NativeType::U16 => format!("abf::NativeDeserializer<uint16_t, 2, {}>", copy),
            NativeType::U24 => format!("abf::NativeDeserializer<uint32_t, 3, {}>", copy),
            NativeType::U32 => format!("abf::NativeDeserializer<uint32_t, 4, {}>", copy),
            NativeType::U64 => format!("abf::NativeDeserializer<uint64_t, 8, {}>", copy),
            NativeType::NoSwapU8 => format!("abf::NativeNoSwapDeserializer<uint8_t, 1>"),
            NativeType::NoSwapU16 => format!("abf::NativeNoSwapDeserializer<uint16_t, 2>"),
            NativeType::NoSwapU24 => format!("abf::NativeNoSwapDeserializer<uint32_t, 3>"),
            NativeType::NoSwapU32 => format!("abf::NativeNoSwapDeserializer<uint32_t, 4>"),
            NativeType::NoSwapU64 => format!("abf::NativeNoSwapDeserializer<uint64_t, 8>"),
            NativeType::ConstU8(_) => format!("abf::NativeDeserializer<uint8_t, 1, {}>", copy),
            NativeType::ConstU16(_) => format!("abf::NativeDeserializer<uint16_t, 2, {}>", copy),
            NativeType::ConstU24(_) => format!("abf::NativeDeserializer<uint32_t, 3, {}>", copy),
            NativeType::ConstU32(_) => format!("abf::NativeDeserializer<uint32_t, 4, {}>", copy),
            NativeType::ConstU64(_) => format!("abf::NativeDeserializer<uint64_t, 8, {}>", copy),
            NativeType::I8 => format!("abf::NativeDeserializer<int8_t, 1, {}>", copy),
            NativeType::I16 => format!("abf::NativeDeserializer<int16_t, 2, {}>", copy),
            NativeType::I32 => format!("abf::NativeDeserializer<int32_t, 4, {}>", copy),
            NativeType::I64 => format!("abf::NativeDeserializer<int64_t, 8, {}>", copy),
            NativeType::Unknown(_) => panic!("unknown type"),
            NativeType::ViewKeyReference(m) => m.native_key.deserializer_typename(protocol_endian),
            NativeType::ArrayDimensionReference(r) => r.origin.deserializer_typename(protocol_endian),
            NativeType::StructMemberSize(m) => m.native.deserializer_typename(protocol_endian),
            NativeType::StructMemberSizeArithmetics(m) => m.native.deserializer_typename(protocol_endian),
        }
    }
    fn native_typename(&self) -> String {
        match &self.typ {
            NativeType::Bool => "bool".into(),
            NativeType::U8 => "uint8_t".into(),
            NativeType::U16 => "uint16_t".into(),
            NativeType::U24 => "uint32_t".into(),
            NativeType::U32 => "uint32_t".into(),
            NativeType::U64 => "uint64_t".into(),
            NativeType::NoSwapU8 => "uint8_t".into(),
            NativeType::NoSwapU16 => "uint16_t".into(),
            NativeType::NoSwapU24 => "uint32_t".into(),
            NativeType::NoSwapU32 => "uint32_t".into(),
            NativeType::NoSwapU64 => "uint64_t".into(),
            NativeType::ConstU8(_) => "uint8_t".into(),
            NativeType::ConstU16(_) => "uint16_t".into(),
            NativeType::ConstU24(_) => "uint32_t".into(),
            NativeType::ConstU32(_) => "uint32_t".into(),
            NativeType::ConstU64(_) => "uint64_t".into(),
            NativeType::I8 => "int8_t".into(),
            NativeType::I16 => "int16_t".into(),
            NativeType::I32 => "int32_t".into(),
            NativeType::I64 => "int64_t".into(),
            NativeType::Unknown(_) => panic!("unknown type"),
            NativeType::ViewKeyReference(m) => m.native_key.native_typename(),
            NativeType::ArrayDimensionReference(r) => r.origin.native_typename(),
            NativeType::StructMemberSize(m) => m.native.native_typename(),
            NativeType::StructMemberSizeArithmetics(m) => m.native.native_typename(),
        }
    }
    fn bytes(&self) -> Option<u32> {
        match &self.typ {
            NativeType::Bool => Some(1),
            NativeType::U8 => Some(1),
            NativeType::U16 => Some(2),
            NativeType::U24 => Some(3),
            NativeType::U32 => Some(4),
            NativeType::U64 => Some(8),
            NativeType::NoSwapU8 => Some(1),
            NativeType::NoSwapU16 => Some(2),
            NativeType::NoSwapU24 => Some(3),
            NativeType::NoSwapU32 => Some(4),
            NativeType::NoSwapU64 => Some(8),
            NativeType::ConstU8(_) => Some(1),
            NativeType::ConstU16(_) => Some(2),
            NativeType::ConstU24(_) => Some(3),
            NativeType::ConstU32(_) => Some(4),
            NativeType::ConstU64(_) => Some(8),
            NativeType::I8 => Some(1),
            NativeType::I16 => Some(2),
            NativeType::I32 => Some(4),
            NativeType::I64 => Some(8),
            NativeType::Unknown(_) => panic!("unknown type"),
            NativeType::ViewKeyReference(m) => m.native_key.bytes(),
            NativeType::ArrayDimensionReference(r) => r.origin.bytes(),
            NativeType::StructMemberSize(m) => m.native.bytes(),
            NativeType::StructMemberSizeArithmetics(m) => m.native.bytes(),
        }
    }
    fn default_constructible_deserializer(&self) -> bool {
        true
    }
}

impl CppMemoryDetail for StructMemory {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn user_value_serializable(&self) -> bool {
        true
    }
    fn directly_serializable(&self) -> bool {
        false
    }
    fn directly_deserializable(&self) -> bool {
        false
    }
    fn serializer_typename(&self, _protocol_endian: &EndianSettings) -> String {
        format!("{}Ser", self.name())
    }
    fn deserializer_typename(&self, _protocol_endian: &EndianSettings) -> String {
        format!("{}De", self.name())
    }
    fn native_typename(&self) -> String {
        panic!("native struct not supported yet")
    }
    fn bytes(&self) -> Option<u32> {
        let mut size: Option<u32> = Some(0);
        for f in &self.fields {
            if let Some(b) = f.as_ref().bytes() {
                size = Some(b + size.unwrap())
            } else {
                return None
            }
        }
        size
    }
    fn default_constructible_deserializer(&self) -> bool {
        true
    }
}

impl CppMemoryDetail for ViewMemory {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn user_value_serializable(&self) -> bool {
        true
    }
    fn directly_serializable(&self) -> bool {
        false
    }
    fn directly_deserializable(&self) -> bool {
        false
    }
    fn serializer_typename(&self, _protocol_endian: &EndianSettings) -> String {
        format!("{}Ser", self.name())
    }
    fn deserializer_typename(&self, _protocol_endian: &EndianSettings) -> String {
        format!("{}De", self.name())
    }
    fn native_typename(&self) -> String {
        panic!("native view not supported yet")
    }
    fn bytes(&self) -> Option<u32> {
        if self.types.iter().all(|t| t.bytes() == self.types[0].bytes()) {
            self.types[0].bytes()
        } else {
            None
        }
    }
    fn default_constructible_deserializer(&self) -> bool {
        true
    }
}

impl CppMemoryDetail for EnumMemory {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn user_value_serializable(&self) -> bool {
        true
    }
    fn directly_serializable(&self) -> bool {
        true
    }
    fn directly_deserializable(&self) -> bool {
        true
    }
    fn serializer_typename(&self, _protocol_endian: &EndianSettings) -> String {
        format!("{}Ser", self.name)
    }
    fn deserializer_typename(&self, _protocol_endian: &EndianSettings) -> String {
        format!("{}De", self.name)
    }
    fn native_typename(&self) -> String {
        self.name.clone()
    }
    fn bytes(&self) -> Option<u32> {
        self.underlaying_type.bytes()
    }
    fn default_constructible_deserializer(&self) -> bool {
        true
    }
}

impl CppMemoryDetail for StructMemberMemory {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn user_value_serializable(&self) -> bool {
        self.memory.borrow().user_value_serializable()
    }
    fn directly_serializable(&self) -> bool {
        self.memory.borrow().directly_serializable()
    }
    fn directly_deserializable(&self) -> bool {
        self.memory.borrow().directly_deserializable()
    }
    fn serializer_typename(&self, protocol_endian: &EndianSettings) -> String {
        let m = self.memory.borrow();
        if let Some(size_member) = self.get_array_size_reference() {
            let size_member_nt = size_member.as_ref().memory.borrow().memory.as_native().unwrap().clone();
            if size_member_nt.bytes().unwrap() != 32 {
                return format!("abf::ArraySizedSerializer<{}, {}>", m.serializer_typename(protocol_endian), size_member.serializer_typename(protocol_endian));
            }
        }
        m.serializer_typename(protocol_endian)
    }
    fn deserializer_typename(&self, protocol_endian: &EndianSettings) -> String {
        let m = self.memory.borrow();
        if let Some(size_member) = self.get_array_size_reference() {
            let size_member_nt = size_member.as_ref().memory.borrow().memory.as_native().unwrap().clone();
            if size_member_nt.bytes().unwrap() != 32 {
                return format!("abf::ArraySizedDeserializer<{}, {}>", m.deserializer_typename(protocol_endian), size_member.deserializer_typename(protocol_endian));
            }
        }
        self.memory.borrow().deserializer_typename(protocol_endian)
    }
    fn native_typename(&self) -> String {
        self.memory.borrow().native_typename()
    }
    fn bytes(&self) -> Option<u32> {
        self.memory.borrow().bytes()
    }
    fn default_constructible_deserializer(&self) -> bool {
        self.memory.borrow().default_constructible_deserializer()
    }
}

impl CppMemoryDetail for ViewPosibilityMemory {
    fn name(&self) -> String {
        self.memory.name()
    }
    fn user_value_serializable(&self) -> bool {
        self.memory.user_value_serializable()
    }
    fn directly_serializable(&self) -> bool {
        self.memory.directly_serializable()
    }
    fn directly_deserializable(&self) -> bool {
        self.memory.directly_deserializable()
    }
    fn serializer_typename(&self, protocol_endian: &EndianSettings) -> String {
        self.memory.serializer_typename(protocol_endian)
    }
    fn deserializer_typename(&self, protocol_endian: &EndianSettings) -> String {
        self.memory.deserializer_typename(protocol_endian)
    }
    fn native_typename(&self) -> String {
        self.memory.native_typename()
    }
    fn bytes(&self) -> Option<u32> {
        self.memory.bytes()
    }
    fn default_constructible_deserializer(&self) -> bool {
        self.memory.default_constructible_deserializer()
    }
}

impl CppMemoryDetail for BitMask {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn user_value_serializable(&self) -> bool {
        false
    }

    fn directly_serializable(&self) -> bool {
        false
    }

    fn directly_deserializable(&self) -> bool {
        false
    }

    fn serializer_typename(&self, _protocol_endian: &EndianSettings) -> String {
        format!("{}Ser", self.name)
    }

    fn deserializer_typename(&self, _protocol_endian: &EndianSettings) -> String {
        format!("{}De", self.name)
    }

    fn native_typename(&self) -> String {
        self.native.native_typename()
    }

    fn bytes(&self) -> Option<u32> {
        self.native.bytes()
    }

    fn default_constructible_deserializer(&self) -> bool {
        self.native.default_constructible_deserializer()
    }
}
