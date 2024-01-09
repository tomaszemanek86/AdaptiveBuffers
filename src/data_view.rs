use std::ops::{Deref, DerefMut};

use super::*;

impl<T: Default + Clone> Deref for DataView<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Default + Clone> DerefMut for DataView<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: Default + Clone> AsRef<T> for DataView<T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T: Default + Clone> DataView<T> {
    pub fn new(data: T, code_view: CodeView) -> Self {
        Self { data, code_view }
    }

    pub fn convert<To: Default + Clone, FnConvert>(self, convert: FnConvert) -> DataView<To>
        where FnConvert: FnOnce(T) -> To 
    {
        DataView {
            code_view: self.code_view,
            data: convert(self.data)
        }
    }
}
