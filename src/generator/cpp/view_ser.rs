use super::*;

pub fn generate_view_serializer(m: &ViewMemory, writer: &mut Writer) {
    writer.write(&format!("class {}", m.serializer_typename()));
    writer.scope_in();
    writer.public();
    generate_ctor(m, writer);
    for i in 0..m.types.len() {
        generate_with_method(m, i, writer);
    }
    generate_serialize(m, writer);
    generate_init(writer);
    generate_set_typypeid_serializer(m, writer);
    writer.private();
    generate_union(m, writer);
    writer.write_line("Types types_;");
    writer.write_line("bool set_;");
    generate_constant(m,  writer);
    writer.scope_out(true);
}

fn generate_ctor(m: &ViewMemory, writer: &mut Writer) {
    writer.write_line(&format!("{}() : types_(), set_(false), type_id_(), type_id_setter_(nullptr) {{}}", 
        m.serializer_typename()));
}

fn generate_union(m: &ViewMemory, writer: &mut Writer) {
    writer.write_with_offset("union Types");
    writer.scope_in();
    writer.write_line("Types() : __init(false) {}");
    writer.write_line("bool __init;");
    for t in &m.types {
        writer.write_line(&format!("{} {};", t.serializer_typename(), t.variable()));
    }
    writer.scope_out(true);
}

fn generate_constant(m: &ViewMemory, writer: &mut Writer) {
    match m.types.last().unwrap().constant {
        ViewPosibilityConstantMemory::Default(_) => writer.write_line(&format!("{} type_id_;", m.get_index_typename().native_typename())),
        ViewPosibilityConstantMemory::Usize(_) => writer.write_line(&format!("{} type_id_;", m.get_index_typename().native_typename())),
        ViewPosibilityConstantMemory::EnumMemberRef(_) => todo!(),
    }
    match m.types.last().unwrap().constant {
        ViewPosibilityConstantMemory::Default(_) => writer.write_line("abf::IViewKeySetter* type_id_setter_;"),
        ViewPosibilityConstantMemory::Usize(_) => writer.write_line("abf::IViewKeySetter* type_id_setter_;"),
        ViewPosibilityConstantMemory::EnumMemberRef(_) => todo!(),
    }
}

fn generate_set_typypeid_serializer(m: &ViewMemory, writer: &mut Writer) {
    writer.write_with_offset("void set_typeid_setter(abf::IViewKeySetter* setter)");
    writer.scope_in();
    writer.write_line("type_id_setter_ = setter;");
    writer.scope_out(false);
}

fn generate_serialize(m: &ViewMemory, writer: &mut Writer) {
    writer.write_with_offset("uint32_t serialize(uint8_t* dest)");
    writer.scope_in();
    writer.write_with_offset("if (!set_)");
    writer.scope_in();
    writer.write_line("throw std::runtime_error(\"Not set\");");
    writer.scope_out(false);

    writer.write_with_offset("if (type_id_setter_ != nullptr)");
    writer.scope_in();
    writer.write_line(&format!("type_id_setter_->set_{}(type_id_);", m.get_index_typename().name()));
    writer.scope_out(false);

    writer.write_with_offset("switch (type_id_)");
    writer.scope_in();
    for t in &m.types {
        writer.write_line(&format!("case {}: return types_.{}.serialize(dest);", t.constant.get_value(), t.variable()));
    }
    writer.scope_out(false);

    writer.write_line("throw std::runtime_error(\"Unknown type id\");");
    writer.scope_out(false);
}

fn generate_init(writer: &mut Writer) {
    writer.write_with_offset("void init()");
    writer.scope_in();
    writer.write_line("set_ = false;");
    writer.scope_out(false);
}

fn generate_with_method(m: &ViewMemory, i: usize, writer: &mut Writer) {
    match &m.types[i].memory {
        MemoryType::Native(_) => generate_with_native(m, i, writer),
        MemoryType::Struct(_) => generate_with_non_native(m, i, writer),
        MemoryType::View(_) => generate_with_non_native(m, i, writer),
        MemoryType::Enum(_) => generate_with_non_native(m, i, writer),
    }
}

fn generate_with_native(m: &ViewMemory, i: usize, writer: &mut Writer) {
    let t = &m.types[i].memory;
    writer.write_with_offset(&format!("void with_{}({} value)",
        t.variable(),
        t.native_typename()));
    writer.scope_in();
    writer.write_line(&format!("types_.{}.init();",
        t.variable()));
    writer.write_line(&format!("types_.{}.set_data(value);",
        t.variable()));
    writer.write_line(&format!("type_id_ = {};", m.types[i].constant.get_value()));
    writer.write_line("set_ = true;");
    writer.scope_out(false);
}

fn generate_with_non_native(m: &ViewMemory, i: usize, writer: &mut Writer) {
    let t = &m.types[i].memory;
    writer.write_with_offset(&format!("{}& with_{}()",
        t.serializer_typename(),
        t.variable()));
    writer.scope_in();
    writer.write_with_offset(&format!("if (set_ == false || type_id_ != {})", m.types[i].constant.get_value()));
    writer.scope_in();
    writer.write_line(&format!("types_.{}.init();",
        t.variable()));
    writer.write_line(&format!("type_id_ = {};", m.types[i].constant.get_value()));
    writer.scope_out(false);
    writer.write_line("set_ = true;");
    writer.write_line(&format!("return types_.{};",
        t.variable()));
    writer.scope_out(false);
}
