use std::ops::Deref;

use super::*;

pub fn generate_bit_mask_serializer(m: &BitMask, writer: &mut Writer) {
    writer.write(&format!("class {}", m.serializer_typename()));
    writer.scope_in();
    writer.public();
    generate_ctor(m, writer);
    for mask in &m.bits {
        generate_with_method(m, mask, writer);
    }
    generate_size(writer);
    generate_serialize(writer);
    generate_serialize_into_vector(writer);
    writer.private();
    writer.write_line(&format!("{} native_;", m.native.serializer_typename()));
    writer.scope_out(true);
}

fn generate_ctor(m: &BitMask, writer: &mut Writer) {
    writer.write_with_offset(&format!("{}() : native_() ", m.serializer_typename()));
    writer.scope_in();
    writer.write_line("native_.set_data(0);");
    writer.scope_out(false);
}

fn generate_with_method(m: &BitMask, mask: &Bits, writer: &mut Writer) {
    if mask.bits.len() == 1 {
        writer.write_with_offset(&format!("{}& with_{}(bool on)", m.serializer_typename(), mask.name));
    } else {
        writer.write_with_offset(&format!("{}& with_{}()", m.serializer_typename(), mask.name));
    }

    writer.scope_in();
    writer.write_line("auto value = native_.data();");

    if mask.bits.len() == 1 {
        writer.write_line(&format!("value = abf::set_bit(value, {}, on);", mask.bits[0].as_value().unwrap()));
    } else {
        for (i, bits) in mask.bits.deref().iter().enumerate() {
            if let Some(bit) = bits.as_value() {
                if i > 0 && mask.bits[i - 1].is_not() {
                    writer.write_line(&format!("value = abf::set_bit(value, {}, false);", bit));
                } else {
                    writer.write_line(&format!("value = abf::set_bit(value, {}, true);", bit));
                }
            }
        }
    }
    
    writer.write_line("native_.set_data(value);");
    writer.write_line("return *this;");
    writer.scope_out(false);
}

fn generate_serialize(writer: &mut Writer) {
    writer.write_with_offset("uint32_t serialize(uint8_t* dest)");
    writer.scope_in();
    writer.write_line("return native_.serialize(dest);");
    writer.scope_out(false);
}

fn generate_size(writer: &mut Writer) {
    writer.write_with_offset("uint32_t size()");
    writer.scope_in();
    writer.write_line("return native_.size();");
    writer.scope_out(false);
}