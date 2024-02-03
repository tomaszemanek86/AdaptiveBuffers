use super::*;
use std::ops::{BitAnd, Shr};

impl<TData> Value<TData>
where
    TData: BitAnd<Output = TData> + Shr<Output = TData> + Copy + PartialEq + Default + FromStr + Debug + From<u8>,
{
    pub fn active_bits(&self) -> usize {
        let mut count = 0usize;
        let n = self.value.unwrap();
        let mut n = n;
        while n != TData::default() {
            if (n & TData::default()) != TData::default() as TData {
                count += 1;
            }
            n = n >> TData::from(1);
        }
        count as usize
    }

    pub fn has_only_one_bit(&self) -> bool {
        self.active_bits() == 1
    }
}
