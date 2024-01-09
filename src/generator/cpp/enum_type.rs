use super::*;

pub fn generate_enum_type(m: &EnumMemory, writer: &mut Writer) {
    writer.write_with_offset(&format!("enum class {} : {}", m.name, m.underlaying_type.native_typename()));
    writer.scope_in();
    for c in &m.constants {
        writer.write_line(&format!("{} = {},", c.name, c.value))
    }
    writer.scope_out(true);
}