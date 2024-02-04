use super::*;

pub fn generate_bit_mask_deserializer(m: &BitMask, writer: &mut Writer) {
    writer.write(&format!("class {}", m.deserializer_typename()));
    writer.scope_in();
    writer.public();
    generate_ctor(m, writer);
    for mask in &m.bits {
        generate_get(mask, writer);
    }
    generate_init(m, writer);
    generate_deserialized(m, writer);
    generate_source_set(m, writer);
    generate_end(writer);
    writer.private();
    writer.write_line(&format!("{} native_;", m.native.deserializer_typename()));
    writer.scope_out(true);
}

fn generate_ctor(m: &BitMask, writer: &mut Writer) {
    writer.write_line(&format!("{}() : native_(nullptr) {{}}", m.deserializer_typename()));
    writer.write_line(&format!("{}(uint8_t* source) : native_(source) {{}}", m.deserializer_typename()));
   
}

fn generate_init(m: &BitMask, writer: &mut Writer) {
    writer.write_with_offset("void init()");
    writer.scope_in();
    writer.write_line("native_.init();");
    writer.scope_out(false);
}

fn generate_deserialized(m: &BitMask, writer: &mut Writer) {
    writer.write_with_offset("bool _deserialized()");
    writer.scope_in();
    writer.write_line("return native_._deserialized();");
    writer.scope_out(false);
}

fn generate_source_set(m: &BitMask, writer: &mut Writer) {
    writer.write_with_offset("void _set_source(uint8_t* source)");
    writer.scope_in();
    writer.write_line("native_._set_source(source);");
    writer.scope_out(false);
}

fn generate_end(writer: &mut Writer) {
    writer.write_with_offset("uint8_t* _end()");
    writer.scope_in();
    writer.write_line("return native_._end();");
    writer.scope_out(false);
}

fn generate_get(mask: &Bits, writer: &mut Writer) {
    writer.write_with_offset(&format!("bool {}()", mask.name));
    writer.scope_in();
    writer.write_line("bool result = true;");
    for (i, bit) in mask.bits.iter().enumerate() {
        if bit.is_value() {
            if i > 0 && mask.bits[i - 1].is_not() {
                writer.write_line(&format!("result = result && !abf::is_bit_set(native_.get_data(), {});", bit.as_value().unwrap()));
            } else {
                writer.write_line(&format!("result = result && abf::is_bit_set(native_.get_data(), {});", bit.as_value().unwrap()));
            }
        }
    }
    writer.write_line("return result;");
    writer.scope_out(false);
}
