use super::*;

pub fn generate_struct_serializer(m: &StructMemory, writer: &mut Writer) {
    writer.write(&format!("class {}", m.serializer_typename()));
    writer.scope_in();
    writer.public();
    generate_ctor(m, writer);
    for i in 0..m.fields.len() {
        if m.fields[i].user_value_serializable() {
            generate_with_method(m, i, writer);
        }
    }
    generate_size(m, writer);
    generate_serialize(m, writer);
    generate_serialize_into_vector(writer);
    generate_init(m, writer);
    writer.private();
    for i in 0..m.fields.len() {
        generate_member_serialzier(m, i, writer);
    }
    writer.scope_out(true);
}

fn generate_ctor(m: &StructMemory, writer: &mut Writer) {
    writer.write_with_offset(&format!("{}()", m.serializer_typename()));
    let initialize_members = m.fields
        .iter()
        .map(|f| format!("{}_()", f.name))
        .collect::<Vec<String>>()
        .join(", ");
    if !initialize_members.is_empty() {
        writer.write(": ");
        writer.write(&initialize_members);
    }
    writer.scope_in();
    for f in &m.fields {
        if let Some(asr) = f.get_array_size_reference() {
            writer.write_line(&format!("{}_.set_size_serializer(&{}_);", f.name, asr.name));
        }
        if let Some(vkr) = f.get_view_key_reference() {
            writer.write_line(&format!("{}_.set_typeid_setter(&{}_);", vkr.view.name, f.name));
        }
    }
    writer.scope_out(false)
}

fn generate_member_serialzier(m: &StructMemory, i: usize, writer: &mut Writer) {
    writer.write_line(&format!("{} {}_;", m.fields[i].as_ref().serializer_typename(), m.fields[i].name));
}

fn generate_with_method(m: &StructMemory, i: usize, writer: &mut Writer) {
    let sm = m.fields[i].as_ref();
    if sm.directly_deserializable() {
        writer.write_with_offset(&format!("void with_{}({} value)",
            sm.variable(),
            sm.native_typename()));
        writer.scope_in();
        writer.write_line(&format!("{}_.set_data(value);",
            sm.variable()));
        writer.scope_out(false);
    } else {
        writer.write_with_offset(&format!("{}& with_{}()",
            sm.serializer_typename(),
            sm.variable()));
        writer.scope_in();
        
        writer.write_line(&format!("return {}_;", sm.variable()));
        writer.scope_out(false);
    }
}

fn generate_size(m: &StructMemory, writer: &mut Writer) {
    writer.write_with_offset("uint32_t size()");
    writer.scope_in();
    writer.write_line("uint32_t size = 0;");
    for sm in &m.fields {
        writer.write_line(&format!("size += {}_.size();", sm.as_ref().variable()));
    }
    writer.write_line("return size;");
    writer.scope_out(false);
}

fn generate_serialize(m: &StructMemory, writer: &mut Writer) {
    writer.write_with_offset("uint32_t serialize(uint8_t *dest)");
    writer.scope_in();
    writer.write_line("uint32_t offset = 0;");
    for sm in &m.fields {
        writer.write_line(&format!("offset += {}_.serialize(dest + offset);", sm.as_ref().variable()));
    }
    writer.write_line("return offset;");
    writer.scope_out(false);
}

fn generate_init(m: &StructMemory, writer: &mut Writer) {
    writer.write_with_offset("void init()");
    writer.scope_in();
    for sm in &m.fields {
        writer.write_line(&format!("{}_.init();", sm.as_ref().variable()));
    }
    writer.scope_out(false);
}
