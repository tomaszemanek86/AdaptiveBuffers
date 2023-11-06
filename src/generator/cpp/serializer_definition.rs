use super::*;

impl SerializerDefinition for MemoryType {
    fn def_serializer_h(&self, writer: &mut Writer) {
        match self {
            Self::Enum(e) => e.def_serializer_h(writer),
            Self::Struct(s) => s.def_serializer_h(writer),
            Self::View(v) => v.def_serializer_h(writer),
            _ => (),
        }
    }

    fn def_serializer_cpp(&self, writer: &mut Writer) {
        match self {
            Self::Enum(e) => e.def_serializer_cpp(writer),
            Self::Struct(s) => s.def_serializer_cpp(writer),
            Self::View(v) => v.def_serializer_cpp(writer),
            _ => (),
        }
    }

    fn def_buffer_h(&self, writer: &mut Writer) {
        match self {
            Self::Enum(e) => e.def_buffer_h(writer),
            Self::Struct(s) => s.def_buffer_h(writer),
            Self::View(v) => v.def_buffer_h(writer),
            _ => (),
        }
    }
}

impl SerializerDefinition for MemoryDeclaration {
    fn def_serializer_h(&self, writer: &mut Writer) {
        self.memory.def_serializer_h(writer)
    }

    fn def_serializer_cpp(&self, writer: &mut Writer) {
        self.memory.def_serializer_cpp(writer)
    }

    fn def_buffer_h(&self, writer: &mut Writer) {
        self.memory.def_buffer_h(writer)
    }
}

impl SerializerDefinition for EnumMemory {
    fn def_serializer_h(&self, writer: &mut Writer) {
        writer.def_class(&self.as_serializer_typename());
        writer.scope_in();
        writer.public();
        snipets::def_serializer_ctor(self,false, writer);
        snipets::def_with_method_for_enum(self, false, writer);
        snipets::def_serialize_method(self, false, writer);
        writer.private();
        snipets::def_member_buffer_var(self, true, writer);
        writer.scope_out(true);
    }

    fn def_serializer_cpp(&self, writer: &mut Writer) {
        snipets::def_serializer_ctor(self, true, writer);
        snipets::def_with_method_for_enum(self, true, writer);
        snipets::def_serialize_method(self, true, writer);
        writer.scope_in();
        snipets::def_offset(writer);
        snipets::if_isset_is_throw(self, false, writer);
        snipets::serialize_copy(self, Some(self), writer);
        snipets::serialize_return(writer);
        writer.scope_out(false);
    }

    fn def_buffer_h(&self, writer: &mut Writer) {
        writer.def_struct(&self.as_buffer_typename());
        writer.scope_in();
        snipets::def_isset_member(self, writer);
        snipets::def_member_data_var(self, writer);
        writer.scope_out(true);
    }
}

impl SerializerDefinition for ViewMemory {
    fn def_serializer_h(&self, writer: &mut Writer) {
        writer.def_class(&self.as_serializer_typename());
        writer.scope_in();
        writer.public();
        snipets::def_serializer_ctor(self, false, writer);
        
        for (i, t) in self.types.iter().enumerate() {
            snipets::def_with_method_for_view(self, t, i, false, writer);
        }

        snipets::def_serialize_method(self, false, writer);
        writer.def_member_function("index_value", &[], Some(&NativeType::U16.as_typename()), None);
        writer.private();
        snipets::def_member_buffer_var(self, true, writer);
        writer.scope_out(true);
    }

    fn def_serializer_cpp(&self, writer: &mut Writer) {
        snipets::def_serializer_ctor(self, true, writer);
        for (i, t) in self.types.iter().enumerate() {
            snipets::def_with_method_for_view(self, t, i, true, writer);    
        }
        snipets::def_serialize_method(self, true, writer);
        writer.scope_in();
        snipets::def_offset(writer);
        snipets::if_isset_is_throw(self, false, writer);
        writer.write_with_offset(&format!("switch ({}.{}) ", self.member_buffer_variable(), self.member_index_variable()));
        writer.scope_in();
        for (i, t) in self.types.iter().enumerate() {
            writer.write_with_offset(&format!("case {}: ", i));
            writer.scope_in();
            snipets::serialize_copy(self, Some(t), writer);
            writer.scope_out(false);
        }
        writer.write_line(&format!("default: throw \"{} corrupted\";", self.as_typename()));
        writer.scope_out(false);
        snipets::serialize_return(writer);
        writer.scope_out(false);
        writer.def_member_function("index_value", &[], Some(&NativeType::U16.as_typename()), None);
        writer.scope_in();
        snipets::if_isset_is_throw(self, false, writer);
        writer.write_with_offset(&format!("switch ({}.{}) ", self.member_buffer_variable(), self.member_index_variable()));
        writer.scope_in();
        for (i, t) in self.types.iter().enumerate() {
            writer.write_line(&format!("case {}: return {};", i, t.constant.get_value()));
        }
        writer.write_line(&format!("default: throw \"{} corrupted\";", self.as_typename()));
        writer.scope_out(false);
        writer.scope_out(false);
    }

    fn def_buffer_h(&self, writer: &mut Writer) {
        writer.def_struct(&self.as_buffer_typename());
        writer.scope_in();
        snipets::def_isset_member(self, writer);
        snipets::def_index_member(self, writer);
        snipets::def_member_data_var(self, writer);
        writer.scope_out(true);
    }
}

impl SerializerDefinition for StructMemory {
    fn def_serializer_h(&self, writer: &mut Writer) {
        writer.def_class(&self.as_serializer_typename());
        writer.scope_in();
        writer.public();
        snipets::def_serializer_ctor(self, false, writer);
        for f in &self.fields {
            if f.memory.borrow().memory.is_reference() {
                continue;
            }
            snipets::def_with_method_for_struct(self, f, false, writer);
        }
        snipets::def_serialize_method(self, false, writer);
        writer.private();
        snipets::def_member_buffer_var(self, true, writer);
        writer.scope_out(true);
    }

    fn def_serializer_cpp(&self, writer: &mut Writer) {
        snipets::def_serializer_ctor(self, true, writer);
        
        for f in &self.fields {
            if f.memory.borrow().memory.is_reference() {
                continue;
            }
            snipets::def_with_method_for_struct(self, f, true, writer);
        }

        snipets::def_serialize_method(self, true, writer);
        writer.scope_in();
        snipets::def_offset(writer);
        for f in &self.fields {
            if f.memory.borrow().memory.is_reference() {
                continue;
            }
            if f.has_only_native_member() {
                snipets::if_isset_is_throw(f, false, writer);
            }
        }
        for f in &self.fields {
            let memory_type = &f.memory.borrow().memory;
            if memory_type.is_reference() {
                if let Some(member) = memory_type.referenced_array() {
                    writer.write_line(&format!("offset += std::memcpy(buffer + offset, &{}._size, {});", 
                        member.member_buffer_variable(), 
                        f.exact_size().unwrap()
                    ));
                } else if let Some(member) = memory_type.referenced_view() {
                    writer.write_line(&format!("*reinterpret_cast<{}*>(buffer + offset) = {}(&{}).index_value();",
                        f.as_typename(),
                        member.as_serializer_typename(),
                        member.member_buffer_variable()
                    ));
                    writer.write_line(&format!("offset += {};", f.exact_size().unwrap()));
                }
            } else {
                snipets::serialize_copy(f, Some(f), writer);
            }
        }
        snipets::serialize_return(writer);
        writer.scope_out(false);
    }

    fn def_buffer_h(&self, writer: &mut Writer) {
        writer.def_struct(&self.as_buffer_typename());
        writer.scope_in();
        for f in &self.fields {
            snipets::def_member_buffer_var(f, false, writer);
        }
        snipets::def_member_data_var(self, writer);
        writer.scope_out(true);
    }
}

impl SerializerDefinition for Memory {
    fn def_serializer_h(&self, writer: &mut Writer) {
        if let Some(max_size) = self.max_array_size {
            if self.memory.has_only_native_member() {
                writer.write_line(&format!("typedef core::MaxSizedArrayNativeSerializer<{}, {}, {}> {}", 
                    self.as_typename(),
                    max_size,
                    self.memory.exact_size().unwrap(),
                    self.as_max_array_serializer_typename()
                ));
            } else {
                writer.write_line(&format!("typedef core::MaxSizedArraySerializer<{}, {}, {}> {}", 
                    self.as_serializer_typename(),
                    max_size,
                    self.memory.exact_size().unwrap(),
                    self.as_max_array_serializer_typename()
                ));
            }
        } else {
            match self.memory {
                MemoryType::Struct(ref s) => s.def_serializer_h(writer),
                MemoryType::Enum(ref e) => e.def_serializer_h(writer),
                MemoryType::View(ref v) => v.def_serializer_h(writer),
                _ => (),
            }
        }
    }

    fn def_serializer_cpp(&self, writer: &mut Writer) {
        if let Some(max_size) = self.max_array_size {
            self.memory.def_serializer_cpp(writer)
        } else {
            match self.memory {
                MemoryType::Struct(ref s) => s.def_serializer_cpp(writer),
                MemoryType::Enum(ref e) => e.def_serializer_cpp(writer),
                MemoryType::View(ref v) => v.def_serializer_cpp(writer),
                _ => (),
            }
        }
    }

    fn def_buffer_h(&self, writer: &mut Writer) {
        match self.memory {
            MemoryType::Struct(ref s) => s.def_buffer_h(writer),
            MemoryType::Enum(ref e) => e.def_buffer_h(writer),
            MemoryType::View(ref v) => v.def_buffer_h(writer),
            _ => (),
        }
    }
}
