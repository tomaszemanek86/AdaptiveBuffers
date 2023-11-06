use super::*;

impl CodeView {
    pub fn offset(&self, offset: usize) -> CodeView {
        Self {
            origin: self.origin.clone(),
            from: self.to,
            to: self.to + offset,
        }
    }

    pub fn trim(&self, to: usize) -> CodeView {
        Self {
            origin: self.origin.clone(),
            from: self.from,
            to: self.from + to,
        }
    }

    pub fn view(&self) -> &str {
        &self.origin.as_str()[self.from..self.to]
    }

    pub fn rest(&self) -> &str {
        &self.origin.as_str()[self.to..]
    }

    pub fn pos(&self) -> String {
        format!("on line {} at column {}", self.line(), self.column())
    }

    fn line(&self) -> usize {
        self.origin.as_str()[..self.from].lines().count()
    }

    fn column(&self) -> usize {
        self.origin.as_str()[..self.from]
            .lines()
            .last()
            .unwrap_or("")
            .len()
    }
}

impl From<String> for CodeView {
    fn from(value: String) -> Self {
        Self {
            origin: Rc::new(value),
            from: 0,
            to: 0,
        }
    }
}

impl From<&str> for CodeView {
    fn from(value: &str) -> Self {
        Self {
            origin: Rc::new(String::from(value)),
            from: 0,
            to: 0,
        }
    }
}
