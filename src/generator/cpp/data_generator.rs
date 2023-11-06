use super::*;

pub fn generate_h(m: &Vec<MemoryDeclaration>, writer: &mut Writer) {
    writer.write_line("#pragma once");
    m.iter()
        .flat_map(|mi| mi.memory.imports())
        .collect::<std::collections::HashSet<String>>()
        .iter()
        .for_each(|header_file| {
            writer.include(header_file, false);
        });

    m.iter().for_each(|mi| generate_data_h(mi, writer))
}

fn generate_data_h(m: &MemoryDeclaration, writer: &mut Writer) {
    match m.memory.memory {
        MemoryType::Struct(ref s) => generate_struct(s, writer),
        MemoryType::Enum(ref e) => generate_enum(e, writer),
        MemoryType::View(ref v) => generate_view(v, writer),
        _ => (),
    }
}

fn generate_enum(e: &EnumMemory, writer: &mut Writer) {
    writer.def_enum(&e.name, &e.underlaying_type.as_typename());
    writer.scope_in();
    for cst in &e.constants {
        writer.def_enum_value(&cst.name, cst.value);
    }
    writer.scope_out(true);
}

fn generate_view(v: &ViewMemory, writer: &mut Writer) {
    // define union
    writer.def_union(&v.as_typename());
    writer.scope_in();
    v.types.iter().for_each(|t| {
        writer.def_union_type(&t.as_variable(None, None), &t.as_typename());
    });
    writer.scope_out(true);
}

fn generate_struct(s: &StructMemory, writer: &mut Writer) {
    writer.def_struct(&s.as_typename());
    writer.scope_in();
    s.fields.iter().for_each(|f| {
        writer.def_member_var(&f.as_variable(None, None), &f.as_typename(), None);
    });
    writer.scope_out(true);
}
