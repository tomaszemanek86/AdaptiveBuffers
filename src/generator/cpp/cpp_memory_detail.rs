use super::*;

impl CppMemoryDetail for MemoryDeclaration {
    fn name(&self) -> String {
        self.memory.memory.name()
    }

    fn user_value_serializable(&self) -> bool {
        self.memory.directly_deserializable()
    }

    fn directly_deserializable(&self) -> bool {
        self.memory.directly_deserializable()
    }

    fn serializer_typename(&self) -> String {
        self.memory.serializer_typename()    
    }

    fn deserializer_typename(&self) -> String {
        self.memory.deserializer_typename()
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

    fn serializer_typename(&self) -> String {
        match self.array_size {
            ArraySize::No => self.memory.serializer_typename(),
            ArraySize::Dyn => format!("abf::DynArraySerializer<{}>", self.memory.serializer_typename()),
            ArraySize::Exact(s) => format!("abf::ArraySerializer<{}, {}>", self.memory.serializer_typename(), s),
        }
    }

    fn deserializer_typename(&self) -> String {
        match self.array_size {
            ArraySize::No => self.memory.deserializer_typename(),
            ArraySize::Dyn => format!("abf::DynArrayDeserializer<{}>", self.memory.deserializer_typename()),
            ArraySize::Exact(s) => format!("abf::ArrayDeserializer<{}, {}>", self.memory.deserializer_typename(), s),
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
        }
    }

    fn user_value_serializable(&self) -> bool {
        match &self {
            MemoryType::Native(m) => m.user_value_serializable(),
            MemoryType::Struct(m) => m.borrow().user_value_serializable(),
            MemoryType::View(m) => m.user_value_serializable(),
            MemoryType::Enum(m) => m.user_value_serializable(),
        }
    }

    fn directly_deserializable(&self) -> bool {
        match &self {
            MemoryType::Native(m) => m.directly_deserializable(),
            MemoryType::Struct(m) => m.borrow().directly_deserializable(),
            MemoryType::View(m) => m.directly_deserializable(),
            MemoryType::Enum(m) => m.directly_deserializable(),
        }
    }

    fn serializer_typename(&self) -> String {
        match &self {
            MemoryType::Native(m) => m.serializer_typename(),
            MemoryType::Struct(m) => m.borrow().serializer_typename(),
            MemoryType::View(m) => m.serializer_typename(),
            MemoryType::Enum(m) => m.serializer_typename(),
        }
    }

    fn deserializer_typename(&self) -> String {
        match &self {
            MemoryType::Native(m) => m.deserializer_typename(),
            MemoryType::Struct(m) => m.borrow().deserializer_typename(),
            MemoryType::View(m) => m.deserializer_typename(),
            MemoryType::Enum(m) => m.deserializer_typename(),
        }
    }

    fn native_typename(&self) -> String {
        match &self {
            MemoryType::Native(m) => m.native_typename(),
            MemoryType::Struct(m) => m.borrow().native_typename(),
            MemoryType::View(m) => m.native_typename(),
            MemoryType::Enum(m) => m.native_typename(),
        }
    }

    fn bytes(&self) -> Option<u32> {
        match &self {
            MemoryType::Native(m) => m.bytes(),
            MemoryType::Struct(m) => m.borrow().bytes(),
            MemoryType::View(m) => m.bytes(),
            MemoryType::Enum(m) => m.bytes(),
        }
    }

    fn default_constructible_deserializer(&self) -> bool {
        match &self {
            MemoryType::Native(m) => m.default_constructible_deserializer(),
            MemoryType::Struct(m) => m.borrow().default_constructible_deserializer(),
            MemoryType::View(m) => m.default_constructible_deserializer(),
            MemoryType::Enum(m) => m.default_constructible_deserializer(),
        }
    }
}

impl CppMemoryDetail for NativeType {
    fn name(&self) -> String {
        match self {
            NativeType::Bool => "b".into(),
            NativeType::U8 => "u8".into(),
            NativeType::U16 => "u16".into(),
            NativeType::U24 => "u24".into(),
            NativeType::U32 => "u32".into(),
            NativeType::U64 => "u64".into(),
            NativeType::ConstU8(_) => "cu8".into(),
            NativeType::ConstU16(_) => "cu16".into(),
            NativeType::ConstU24(_) => "cu24".into(),
            NativeType::ConstU32(_) => "cu32".into(),
            NativeType::ConstU64(_) => "cu64".into(),
            NativeType::I8 => "i8".into(),
            NativeType::I16 => "i16".into(),
            NativeType::I32 => "i32".into(),
            NativeType::I64 => "i64".into(),
            NativeType::Unknown => panic!("unknown type"),
            NativeType::ViewKeyReference(m) => m.native_key.name(),
            NativeType::ArrayDimensionReference(r) => r.origin.as_ref().name(),
            NativeType::StructMemberSize(m) => m.origin.name(),
        }
    }
    fn user_value_serializable(&self) -> bool {
        match self {
            NativeType::Unknown => panic!("unknown type"),
            NativeType::ViewKeyReference(_) => false,
            NativeType::ArrayDimensionReference(_) => false,
            NativeType::ConstU8(_) => false,
            NativeType::ConstU16(_) => false,
            NativeType::ConstU24(_) => false,
            NativeType::ConstU32(_) => false,
            NativeType::ConstU64(_) => false,
            NativeType::StructMemberSize(_) => false,
            _ => true
        }
    }
    fn directly_deserializable(&self) -> bool {
        true
    }
    fn serializer_typename(&self) -> String {
        match self {
            NativeType::Bool => "abf::NativeSerializer<bool, 1>".into(),
            NativeType::U8 => "abf::NativeSerializer<uint8_t, 1>".into(),
            NativeType::U16 => "abf::NativeSerializer<uint16_t, 2>".into(),
            NativeType::U24 => "abf::NativeSerializer<uint32_t, 3>".into(),
            NativeType::U32 => "abf::NativeSerializer<uint32_t, 4>".into(),
            NativeType::U64 => "abf::NativeSerializer<uint64_t, 8>".into(),
            NativeType::ConstU8(v) => format!("abf::ConstantSerializer<abf::NativeSerializer<uint8_t, 1>, uint8_t, {}>", v),
            NativeType::ConstU16(v) => format!("abf::ConstantSerializer<abf::NativeSerializer<uint16_t, 2>, uint16_t, {}>", v),
            NativeType::ConstU24(v) => format!("abf::ConstantSerializer<abf::NativeSerializer<uint32_t, 3>, uint32_t, {}>", v),
            NativeType::ConstU32(v) => format!("abf::ConstantSerializer<abf::NativeSerializer<uint32_t, 4>, uint32_t, {}>", v),
            NativeType::ConstU64(v) => format!("abf::ConstantSerializer<abf::NativeSerializer<uint64_t, 8>, uint64_t, {}>", v),
            NativeType::I8 => "abf::NativeSerializer<int8_t, 1>".into(),
            NativeType::I16 => "abf::NativeSerializer<int16_t, 2>".into(),
            NativeType::I32 => "abf::NativeSerializer<int32_t, 4>".into(),
            NativeType::I64 => "abf::NativeSerializer<int64_t, 8>".into(),
            NativeType::Unknown => panic!("unknown type"),
            NativeType::ViewKeyReference(m) => format!("abf::ViewKeySerializer<{}, {}>", m.native_key.native_typename(), m.native_key.bytes().unwrap()),
            NativeType::ArrayDimensionReference(r) => format!("abf::LazySerializer<{}>", r.origin.as_ref().serializer_typename()),
            NativeType::StructMemberSize(m) => format!("abf::LazySerializer<{}>", m.native.serializer_typename()),
        }
    }
    fn deserializer_typename(&self) -> String {
        match self {
            NativeType::Bool => "abf::NativeDeserializer<bool, 1>".into(),
            NativeType::U8 => "abf::NativeDeserializer<uint8_t, 1>".into(),
            NativeType::U16 => "abf::NativeDeserializer<uint16_t, 2>".into(),
            NativeType::U24 => "abf::NativeDeserializer<uint32_t, 3>".into(),
            NativeType::U32 => "abf::NativeDeserializer<uint32_t, 4>".into(),
            NativeType::U64 => "abf::NativeDeserializer<uint64_t, 8>".into(),
            NativeType::ConstU8(_) => "abf::NativeDeserializer<uint8_t, 1>".into(),
            NativeType::ConstU16(_) => "abf::NativeDeserializer<uint16_t, 2>".into(),
            NativeType::ConstU24(_) => "abf::NativeDeserializer<uint32_t, 3>".into(),
            NativeType::ConstU32(_) => "abf::NativeDeserializer<uint32_t, 4>".into(),
            NativeType::ConstU64(_) => "abf::NativeDeserializer<uint64_t, 8>".into(),
            NativeType::I8 => "abf::NativeDeserializer<int8_t, 1>".into(),
            NativeType::I16 => "abf::NativeDeserializer<int16_t, 2>".into(),
            NativeType::I32 => "abf::NativeDeserializer<int32_t, 4>".into(),
            NativeType::I64 => "abf::NativeDeserializer<int64_t, 8>".into(),
            NativeType::Unknown => panic!("unknown type"),
            NativeType::ViewKeyReference(m) => m.native_key.deserializer_typename(),
            NativeType::ArrayDimensionReference(r) => r.origin.deserializer_typename(),
            NativeType::StructMemberSize(m) => m.native.deserializer_typename(),
        }
    }
    fn native_typename(&self) -> String {
        match self {
            NativeType::Bool => "bool".into(),
            NativeType::U8 => "uint8_t".into(),
            NativeType::U16 => "uint16_t".into(),
            NativeType::U24 => "uint32_t".into(),
            NativeType::U32 => "uint32_t".into(),
            NativeType::U64 => "uint64_t".into(),
            NativeType::ConstU8(_) => "uint8_t".into(),
            NativeType::ConstU16(_) => "uint16_t".into(),
            NativeType::ConstU24(_) => "uint32_t".into(),
            NativeType::ConstU32(_) => "uint32_t".into(),
            NativeType::ConstU64(_) => "uint64_t".into(),
            NativeType::I8 => "int8_t".into(),
            NativeType::I16 => "int16_t".into(),
            NativeType::I32 => "int32_t".into(),
            NativeType::I64 => "int64_t".into(),
            NativeType::Unknown => panic!("unknown type"),
            NativeType::ViewKeyReference(m) => m.native_key.native_typename(),
            NativeType::ArrayDimensionReference(r) => r.origin.native_typename(),
            NativeType::StructMemberSize(m) => m.native.native_typename(),
        }
    }
    fn bytes(&self) -> Option<u32> {
        match self {
            NativeType::Bool => Some(1),
            NativeType::U8 => Some(1),
            NativeType::U16 => Some(2),
            NativeType::U24 => Some(3),
            NativeType::U32 => Some(4),
            NativeType::U64 => Some(8),
            NativeType::ConstU8(_) => Some(1),
            NativeType::ConstU16(_) => Some(2),
            NativeType::ConstU24(_) => Some(3),
            NativeType::ConstU32(_) => Some(4),
            NativeType::ConstU64(_) => Some(8),
            NativeType::I8 => Some(1),
            NativeType::I16 => Some(2),
            NativeType::I32 => Some(4),
            NativeType::I64 => Some(8),
            NativeType::Unknown => panic!("unknown type"),
            NativeType::ViewKeyReference(m) => m.native_key.bytes(),
            NativeType::ArrayDimensionReference(r) => r.origin.bytes(),
            NativeType::StructMemberSize(m) => m.native.bytes(),
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
    fn directly_deserializable(&self) -> bool {
        false
    }
    fn serializer_typename(&self) -> String {
        format!("{}Ser", self.name())
    }
    fn deserializer_typename(&self) -> String {
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
    fn directly_deserializable(&self) -> bool {
        false
    }
    fn serializer_typename(&self) -> String {
        format!("{}Ser", self.name())
    }
    fn deserializer_typename(&self) -> String {
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
    fn directly_deserializable(&self) -> bool {
        true
    }
    fn serializer_typename(&self) -> String {
        format!("{}Ser", self.name)
    }
    fn deserializer_typename(&self) -> String {
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
    fn directly_deserializable(&self) -> bool {
        self.memory.borrow().directly_deserializable()
    }
    fn serializer_typename(&self) -> String {
        let m = self.memory.borrow();
        if let Some(size_member) = self.get_array_size_reference() {
            let size_member_nt = size_member.as_ref().memory.borrow().memory.as_native().unwrap().clone();
            if size_member_nt.bytes().unwrap() != 32 {
                return format!("abf::ArraySizedSerializer<{}, {}>", m.serializer_typename(), size_member.serializer_typename());
            }
        }
        m.serializer_typename()
    }
    fn deserializer_typename(&self) -> String {
        let m = self.memory.borrow();
        if let Some(size_member) = self.get_array_size_reference() {
            let size_member_nt = size_member.as_ref().memory.borrow().memory.as_native().unwrap().clone();
            if size_member_nt.bytes().unwrap() != 32 {
                return format!("abf::ArraySizedDeserializer<{}, {}>", m.deserializer_typename(), size_member.deserializer_typename());
            }
        }
        self.memory.borrow().deserializer_typename()
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
    fn directly_deserializable(&self) -> bool {
        self.memory.directly_deserializable()
    }
    fn serializer_typename(&self) -> String {
        self.memory.serializer_typename()
    }
    fn deserializer_typename(&self) -> String {
        self.memory.deserializer_typename()
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
