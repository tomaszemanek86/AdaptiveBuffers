use super::*;

pub fn generate_enum_serializer(m: &EnumMemory, protocol_endian: &EndianSettings, writer: &mut Writer) {
    writer.write(&format!("class {}", m.serializer_typename(protocol_endian)));
    writer.scope_in();
    writer.public();
    generate_ctor(m, protocol_endian, writer);
    generate_with_method(m, writer);
    generate_init(writer);
    generate_size(writer);
    generate_serialize(writer);
    generate_serialize_into_vector(writer);
    writer.private();
    writer.write_line(&format!("{} native_;", m.underlaying_type.serializer_typename(protocol_endian)));
    writer.scope_out(true);
}

fn generate_ctor(m: &EnumMemory, protocol_endian: &EndianSettings, writer: &mut Writer) {
    writer.write_line(&format!("{}() : native_() {{}}", m.serializer_typename(protocol_endian)));
}

fn generate_with_method(m: &EnumMemory, writer: &mut Writer) {
    writer.write_with_offset(&format!("void set_data({} value)", m.native_typename()));
    writer.scope_in();
    writer.write_line(&format!("native_.set_data(static_cast<{}>(value));", m.underlaying_type.native_typename()));
    writer.scope_out(false);
}

fn generate_serialize(writer: &mut Writer) {
    writer.write_with_offset("uint32_t serialize(uint8_t* dest)");
    writer.scope_in();
    writer.write_line("return native_.serialize(dest);");
    writer.scope_out(false);
}

fn generate_size(writer: &mut Writer) {
    writer.write_with_offset("uint32_t size()");
    writer.scope_in();
    writer.write_line("return native_.size();");
    writer.scope_out(false);
}

fn generate_init(writer: &mut Writer) {
    writer.write_with_offset("void init()");
    writer.scope_in();
    writer.write_line("native_.init();");
    writer.scope_out(false);
}