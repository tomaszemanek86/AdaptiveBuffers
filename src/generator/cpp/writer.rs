use std::io::Write;

use super::*;

impl Writer {
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            buffer: String::new(),
            begin_spaces: 0,
        }
    }

    pub fn write(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    pub fn write_line(&mut self, s: &str) {
        self.put_line_offset();
        self.buffer.push_str(s);
        self.buffer.push_str("\n");
    }

    pub fn write_with_offset(&mut self, s: &str) {
        self.buffer
            .push_str(&format!("{}{}", " ".repeat(self.begin_spaces), s));
    }

    pub fn public(&mut self) {
        self.buffer.push_str("public:\n");
    }

    pub fn private(&mut self) {
        self.buffer.push_str("private:\n");
    }

    pub fn scope_in(&mut self) {
        self.put_blank_spaces(1);
        self.buffer.push_str("{\n");
        self.begin_spaces += 4;
    }

    pub fn scope_out(&mut self, semicolon: bool) {
        self.begin_spaces -= 4;
        self.put_line_offset();
        if semicolon {
            self.buffer.push_str("};\n");
        } else {
            self.buffer.push_str("}\n");
        }
    }

    pub fn def_struct(&mut self, name: &str) {
        self.buffer.push_str(&format!("struct {}", name));
    }

    pub fn def_class(&mut self, name: &str) {
        self.buffer.push_str(&format!("class {}", name));
    }

    pub fn def_member_function(
        &mut self,
        name: &str,
        arguments: &[Argument],
        return_type: Option<&str>,
        classname: Option<&str>,
    ) {
        self.put_line_offset();
        self.buffer.push_str(&format!(
            "{} {}{}(",
            return_type.unwrap_or("void"),
            classname
                .map(|c| format!("{}::", c))
                .unwrap_or(String::default()),
            name
        ));
        self.write(
            &arguments
                .iter()
                .map(|a| format!("{} {}", a.typename, a.name))
                .collect::<Vec<String>>()
                .join(", "),
        );
        self.buffer.push_str(")");
    }

    pub fn def_ctor(&mut self, classname: &str, arguments: &[Argument], in_cpp: bool) {
        self.put_line_offset();
        if in_cpp {
            self.buffer
                .push_str(&format!("{}::{}(", classname, classname));
        } else {
            self.buffer.push_str(&format!("{}(", classname));
        }
        self.write(
            &arguments
                .iter()
                .map(|a| format!("{} {}", a.typename, a.name))
                .collect::<Vec<String>>()
                .join(", "),
        );
        self.buffer.push_str(")");
        if !in_cpp {
            self.semicolon()
        }
    }

    pub fn def_enum(&mut self, name: &str, typ: &str) {
        self.buffer
            .push_str(&format!("enum class {} : {}", name, typ));
    }

    pub fn def_enum_value(&mut self, name: &str, value: usize) {
        self.put_line_offset();
        self.buffer.push_str(&format!("{} = {};\n", name, value));
    }

    pub fn def_union(&mut self, name: &str) {
        self.buffer.push_str(&format!("union {}", name));
    }

    pub fn def_union_type(&mut self, name: &str, typ: &str) {
        self.put_line_offset();
        self.buffer.push_str(&format!("{} {};\n", typ, name));
    }

    pub fn def_member_var(&mut self, name: &str, typ: &str, value: Option<&str>) {
        self.put_line_offset();
        self.buffer.push_str(&format!(
            "{} {}{};\n",
            typ,
            name,
            value
                .map(|v| format!(" = {}", v))
                .unwrap_or(String::default())
        ));
    }

    fn put_line_offset(&mut self) {
        self.put_blank_spaces(self.begin_spaces);
    }

    fn put_blank_spaces(&mut self, spaces: usize) {
        self.buffer.push_str(&format!("{}", " ".repeat(spaces)));
    }

    pub fn include(&mut self, name: &str, is_system: bool) {
        if is_system {
            self.buffer.push_str(&format!("#include <{}>\n", name));
        } else {
            self.buffer.push_str(&format!("#include \"{}\"\n", name));
        }
    }

    pub fn semicolon(&mut self) {
        self.buffer.push_str(";\n");
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        let mut file = std::fs::File::create(&self.filename).unwrap();
        file.write_all(self.buffer.as_bytes()).unwrap();
    }
}
