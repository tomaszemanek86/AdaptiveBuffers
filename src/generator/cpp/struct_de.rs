use super::*;

pub fn generate_struct_deserializer(m: &StructMemory, writer: &mut Writer) {
    writer.write(&format!("class {}", m.deserializer_typename()));
    writer.scope_in();
    writer.public();
    generate_ctor(m, false, writer);
    generate_ctor(m, true, writer);
    for i in 0..m.fields.len() {
        generate_deserialze(m, i, writer);
    }
    let groups = m.get_groups();
    if groups.is_empty() {
        generate_empty_struct_methods(m, writer);
    } else {
        generate_init(m, writer);
        generate_deserialize_methods(m, 0, groups[0].0, groups[0].1, writer);
        generate_deserialized(m, writer);
        generate_source_set(m, writer);
        generate_end(m, writer);
    }
    writer.private();
    for i in 1..groups.len() {
        generate_deserialize_methods(m, i, groups[i].0, groups[i].1, writer);
    }
    for i in 0..m.fields.len() {
        generate_member_deserialzier(m, i, writer);
    }
    writer.write_line("uint8_t* source_;");
    writer.scope_out(true);
}

fn generate_init(m: &StructMemory, writer: &mut Writer) {
    writer.write_with_offset("void init()");
    writer.scope_in();
    writer.write_line("source_ = nullptr;");
    for f in &m.fields {
        writer.write_line(&format!("{}_.init();", f.name));
    }
    writer.scope_out(false);
}

fn generate_ctor(m: &StructMemory, default: bool, writer: &mut Writer) {
    writer.write_with_offset(&format!("{}({})", m.deserializer_typename(), if default { "uint8_t* source" } else { "" }));
    let init = m.fields
        .iter()
        //.filter(|f| f.default_constructible_deserializer())
        .map(|f| format!("{}_(nullptr)", f.name)
        )
        .collect::<Vec<String>>()
        .join(", ");
    if !init.is_empty() {
        writer.write(&format!(": {}", init));
    }
    writer.scope_in();
    let _ = m.fields.iter()
            .filter(|f| f.default_constructible_deserializer())
            .inspect(|f| writer.write_line(&format!("{}_.init();", f.name)));
    writer.write_line(&format!("_set_source({});", if default { "source" } else { "nullptr" }));
    for f in &m.fields {
        if let Some(asr) = f.get_array_size_reference() {
            writer.write_line(&format!("{}_.set_size_deserializer(&{}_);", f.name, asr.name));
        }
    }
    writer.scope_out(false);
}

fn generate_deserialze(m: &StructMemory, i: usize, writer: &mut Writer) {
    if m.fields[i].directly_deserializable() {
        writer.write_with_offset(&format!("{} {}()", 
            m.fields[i].native_typename(), 
            m.fields[i].name));
        writer.scope_in();
        generate_if_not_prev_deserialized_throw(m, i, writer);
        writer.write_line(&format!("return {}_.get_data();", m.fields[i].name));
        writer.scope_out(false);
    } else {
        writer.write_with_offset(&format!("{}& {}()", 
            m.fields[i].deserializer_typename(), 
            m.fields[i].name));
            writer.scope_in();
            generate_if_not_prev_deserialized_throw(m, i, writer);
            writer.write_line(&format!("return {}_;", m.fields[i].name));
            writer.scope_out(false);
    }
}

fn generate_if_not_prev_deserialized_throw(m: &StructMemory, i: usize, writer: &mut Writer) {
    if i > 0 {
        writer.write_with_offset(&format!("if (!{}_._deserialized())", m.fields[i - 1].as_ref().name));
        writer.scope_in();
        writer.write_line(&format!("throw std::runtime_error(\"{}\");", m.fields[i - 1].as_ref().name));
        writer.scope_out(false);
        writer.write_line(&format!("{}_._set_source({}_._end());",
            m.fields[i].as_ref().name,
            m.fields[i - 1].as_ref().name));
    }
}

fn generate_member_deserialzier(m: &StructMemory, i: usize, writer: &mut Writer) {
    let sm = m.fields[i].as_ref();
    writer.write_line(&format!("{} {}_;", 
        sm.deserializer_typename(),
        sm.name));
}

fn generate_empty_struct_methods(
    m: &StructMemory, 
    writer: &mut Writer
) {
    writer.write_with_offset("bool _deserialized() ");
    writer.scope_in();
    writer.write_line("return source_ != nullptr;");
    writer.scope_out(false);

    writer.write_with_offset("void _set_source(uint8_t *source) ");
    writer.scope_in();
    writer.write_line("source_ = source;");
    writer.scope_out(false);

    writer.write_with_offset("bool _source_set() ");
    writer.scope_in();
    writer.write_line("return source_ != nullptr;");
    writer.scope_out(false);

    writer.write_with_offset("uint8_t* _end() ");
    writer.scope_in();
    writer.write_line("return source_;");
    writer.scope_out(false);

    writer.write_with_offset("void init() ");
    writer.scope_in();
    writer.write_line("source_ = nullptr;");
    writer.scope_out(false);
}

fn generate_deserialize_methods(
    m: &StructMemory, 
    group_id: usize, 
    i0: usize, 
    i1: usize, 
    writer: &mut Writer
) {
    if group_id == 0 {
        writer.write_with_offset(&format!("void _set_source(uint8_t* source)"));
        writer.scope_in();
        writer.write_line("source_ = source;");
        writer.write_line(&format!("{}_._set_source(source_);", m.fields[0].name));
        for i in 1..(i1 + 1) {
            writer.write_line(&format!("{}_._set_source({}_._end());", m.fields[i].name, m.fields[i - 1].name));
        }
        writer.scope_out(false);
    } else {
        writer.write_with_offset(&format!("bool deserialize_group{}()", group_id));
        writer.scope_in();
        writer.write_line(&format!("if ({}_._deserialized()) return true;", m.fields[i0].name));
        writer.write_line(&format!("if (!{}_._deserialized()) return false;", m.fields[i0 - 1].name));
        for i in i0..(i1 + 1) {
            writer.write_line(&format!("{}_._set_source({}_._end());", m.fields[i].name, m.fields[i - 1].name));
        }
        writer.write_line("return true;");
        writer.scope_out(false);
    }
}

fn generate_deserialized(
    m: &StructMemory, 
    writer: &mut Writer
) {
    writer.write_with_offset(&format!("bool _deserialized()"));
    writer.scope_in();
    if m.fields.is_empty() {
        writer.write_line("return source_ != nullptr;");
    } else {
        writer.write_line(&format!("return {}_._deserialized();", m.fields.last().unwrap().name));
    }
    writer.scope_out(false);
}

fn generate_source_set(
    m: &StructMemory, 
    writer: &mut Writer
) {
    writer.write_with_offset(&format!("bool _source_set()"));
    writer.scope_in();
    writer.write_line(&format!("return {}_._source_set();", m.fields.first().unwrap().name));
    writer.scope_out(false);
}

fn generate_end(
    m: &StructMemory, 
    writer: &mut Writer
) {
    writer.write_with_offset(&format!("uint8_t* _end()"));
    writer.scope_in();
    writer.write_line(&format!("return {}_._end();", m.fields.first().unwrap().name));
    writer.scope_out(false);
}
