use super::*;

pub fn generate_enum_deserializer(m: &EnumMemory, protocol_endian: &EndianSettings, writer: &mut Writer) {
    writer.write(&format!("class {}", m.deserializer_typename(protocol_endian)));
    writer.scope_in();
    writer.public();
    generate_ctor(m, protocol_endian, writer);
    generate_get(m, writer);
    generate_init(m, writer);
    generate_deserialized(m, writer);
    generate_source_set(m, writer);
    generate_set_source(m, writer);
    generate_end(writer);
    generate_get_size(m, writer);
    writer.private();
    writer.write_line(&format!("{} native_;", m.underlaying_type.deserializer_typename(protocol_endian)));
    writer.scope_out(true);
}

fn generate_ctor(m: &EnumMemory, protocol_endian: &EndianSettings, writer: &mut Writer) {
    writer.write_line(&format!("{}() : native_(nullptr) {{}}", m.deserializer_typename(protocol_endian)));
    writer.write_line(&format!("{}(uint8_t* source) : native_(source) {{}}", m.deserializer_typename(protocol_endian)));
   
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
    writer.write_with_offset("void set_source(uint8_t* source)");
    writer.scope_in();
    writer.write_line("native_.set_source(source);");
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

fn generate_get_size(m: &EnumMemory, writer: &mut Writer) {
    writer.write_with_offset("uint32_t get_size()");
    writer.scope_in();
    writer.write_line("return native_.get_size();");
    writer.scope_out(false);
}