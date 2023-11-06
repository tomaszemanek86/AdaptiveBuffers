use super::*;

mod cpp_details;
mod data_generator;
mod serializer_definition;
mod snipets;
mod user_defined_type;
mod writer;
mod deserializer_definition;

const DATA_H: &str = "data.h";
const BUFFERS_H: &str = "buffers.h";
const SERIALZIERS_H: &str = "serializers.h";
const SERIALZIERS_CPP: &str = "serializers.cpp";
const DESERIALZIERS_H: &str = "deserializers.h";
const DESERIALZIERS_CPP: &str = "deserializers.cpp";
const CONTEXT_H: &str = "context.h";
const CONTEXT_CPP: &str = "context.cpp";

#[derive(Clone)]
pub struct Argument {
    name: String,
    typename: String
}

#[derive(Clone)]
pub struct CppSettings {
    pub dir: String,
}

pub trait UserDefinedType {}

pub trait SerializerDefinition {
    fn def_serializer_h(&self, writer: &mut Writer);
    fn def_serializer_cpp(&self, writer: &mut Writer);
    fn def_buffer_h(&self, writer: &mut Writer);
}

pub trait DeserializerDefinition {
    fn def_deserializer_h(&self, writer: &mut Writer);
    fn def_deserializer_cpp(&self, writer: &mut Writer);
    fn def_context_h(&self, writer: &mut Writer);
    fn def_context_cpp(&self, writer: &mut Writer);
}

pub trait CppDetails: MemoryDetails {
    fn as_typename(&self) -> String;
    fn as_variable(&self, prefix: Option<&str>, postfix: Option<&str>) -> String;
    fn as_pure_typename(&self) -> String;
    fn has_only_native_member(&self) -> bool;
    fn imports(&self) -> Vec<String>;

    fn as_buffer_typename(&self) -> String {
        if self.has_only_native_member() {
            format!("core::Buffer<{}>", self.as_typename())
        } else {
            format!("{}Buffer", self.as_typename())
        }
    }

    fn as_context_typename(&self) -> String {
        format!("{}Context", self.as_typename())
    }

    fn as_context_member_typename(&self) -> String {
        if self.has_only_native_member() {
            "core::Context".into()
        } else {
            self.as_context_typename()
        }
    }

    fn as_context_union_typename(&self) -> Option<String> {
        if self.has_only_native_member() {
            None
        } else {
            Some(format!("{}ContextUnion", self.as_typename()))
        }
    }

    fn as_serializer_typename(&self) -> String {
        format!("{}Serializer", self.as_typename())
    }

    fn as_deserializer_typename(&self) -> String {
        format!("{}Deserializer", self.as_typename())
    }

    fn as_max_array_serializer_typename(&self) -> String {
        format!("{}MaxArraySerializer", self.as_typename())
    }

    fn return_error_code(&self) -> String {
        format!("{}Error", self.as_serializer_typename())
    }

    fn member_buffer_variable(&self) -> String {
        self.as_variable(Some("_"), Some("_buffer"))
    }

    fn member_previous_end_ptr_variable(&self) -> String {
        self.as_variable(Some("_"), Some("_previous_end"))
    }

    fn member_end_ptr_variable(&self) -> String {
        self.as_variable(Some("_"), Some("_end"))
    }

    fn member_offset_variable(&self) -> String {
        self.as_variable(Some("_"), Some("_offset"))
    }

    fn member_context_variable(&self) -> String {
        self.as_variable(Some("_"), Some("_context"))
    }

    fn member_context_union_variable(&self) -> String {
        self.as_variable(Some("_"), Some("_context_union"))
    }

    fn member_deserialized_variable(&self) -> String {
        self.as_variable(Some("_"), Some("_deserialized"))
    }

    fn member_data_ptr_variable(&self) -> String {
        self.as_variable(Some("_"), Some("_data_ptr"))
    }

    fn member_serializer_variable(&self) -> String {
        self.as_variable(Some("_"), Some("_serializer"))
    }

    fn member_data_variable(&self) -> String {
        if self.has_only_native_member() {
            "_data".into()
        } else {
            self.as_variable(Some("_"), Some("_data"))
        }
    }

    fn member_isset_variable(&self) -> String {
        if self.has_only_native_member() {
            "_is_set".into()
        } else {
            self.as_variable(Some("_"), Some("_is_set"))
        }
    }

    fn member_index_variable(&self) -> String {
        if self.has_only_native_member() {
            "_index".into()
        } else {
            self.as_variable(Some("_"), Some("_index"))
        }
    }

    fn member_not_set_error_text(&self) -> String {
        format!("{} not set", self.member_data_variable())
    }

    fn member_buffer_ptr(&self) -> String {
        self.as_variable(Some("_"), Some("_buffer"))
    }

    fn fn_get(&self) -> String {
        self.as_variable(None, None)
    }
}

pub trait CppMemoryDetails {
    fn typename(&self) -> String;
    fn imports(&self) -> Vec<String>;
    fn field_name(&self, prefix: Option<&str>, postfix: Option<&str>) -> String;
}

const NATIVE_SERIALIZER_H: &str = "serializer_base.h";
const NATIVE_SERIALIZER_CONTENT: &str = "#pragma once
#include <cstdint>
#include <expected>
#include <string>

// Helpers for serialization
";

pub fn generate(m: &Vec<MemoryDeclaration>, args: &Args) {
    let mut writer = Writer::new(&format!("{}/{}", args.output_dir, NATIVE_SERIALIZER_H));
    writer.write(NATIVE_SERIALIZER_CONTENT);
    let mut writer = Writer::new(&format!("{}/{}", args.output_dir, DATA_H));
    data_generator::generate_h(m, &mut writer);
    let mut writer = Writer::new(&format!("{}/{}", args.output_dir, BUFFERS_H));
    // Data
    writer.write_line("#pragma once");
    writer.write_line(&format!("#include \"{}\"", DATA_H));
    m.iter().for_each(|mi| mi.def_buffer_h(&mut writer));
    // Serializer
    let mut writer = Writer::new(&format!("{}/{}", args.output_dir, SERIALZIERS_H));
    writer.write_line("#pragma once");
    writer.write_line(&format!("#include \"{}\"", BUFFERS_H));
    writer.write_line(&format!("#include \"{}\"", NATIVE_SERIALIZER_H));
    m.iter().for_each(|mi| mi.def_serializer_h(&mut writer));
    let mut writer = Writer::new(&format!("{}/{}", args.output_dir, SERIALZIERS_CPP));
    writer.write_line(&format!("#include \"{}\"", SERIALZIERS_H));
    m.iter().for_each(|mi| mi.def_serializer_cpp(&mut writer));
    // Deserializer
    let mut writer = Writer::new(&format!("{}/{}", args.output_dir, DESERIALZIERS_H));
    writer.write_line("#pragma once");
    writer.write_line(&format!("#include \"{}\"", DATA_H));
    writer.write_line(&format!("#include \"{}\"", CONTEXT_H));
    m.iter().for_each(|mi| mi.def_deserializer_h(&mut writer));
    let mut writer = Writer::new(&format!("{}/{}", args.output_dir, DESERIALZIERS_CPP));
    writer.write_line(&format!("#include \"{}\"", DESERIALZIERS_H));
    m.iter().for_each(|mi| mi.def_deserializer_cpp(&mut writer));
    // Context
    let mut writer = Writer::new(&format!("{}/{}", args.output_dir, CONTEXT_H));
    writer.write_line("#pragma once");
    m.iter().for_each(|mi| mi.def_context_h(&mut writer));
    let mut writer = Writer::new(&format!("{}/{}", args.output_dir, CONTEXT_CPP));
    writer.write_line(&format!("#include \"{}\"", CONTEXT_H));
    m.iter().for_each(|mi| mi.def_context_cpp(&mut writer));
}
