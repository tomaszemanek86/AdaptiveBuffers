use super::*;

pub fn generate_factory(m: &dyn CppMemoryDetail, writer: &mut Writer) {
    writer.write_with_offset(&format!("class {}Factory", m.name()));
    writer.scope_in();
    writer.public();
    
    writer.write_with_offset(&format!("{} serializer() ", m.serializer_typename()));
    writer.scope_in();
    writer.write_line(&format!("return {}();", m.serializer_typename()));
    writer.scope_out(false);
    
    if m.default_constructible_deserializer() {
        writer.write_with_offset(&format!("{} deserializer() ", m.deserializer_typename()));
        writer.scope_in();
        writer.write_line(&format!("return {}();", m.deserializer_typename()));
        writer.scope_out(false);
    }
    
    writer.write_with_offset(&format!("{} deserializer(uint8_t *source) ", m.deserializer_typename()));
    writer.scope_in();
    writer.write_line(&format!("return {}(source);", m.deserializer_typename()));
    writer.scope_out(false);
    
    writer.scope_out(true);
}