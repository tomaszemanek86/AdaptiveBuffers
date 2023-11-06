use super::*;

fn to_variable_name(name: &str, prefix: Option<&str>, postfix: Option<&str>) -> String {
    format!(
        "{}{}{}",
        prefix.unwrap_or(""),
        utils::to_snake_case(name),
        postfix.unwrap_or("")
    )
}

impl<T: CppDetails> CppDetails for std::rc::Rc<T> {
    fn as_typename(&self) -> String {
        self.as_ref().as_typename()
    }

    fn as_variable(&self, prefix: Option<&str>, postfix: Option<&str>) -> String {
        self.as_ref().as_variable(prefix, postfix)
    }

    fn as_pure_typename(&self) -> String {
        self.as_ref().as_pure_typename()
    }

    fn has_only_native_member(&self) -> bool {
        self.as_ref().has_only_native_member()
    }

    fn imports(&self) -> Vec<String> {
        self.as_ref().imports()
    }
}

impl<T: CppDetails> CppDetails for RefCell<T> {
    fn as_typename(&self) -> String {
        self.borrow().as_typename()
    }

    fn as_variable(&self, prefix: Option<&str>, postfix: Option<&str>) -> String {
        self.borrow().as_variable(prefix, postfix)
    }

    fn as_pure_typename(&self) -> String {
        self.borrow().as_pure_typename()
    }

    fn has_only_native_member(&self) -> bool {
        self.borrow().has_only_native_member()
    }

    fn imports(&self) -> Vec<String> {
        self.borrow().imports()
    }
}

impl CppDetails for MemoryDeclaration {
    fn as_typename(&self) -> String {
        self.memory.as_typename()
    }

    fn as_variable(&self, prefix: Option<&str>, postfix: Option<&str>) -> String {
        self.memory.as_variable(prefix, postfix)
    }

    fn as_pure_typename(&self) -> String {
        self.memory.as_pure_typename()
    }

    fn has_only_native_member(&self) -> bool {
        self.memory.has_only_native_member()
    }

    fn imports(&self) -> Vec<String> {
        self.memory.imports()
    }
}

impl CppDetails for Memory {
    fn as_typename(&self) -> String {
        if let Some(size) = self.max_array_size {
            format!("std::array<{}, {}>",self.memory.as_typename(), size)
        } else {
            self.memory.as_typename()
        }
    }

    fn as_variable(&self, prefix: Option<&str>, postfix: Option<&str>) -> String {
        if self.max_array_size.is_some() {
            if let Some(p) = postfix {
                self.memory.as_variable(prefix, Some(&format!("_array_{}", p)))
            } else {
                self.memory.as_variable(prefix, Some("_array"))
            }
        } else {
            match &self.memory {
                MemoryType::Struct(s) => s.as_variable(prefix, postfix),
                MemoryType::Enum(e) => e.as_variable(prefix, postfix),
                MemoryType::View(v) => v.as_variable(prefix, postfix),
                MemoryType::Native(n) => n.as_variable(prefix, postfix),
            }
        }
    }

    fn as_pure_typename(&self) -> String {
        if let Some(size) = self.max_array_size {
            self.memory.as_pure_typename() + &format!("_{}", size)
        } else {
            self.memory.as_pure_typename()
        }
    }

    fn has_only_native_member(&self) -> bool {
        self.memory.has_only_native_member()
    }

    fn imports(&self) -> Vec<String> {
        if self.max_array_size.is_some() {
            if self.memory.has_only_native_member() {
                return vec!["max_sized_native_array.h".to_string()]
            } else {
                return vec!["max_sized_array.h".to_string()]
            }
        }
        Default::default()
    }
}

impl CppDetails for MemoryType {
    fn as_typename(&self) -> String {
        match self {
            MemoryType::Struct(s) => s.as_typename(),
            MemoryType::Enum(e) => e.as_typename(),
            MemoryType::View(v) => v.as_typename(),
            MemoryType::Native(n) => n.as_typename(),
        }
    }

    fn as_variable(&self, prefix: Option<&str>, postfix: Option<&str>) -> String {
        match self {
            MemoryType::Struct(s) => s.as_variable(prefix, postfix),
            MemoryType::Enum(e) => e.as_variable(prefix, postfix),
            MemoryType::View(v) => v.as_variable(prefix, postfix),
            MemoryType::Native(n) => n.as_variable(prefix, postfix),
        }
    }

    fn as_pure_typename(&self) -> String {
        match self {
            MemoryType::Struct(s) => s.as_pure_typename(),
            MemoryType::Enum(e) => e.as_pure_typename(),
            MemoryType::View(v) => v.as_pure_typename(),
            MemoryType::Native(n) => n.as_pure_typename(),
        }
    }

    fn has_only_native_member(&self) -> bool {
        match self {
            MemoryType::Struct(s) => s.has_only_native_member(),
            MemoryType::Enum(e) => e.has_only_native_member(),
            MemoryType::View(v) => v.has_only_native_member(),
            MemoryType::Native(n) => n.has_only_native_member(),
        }
    }

    fn imports(&self) -> Vec<String> {
        match self {
            MemoryType::Struct(s) => s.imports(),
            MemoryType::Enum(e) => e.imports(),
            MemoryType::View(v) => v.imports(),
            MemoryType::Native(n) => n.imports(),
        }
    }
}

impl CppDetails for StructMemory {
    fn as_typename(&self) -> String {
        self.name.clone()
    }

    fn as_variable(&self, prefix: Option<&str>, postfix: Option<&str>) -> String {
        to_variable_name(&self.name, prefix, postfix)
    }

    fn as_pure_typename(&self) -> String {
        self.name.clone()
    }

    fn has_only_native_member(&self) -> bool {
        self.fields.iter().all(|f| f.memory.borrow().memory.is_native())
    }

    fn imports(&self) -> Vec<String> {
        self.fields.iter().flat_map(|f| f.memory.borrow().memory.imports()).collect()
    }
}

impl CppDetails for EnumMemory {
    fn as_typename(&self) -> String {
        self.name.clone()
    }

    fn as_variable(&self, prefix: Option<&str>, postfix: Option<&str>) -> String {
        to_variable_name(&self.name, prefix, postfix)
    }

    fn as_pure_typename(&self) -> String {
        self.name.clone()
    }

    fn has_only_native_member(&self) -> bool {
        true
    }

    fn imports(&self) -> Vec<String> {
        vec!["stdint.h".to_string()]
    }
}

impl CppDetails for ViewMemory {
    fn as_typename(&self) -> String {
        self.name.clone()
    }

    fn as_variable(&self, prefix: Option<&str>, postfix: Option<&str>) -> String {
        to_variable_name(&self.name, prefix, postfix)
    }

    fn as_pure_typename(&self) -> String {
        self.name.clone()
    }

    fn has_only_native_member(&self) -> bool {
        false
    }

    fn imports(&self) -> Vec<String> {
        vec!["stdint.h".to_string()]
    }
}

impl CppDetails for ViewPosibilityMemory {
    fn as_typename(&self) -> String {
        self.memory.as_typename()
    }

    fn as_variable(&self, prefix: Option<&str>, postfix: Option<&str>) -> String {
        self.memory.as_variable(prefix, postfix)
    }

    fn as_pure_typename(&self) -> String {
        self.memory.as_pure_typename()
    }

    fn has_only_native_member(&self) -> bool {
        self.memory.has_only_native_member()
    }

    fn imports(&self) -> Vec<String> {
        self.memory.imports()
    }
}

impl CppDetails for NativeType {
    fn as_typename(&self) -> String {
        match self {
            NativeType::Bool => "bool".to_string(),
            NativeType::I8 => "int8_t".to_string(),
            NativeType::I16 => "int16_t".to_string(),
            NativeType::I32 => "int32_t".to_string(),
            NativeType::I64 => "int64_t".to_string(),
            NativeType::U8 => "uint8_t".to_string(),
            NativeType::U16 => "uint16_t".to_string(),
            NativeType::U32 => "uint32_t".to_string(),
            NativeType::U64 => "uint64_t".to_string(),
            NativeType::Unknown => "uint32_t".to_string(),
            NativeType::ViewKeyReference(mr) => NativeType::from_max_number(mr.memory.borrow().memory.as_view().unwrap().types.len(), false).as_typename(),
            NativeType::ArrayDimensionReference(mr) => mr.memory.borrow().memory.as_native().unwrap().as_typename()
        }
    }

    fn as_variable(&self, prefix: Option<&str>, postfix: Option<&str>) -> String {
        match self {
            Self::U8 => to_variable_name("u8", prefix, postfix),
            Self::U16 => to_variable_name("u16", prefix, postfix),
            Self::U32 => to_variable_name("u32", prefix, postfix),
            Self::U64 => to_variable_name("u64", prefix, postfix),
            Self::I8 => to_variable_name("i8", prefix, postfix),
            Self::I16 => to_variable_name("i16", prefix, postfix),
            Self::I32 => to_variable_name("i32", prefix, postfix),
            Self::I64 => to_variable_name("i64", prefix, postfix),
            Self::Bool => to_variable_name("bool", prefix, postfix),
            Self::Unknown => to_variable_name("u32", prefix, postfix),
            Self::ViewKeyReference(m) => to_variable_name(&format!("{}Key", m.as_variable(None, None)), prefix, postfix),
            Self::ArrayDimensionReference(m) => to_variable_name(&format!("{}Dimension", m.as_variable(None, None)), prefix, postfix),
        }
    }

    fn as_pure_typename(&self) -> String {
        match self {
            Self::U8 => "U8".to_string(),
            Self::U16 => "U16".to_string(),
            Self::U32 => "U32".to_string(),
            Self::U64 => "U64".to_string(),
            Self::I8 => "I8".to_string(),
            Self::I16 => "I16".to_string(),
            Self::I32 => "I32".to_string(),
            Self::I64 => "I64".to_string(),
            Self::Bool => "Bool".to_string(),
            Self::Unknown => "U32".to_string(),
            NativeType::ViewKeyReference(mr) => NativeType::from_max_number(mr.memory.borrow().memory.as_view().unwrap().types.len(), false).as_pure_typename(),
            NativeType::ArrayDimensionReference(mr) => mr.memory.as_pure_typename()
        }
    }

    fn has_only_native_member(&self) -> bool {
        true
    }

    fn imports(&self) -> Vec<String> {
        match self {
            Self::Bool => vec![],
            _ => vec!["stdint.h".to_string()]
        }
    }
}

impl CppDetails for StructMemberMemory {
    fn as_typename(&self) -> String {
        self.memory.as_typename()
    }

    fn as_variable(&self, prefix: Option<&str>, postfix: Option<&str>) -> String {
        to_variable_name(&self.name, prefix, postfix)
    }

    fn as_pure_typename(&self) -> String {
        self.memory.as_pure_typename()
    }

    fn has_only_native_member(&self) -> bool {
        self.memory.borrow().memory.is_native()
    }

    fn imports(&self) -> Vec<String> {
        self.memory.borrow().memory.imports()
    }
}
