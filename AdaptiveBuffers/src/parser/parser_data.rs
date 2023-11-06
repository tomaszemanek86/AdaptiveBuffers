use super::*;

impl<'a> ParserData<String> for Token<'a> {
    fn data(&self) -> Option<String> {
        if self.found {
            return Some(self.token.into());
        }
        None
    }
}

impl<'a, TData: Parser + Clone> ParserData<TData> for TData {
    fn data(&self) -> Option<TData> {
        Some(self.clone())
    }
}
