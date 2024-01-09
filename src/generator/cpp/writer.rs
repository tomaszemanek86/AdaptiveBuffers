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

    fn put_line_offset(&mut self) {
        self.put_blank_spaces(self.begin_spaces);
    }

    fn put_blank_spaces(&mut self, spaces: usize) {
        self.buffer.push_str(&format!("{}", " ".repeat(spaces)));
    }
}

impl Drop for Writer {
    fn drop(&mut self) {
        let mut file = std::fs::File::create(&self.filename).unwrap();
        file.write_all(self.buffer.as_bytes()).unwrap();
    }
}
