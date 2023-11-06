use super::*;

impl<'a, 'b: 'a> Or<'a, 'b> {
    pub fn new(parsers: &'b mut [&'b mut dyn Parser]) -> Self {
        Self { parsers: parsers, index: 0 }
    }
}

impl<'a, 'b: 'a> Token<'a> {
    pub fn new(token: &'b str) -> Self {
        Self { token: token, found: false }
    }
}

impl<'a, 'b: 'a> Sequence<'a, 'b> {
    pub fn new(parsers: &'b mut [&'b mut dyn Parser]) -> Self {
        Self { parsers: parsers }
    }
}

impl<'a, 'b: 'a, TData> Repeat<'a, TData> {
    pub fn new(parse_fn: &'b dyn Fn(&str) -> Result<(ParseResult, TData), ParseError>) -> Self {
        Self { parse_fn: parse_fn, parsed: Default::default() }
    }
}

impl<'a, 'b: 'a, TData> Separated<'a, TData> {
    pub fn new(parse_fn: &'b dyn Fn(&str) -> Result<(ParseResult, TData), ParseError>, separator: &'b str) -> Self {
        Self { parse_fn: parse_fn, separator: separator, data: Default::default() }
    }
}

impl WhiteChars {
    pub fn new(min_count: usize) -> Self {
        Self { min_count: min_count }
    }
}
