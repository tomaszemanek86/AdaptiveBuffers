use super::*;

pub fn generate_enum_serializer(m: &EnumMemory, writer: &mut Writer) {
    writer.write(&format!("class {}", m.serializer_typename()));
    writer.scope_in();
    writer.public();
    generate_ctor(m, writer);
    generate_with_method(m, writer);
    generate_serialize(m, writer);
    writer.private();
    writer.write_line(&format!("abf::NativeSerializer<{}, {}> native_;", m.underlaying_type.native_typename(), m.underlaying_type.bytes().unwrap()));
    writer.scope_out(true);
}

fn generate_ctor(m: &EnumMemory, writer: &mut Writer) {
    writer.write_line(&format!("{}() : native_() {{}}", m.serializer_typename()));
}

fn generate_with_method(m: &EnumMemory, writer: &mut Writer) {
    writer.write_with_offset(&format!("void set_data({} value)", m.native_typename()));
    writer.scope_in();
    writer.write_line(&format!("native_.set_data(static_cast<{}>(value));", m.underlaying_type.native_typename()));
    writer.scope_out(false);
}

fn generate_serialize(m: &EnumMemory, writer: &mut Writer) {
    writer.write_with_offset("uint32_t serialize(uint8_t* dest)");
    writer.scope_in();
    writer.write_line("return native_.serialize(dest);");
    writer.scope_out(false);
}