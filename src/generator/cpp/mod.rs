use std::{fmt::format, io::Write};

use super::*;

mod writer;
mod cpp_memory_detail;
mod enum_type;
mod enum_ser;
mod enum_de;
mod struct_ser;
mod struct_de;
mod view_ser;
mod view_de;
mod abf_core;
mod bit_mask_ser;
mod bit_mask_de;

#[cfg(test)]
mod test;

pub struct Writer {
    begin_spaces: usize,
    buffer: String,
    filename: String,
}

trait CppMemoryDetail {
    fn name(&self) -> String;
    fn user_value_serializable(&self) -> bool;
    fn directly_deserializable(&self) -> bool;
    fn serializer_typename(&self) -> String;
    fn deserializer_typename(&self) -> String;
    fn native_typename(&self) -> String;
    fn bytes(&self) -> Option<u32>;
    fn default_constructible_deserializer(&self) -> bool;

    fn variable(&self) -> String {
        utils::to_snake_case(&self.name())
    }

}

pub fn generate(m: &Vec<MemoryDeclaration>, byte_swap: bool, args: &Args) {
    let output_namespace = std::path::Path::new(&args.protofile)
        .file_stem()
        .expect("could not extract stem")
        .to_str()
        .unwrap();

    std::fs::create_dir_all(&args.output_dir)
        .expect("could not create output directory");

    let abf_file = format!("{}/abf.h", args.output_dir);
    let mut f = std::fs::File::create(&abf_file).expect("could not create abf.h");
    let abf_source_code = abf_core::SOURCE.replace("<<BSWAP_SOURCE>>", if byte_swap {
        abf_core::BSWAP_SOURCE
    } else {
        abf_core::NO_BSWAP_SOURCE
    });
    let _ = f.write_all(abf_source_code.as_bytes()).expect("write abf.h failed");
    
    let mut writer = Writer::new(&format!("{}/{}.h", args.output_dir, output_namespace));
    writer.write_line("#pragma once");
    writer.write_line("#include \"abf.h\"");
    writer.write_line(&format!("namespace {} {{", output_namespace));
    for md in m {
        match &md.memory.memory {
            MemoryType::Native(_) => panic!("Unexpected"),
            MemoryType::Struct(s) => {
                struct_ser::generate_struct_serializer(&s.borrow(), &mut writer);
                struct_de::generate_struct_deserializer(&s.borrow(), &mut writer);
            },
            MemoryType::View(v) => {
                view_ser::generate_view_serializer(v, &mut writer);
                view_de::generate_view_deserializer(v, &mut writer);
            },
            MemoryType::Enum(e) => {
                enum_type::generate_enum_type(e, &mut writer);
                enum_ser::generate_enum_serializer(e, &mut writer);
                enum_de::generate_enum_deserializer(e, &mut writer);
            },
            MemoryType::BitMask(b) => {
                bit_mask_ser::generate_bit_mask_serializer(&b, &mut writer);
                bit_mask_de::generate_bit_mask_deserializer(&b, &mut writer);
            },
        }
    }
    writer.write_line("}");
    
}

fn generate_serialize_into_vector(writer: &mut Writer) {
    writer.write_with_offset("std::vector<uint8_t> serialize() ");
    writer.scope_in();
    writer.write_line("std::vector<uint8_t> out(size(), 0);");
    writer.write_line("serialize(out.data());");
    writer.write_line("return out;");
    writer.scope_out(false);
}