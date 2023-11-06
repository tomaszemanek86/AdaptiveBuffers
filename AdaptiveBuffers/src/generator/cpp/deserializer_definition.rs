use std::collections::HashSet;

use super::*;

impl DeserializerDefinition for EnumMemory {
    fn def_deserializer_h(&self, writer: &mut Writer) {
        writer.def_class(&self.as_deserializer_typename());
        writer.scope_in();
        writer.public();
        snipets::def_deserializer_ctor(self, false, writer);
        snipets::def_deserializer_get(self, self, false, writer);
        writer.private();
        snipets::def_reader(writer);
        snipets::def_member_context_var(self, writer);
        writer.scope_out(true);
    }

    fn def_deserializer_cpp(&self, writer: &mut Writer) {
        snipets::def_deserializer_ctor(self, true, writer);
        writer.scope_in();
        writer.write_line(&format!("{}.set_size({});", 
            self.member_context_variable(), 
            self.exact_size().unwrap()));
        writer.scope_out(false);
        snipets::def_deserializer_get(self, self, true, writer);
        writer.scope_in();
        snipets::if_previous_not_set_throw(self, writer);
        snipets::read_and_return_value(self, None, self, writer);
        writer.scope_out(false);
    }

    fn def_context_h(&self, writer: &mut Writer) {
        writer.def_struct(&self.as_context_typename());
        writer.scope_in();
        snipets::def_context_ctor(self, true, &[], false, writer);
        writer.def_member_var(&self.member_context_variable(), &self.as_context_member_typename(), None);
        writer.scope_out(false);
    }

    fn def_context_cpp(&self, writer: &mut Writer) {
        snipets::def_context_ctor(self, true, &[], true, writer);
        writer.scope_in();
        writer.scope_out(false);
    }
}

impl DeserializerDefinition for ViewMemory {
    fn def_deserializer_h(&self, writer: &mut Writer) {
        writer.def_class(&self.as_deserializer_typename());
        writer.scope_in();
        writer.public();
        snipets::def_deserializer_ctor(self, false, writer);
        for t in &self.types {
            snipets::def_deserializer_get(self, t, false, writer);
        }
        writer.private();
        snipets::def_reader(writer);
        snipets::def_member_context_var(self, writer);
        writer.scope_out(true);
    }

    fn def_deserializer_cpp(&self, writer: &mut Writer) {
        snipets::def_deserializer_ctor(self, true, writer);
        writer.scope_in();
        writer.scope_out(false);
        for t in &self.types {
            snipets::def_deserializer_get(self, t, true, writer);
            writer.scope_in();
            snipets::if_previous_not_set_throw(self, writer);
            writer.write_with_offset(&format!("if ({}._index != nullptr && *{}._index != {}) ", 
                self.member_context_variable(), 
                self.member_context_variable(),
                t.constant.get_value()
            ));
            writer.scope_in();
            writer.write_line(&format!("throw std::exception(\"{} is not {}\");", self.as_typename(), t.as_typename()));
            writer.scope_out(false);
            if t.exact_size().is_some() {
                writer.write_line(&format!("{}.set_size({});", 
                    self.member_context_variable(), 
                    t.exact_size().unwrap()));
                snipets::read_and_return_value(self, None, t, writer);
            } else {
                writer.write_line(&format!("{} = {}({}._previous_end, {}._next_back_reference);", 
                    self.member_context_union_variable(),
                    t.as_context_member_typename(),
                    self.member_context_variable(),
                    self.member_context_variable()
                ));
                writer.write_line(&format!("return {}(_reader, {}.{});",
                    t.as_deserializer_typename(),
                    self.member_context_union_variable(),
                    t.member_context_union_variable(),
                ));
            }
            writer.scope_out(false);
        }
    }

    fn def_context_h(&self, writer: &mut Writer) {
        if let Some(u) = self.as_context_union_typename() {
            writer.def_union(&u);
            writer.scope_in();
            for td in self.types
            .iter()
            .map(|t| format!("{} {};", 
                t.as_context_member_typename(), 
                t.member_context_variable()
            ))
            .collect::<HashSet<String>>() {
                writer.write_line(&td);
            }
            writer.scope_out(true);
        }
        writer.def_struct(&self.as_context_typename());
        writer.scope_in();
        snipets::def_context_ctor(self, true, &[Argument {
                name: "_index".to_string(),
                typename: format!("{}*", self.get_index_typename().as_typename()),
            }],false, writer);
        writer.def_member_var(&self.member_context_variable(), "core::Context", None);
        writer.def_member_var("_index", &format!("{}*", self.get_index_typename().as_typename()), None);
        if let Some(u) = self.as_context_union_typename() {
            writer.def_member_var(&self.member_context_union_variable(), &u, None)
        }
        writer.scope_out(true);
    }

    fn def_context_cpp(&self, writer: &mut Writer) {
        snipets::def_context_ctor(self, true, &[Argument {
                name: "index".to_string(),
                typename: self.get_index_typename().as_typename(),
            }], true, writer);
        writer.write(", _index(index)");
        writer.scope_in();
        writer.scope_out(false);
    }
}

impl DeserializerDefinition for StructMemory {
    fn def_deserializer_h(&self, writer: &mut Writer) {
        writer.def_class(&self.as_deserializer_typename());
        writer.scope_in();
        writer.public();
        snipets::def_deserializer_ctor(self, false, writer);
        for f in &self.fields {
            snipets::def_deserializer_get(self, f, false, writer);
        }
        writer.private();
        snipets::def_reader(writer);
        snipets::def_member_context_var(self, writer);
        writer.scope_out(true);
    }

    fn def_deserializer_cpp(&self, writer: &mut Writer) {
        snipets::def_deserializer_ctor(self, true, writer);
        writer.scope_in();
        writer.scope_out(false);
        for (i, f) in self.fields.iter().enumerate() {
            snipets::def_deserializer_get(self, f, true, writer);
            writer.scope_in();
            snipets::if_previous_not_set_throw(f, writer);
            if i > 0 && !self.fields[i - 1].has_only_native_member() {
                for j in i..self.fields.len() {
                    if self.fields[j].has_only_native_member() {
                        writer.write_line(&format!("{}.set_size({});", self.fields[j].member_context_variable(), self.fields[j].exact_size().unwrap()));
                    } else {
                        break;
                    }
                }
            }
            if f.has_only_native_member() {
                snipets::read_and_return_value(f, None, f, writer);
            } else {
                writer.write_line(&format!("return {}(_reader, {});",
                        f.as_deserializer_typename(),
                        f.member_context_variable()));
            }
            writer.scope_out(false);
        }
    }

    fn def_context_h(&self, writer: &mut Writer) {
        writer.def_struct(&self.as_context_typename());
        writer.scope_in();
        snipets::def_context_ctor(self, false, &[], false, writer);
        for f in &self.fields {
            writer.def_member_var(&f.member_context_variable(), &f.as_context_member_typename(), None);
        }
        writer.scope_out(false);
    }

    fn def_context_cpp(&self, writer: &mut Writer) {
        snipets::def_context_ctor(self, false, &[], true, writer);
        let previous_has_native_members = |i: usize| -> bool {
            if i > 0 {
                self.fields[i - 1].has_only_native_member()
            } else {
                false
            }
        };
        for (i, f) in self.fields.iter().enumerate() {
            if i == 0 {
                writer.write(&format!(": {}(", f.member_context_variable()));
            } else {
                writer.write(&format!(", {}(", f.member_context_variable()));
            }

            if i == 0 {
                writer.write("previous");
            } else if previous_has_native_members(i) {
                writer.write(&format!("&{}", self.fields[i - 1].member_context_variable()));
            } else {
                writer.write("nullptr")
            }

            writer.write(", ");

            if i + 1 >= self.fields.len() {
                writer.write("next_back_reference")
            } else if f.has_only_native_member() {
                writer.write("nullptr")
            } else {
                writer.write(&format!("&{}._previous_end", 
                    self.fields[i + 1].member_context_variable()));
            }

            writer.write(")");
        }
        writer.scope_in();
        for f in &self.fields {
            if f.has_only_native_member() {
                writer.write_line(&format!("{}.set_size({});", f.member_context_variable(), f.exact_size().unwrap()));
            } else {
                break;
            }
        }
        writer.scope_out(false);
    }
}

impl DeserializerDefinition for MemoryDeclaration {
    fn def_deserializer_h(&self, writer: &mut Writer) {
        self.memory.def_deserializer_h(writer)
    }

    fn def_deserializer_cpp(&self, writer: &mut Writer) {
        self.memory.def_deserializer_cpp(writer)
    }

    fn def_context_h(&self, writer: &mut Writer) {
        self.memory.def_context_h(writer)
    }

    fn def_context_cpp(&self, writer: &mut Writer) {
        self.memory.def_context_cpp(writer)
    }
}

impl DeserializerDefinition for Memory {
    fn def_deserializer_h(&self, writer: &mut Writer) {
        match &self.memory {
            MemoryType::Native(_) => panic!("unexpected"),
            MemoryType::Struct(m) => m.def_deserializer_h(writer),
            MemoryType::View(m) => m.def_deserializer_h(writer),
            MemoryType::Enum(m) => m.def_deserializer_h(writer),
        }
    }

    fn def_deserializer_cpp(&self, writer: &mut Writer) {
        match &self.memory {
            MemoryType::Native(_) => panic!("unexpected"),
            MemoryType::Struct(m) => m.def_deserializer_cpp(writer),
            MemoryType::View(m) => m.def_deserializer_cpp(writer),
            MemoryType::Enum(m) => m.def_deserializer_cpp(writer),
        }
    }

    fn def_context_h(&self, writer: &mut Writer) {
        match &self.memory {
            MemoryType::Native(_) => panic!("unexpected"),
            MemoryType::Struct(m) => m.def_context_h(writer),
            MemoryType::View(m) => m.def_context_h(writer),
            MemoryType::Enum(m) => m.def_context_h(writer),
        }
    }

    fn def_context_cpp(&self, writer: &mut Writer) {
        match &self.memory {
            MemoryType::Native(_) => panic!("unexpected"),
            MemoryType::Struct(m) => m.def_context_cpp(writer),
            MemoryType::View(m) => m.def_context_cpp(writer),
            MemoryType::Enum(m) => m.def_context_cpp(writer),
        }
    }
}
