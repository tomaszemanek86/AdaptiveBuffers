use super::*;

impl Bits {
    pub fn value(&self) -> usize {
        let mut value = 0usize;
        for b in &self.bits {
            value |= 2usize.pow(*b as u32);
        }
        return value;
    }
}

impl BitMask {
    pub fn value(&self, name: &str) -> usize {
        for it in &self.bits {
            if it.name == name {
                return it.value();
            }
        }
        panic!("unknon name")
    }
}
