use super::*;

pub fn generate_view_deserializer(m: &ViewMemory, protocol_endian: &EndianSettings, writer: &mut Writer) {
    writer.write(&format!("class {}", m.deserializer_typename(protocol_endian)));
    writer.scope_in();
    writer.public();
    generate_ctor(m, protocol_endian, writer);
    for i in 0..m.types.len() {
        generate_get_method(m, i, protocol_endian, writer);
    }
    generate_deserialized(m, writer);
    generate_set_source(writer);
    generate_source_set(writer);
    generate_end(m, writer);
    generate_init(writer);
    writer.private();
    generate_check_deserialize(writer);
    generate_union(m, protocol_endian, writer);
    writer.write_line("uint8_t* source_;");
    writer.write_line("Types types_;");
    writer.write_line("bool deserialized_;");
    generate_constant(m,  protocol_endian, writer);
    writer.scope_out(true);
}

fn generate_set_source(writer: &mut Writer) {
    writer.write_with_offset("void _set_source(uint8_t *source)");
    writer.scope_in();
    writer.write_line("source_ = source;");
    writer.scope_out(false);
}

fn generate_end(m: &ViewMemory, writer: &mut Writer) {
    writer.write_with_offset("uint8_t* _end()");
    writer.scope_in();
    writer.write_line("_check_deserialized();");
    writer.write_with_offset("switch (type_id_)");
    writer.scope_in();
    for t in &m.types {
        writer.write_line(&format!("case {}: return types_.{}._end();",
            t.constant.get_value(),
            t.variable()
        ));
    }
    writer.scope_out(false);
    writer.write_line("return nullptr;");
    writer.scope_out(false);
}

fn generate_init(writer: &mut Writer) {
    writer.write_with_offset("void init()");
    writer.scope_in();
    writer.write_line("source_ = nullptr;");
    writer.write_line("deserialized_ = false;");
    writer.write_line("type_id_ = 0;");
    writer.scope_out(false);
}

fn generate_ctor(m: &ViewMemory, protocol_endian: &EndianSettings, writer: &mut Writer) {
    writer.write_line(&format!("{}() : source_(nullptr), types_(), deserialized_(false), type_id_(), type_id_deserializer_(nullptr) {{}}", 
        m.deserializer_typename(protocol_endian)));
    writer.write_line(&format!("{}(uint8_t *source) : source_(source), types_(), deserialized_(false), type_id_(), type_id_deserializer_(nullptr) {{}}", 
        m.deserializer_typename(protocol_endian)));
}

fn generate_union(m: &ViewMemory, protocol_endian: &EndianSettings, writer: &mut Writer) {
    writer.write_with_offset("union Types");
    writer.scope_in();
    writer.write_line("Types() : __init(false) {}");
    writer.write_line("~Types() {}");
    writer.write_line("bool __init;");
    for t in &m.types {
        writer.write_line(&format!("{} {};", t.deserializer_typename(protocol_endian), t.variable()));
    }
    writer.scope_out(true);
}

fn generate_constant(m: &ViewMemory, protocol_endian: &EndianSettings, writer: &mut Writer) {
    match m.types.last().unwrap().constant {
        ViewPosibilityConstantMemory::Default(_) => writer.write_line(&format!("{} type_id_;", m.get_index_typename().native_typename())),
        ViewPosibilityConstantMemory::Usize(_) => writer.write_line(&format!("{} type_id_;", m.get_index_typename().native_typename())),
        ViewPosibilityConstantMemory::EnumMemberRef(_) => writer.write_line(&format!("{} type_id_;", m.get_index_typename().native_typename())),
    }
    match m.types.last().unwrap().constant {
        ViewPosibilityConstantMemory::Default(_) => writer.write_line(&format!("{}* type_id_deserializer_;", m.get_index_typename().deserializer_typename(protocol_endian))),
        ViewPosibilityConstantMemory::Usize(_) => writer.write_line(&format!("{}* type_id_deserializer_;", m.get_index_typename().deserializer_typename(protocol_endian))),
        ViewPosibilityConstantMemory::EnumMemberRef(_) => writer.write_line(&format!("{}* type_id_deserializer_;", m.get_index_typename().deserializer_typename(protocol_endian))),
    }
}

fn generate_deserialized(m: &ViewMemory, writer: &mut Writer) {
    writer.write_with_offset("bool _deserialized()");
    writer.scope_in();

    writer.write_with_offset("if (deserialized_)");
    writer.scope_in();
    writer.write_with_offset("switch (type_id_)");
    writer.scope_in();
    for t in &m.types {
        writer.write_line(&format!("case {}: return types_.{}._deserialized();",
            t.constant.get_value(),
            t.variable()
        ));
    }
    writer.scope_out(false);
    writer.scope_out(false);
    
    writer.write_line("return false;");
    writer.scope_out(false);
}

fn generate_get_method(m: &ViewMemory, i: usize, protocol_endian: &EndianSettings, writer: &mut Writer) {
    match &m.types[i].memory {
        MemoryType::Native(_) => generate_get_native(m, i, writer),
        MemoryType::Struct(_) => generate_get_non_native(m, i, protocol_endian, writer),
        MemoryType::View(_) => generate_get_non_native(m, i, protocol_endian, writer),
        MemoryType::Enum(_) => generate_get_non_native(m, i, protocol_endian, writer),
        MemoryType::BitMask(_) => generate_get_non_native(m, i, protocol_endian, writer),
    }
}

fn generate_get_body(m: &ViewMemory, i: usize, writer: &mut Writer) {
    let t = &m.types[i];
    writer.write_line("_check_deserialized();");

    writer.write_with_offset(&format!("if (deserialized_)"));
    writer.scope_in();
    writer.write_with_offset(&format!("if (type_id_ != {})", m.types[i].constant.get_value()));
    writer.scope_in();
    writer.write_line(&format!("throw std::runtime_error(\"Already deserialized\");"));
    writer.scope_out(false);
    writer.scope_out(false);
    writer.write_with_offset("else");
    writer.scope_in();
    writer.write_line(&format!("types_.{}.init();", t.variable()));
    writer.scope_out(false);
    
    writer.write_line(&format!("type_id_ = {};", t.constant.get_value()));
    writer.write_line("deserialized_ = true;");
    writer.write_line(&format!("types_.{}._set_source(source_);", t.variable()));
}

fn generate_get_native(m: &ViewMemory, i: usize, writer: &mut Writer) {
    let t = &m.types[i].memory;
    writer.write_with_offset(&format!("{} {}()",
        t.native_typename(),
        t.variable()));
    writer.scope_in();
    generate_get_body(m, i, writer);
    writer.write_line(&format!("return types_.{}.get_data();", t.variable()));
    writer.scope_out(false);
}

fn generate_get_non_native(m: &ViewMemory, i: usize, protocol_endian: &EndianSettings, writer: &mut Writer) {
    let t = &m.types[i].memory;
    writer.write_with_offset(&format!("{}& {}()",
        t.deserializer_typename(protocol_endian),
        t.variable()));
    writer.scope_in();
    generate_get_body(m, i, writer);
    writer.write_line(&format!("return types_.{};", t.variable()));
    writer.scope_out(false);
}

fn generate_check_deserialize(writer: &mut Writer) {
    writer.write_with_offset(&format!("void _check_deserialized()"));
    writer.scope_in();
    writer.write_with_offset(&format!("if (type_id_deserializer_ != nullptr)"));
    writer.scope_in();

    writer.write_with_offset("if (type_id_deserializer_->_deserialized())");
    writer.scope_in();
    writer.write_line("deserialized_ = true;");
    writer.write_with_offset("type_id_ = type_id_deserializer_->get_data();");
    writer.scope_out(false);
    
    writer.write_with_offset("else");
    writer.scope_in();
    writer.write_line("deserialized_ = false;");
    writer.scope_out(false);
    
    writer.scope_out(false);
    writer.scope_out(false);
}

fn generate_source_set(
    writer: &mut Writer
) {
    writer.write_with_offset(&format!("bool _source_set()"));
    writer.scope_in();
    writer.write_line("return source_ != nullptr;");
    writer.scope_out(false);
}