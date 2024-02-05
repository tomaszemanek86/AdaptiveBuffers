use super::*;

pub fn generate_enum_deserializer(m: &EnumMemory, writer: &mut Writer) {
    writer.write(&format!("class {}", m.deserializer_typename()));
    writer.scope_in();
    writer.public();
    generate_ctor(m, writer);
    generate_get(m, writer);
    generate_init(m, writer);
    generate_deserialized(m, writer);
    generate_source_set(m, writer);
    generate_set_source(m, writer);
    generate_end(writer);
    writer.private();
    writer.write_line(&format!("abf::NativeDeserializer<{}, {}> native_;", m.underlaying_type.native_typename(), m.underlaying_type.bytes().unwrap()));
    writer.scope_out(true);
}

fn generate_ctor(m: &EnumMemory, writer: &mut Writer) {
    writer.write_line(&format!("{}() : native_(nullptr) {{}}", m.deserializer_typename()));
    writer.write_line(&format!("{}(uint8_t* source) : native_(source) {{}}", m.deserializer_typename()));
   
}

fn generate_init(m: &EnumMemory, writer: &mut Writer) {
    writer.write_with_offset("void init()");
    writer.scope_in();
    writer.write_line("native_.init();");
    writer.scope_out(false);
}

fn generate_deserialized(m: &EnumMemory, writer: &mut Writer) {
    writer.write_with_offset("bool _deserialized()");
    writer.scope_in();
    writer.write_line("return native_._deserialized();");
    writer.scope_out(false);
}

fn generate_source_set(m: &EnumMemory, writer: &mut Writer) {
    writer.write_with_offset("bool _source_set()");
    writer.scope_in();
    writer.write_line("return native_._source_set();");
    writer.scope_out(false);
}

fn generate_set_source(m: &EnumMemory, writer: &mut Writer) {
    writer.write_with_offset("void _set_source(uint8_t* source)");
    writer.scope_in();
    writer.write_line("native_._set_source(source);");
    writer.scope_out(false);
}

fn generate_end(writer: &mut Writer) {
    writer.write_with_offset("uint8_t* _end()");
    writer.scope_in();
    writer.write_line("return native_._end();");
    writer.scope_out(false);
}

fn generate_get(m: &EnumMemory, writer: &mut Writer) {
    writer.write_with_offset(&format!("{} get_data()", m.name));
    writer.scope_in();
    writer.write_line(&format!("return static_cast<{}>(native_.get_data());", m.name));
    writer.scope_out(false);
}
