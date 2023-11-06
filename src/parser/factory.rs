use super::*;

impl<'a> Or<'a> {
    pub fn new<'b: 'a>(parsers: &'b mut [&'b mut dyn Parser], error_message: &'a str) -> Self
    where
        'a: 'b,
    {
        Self {
            parsers,
            index: 0,
            error_message,
        }
    }
}

impl<'a> Token<'a> {
    pub fn new<'b: 'a>(token: &'b str, produce_error: bool) -> Self
    where
        'a: 'b,
    {
        Self {
            token: token,
            found: false,
            produce_error: produce_error,
        }
    }
}

impl<'a> Sequence<'a> {
    pub fn new<'b: 'a>(parsers: &'b mut [&'b mut dyn Parser]) -> Self
    where
        'a: 'b,
    {
        Self { parsers }
    }
}

impl<'a, TData, TParser: Parser + ParserData<TData>> Repeat<TData, TParser> {
    pub fn new(parser: TParser) -> Self {
        Self {
            parser,
            parsed: Default::default(),
        }
    }
}

impl<'a, TData, TParser: Parser + ParserData<TData>> Separated<'a, TData, TParser> {
    pub fn new(parser: TParser, separator: &'a mut dyn Parser) -> Self {
        Self {
            parser,
            separator,
            data: Default::default(),
        }
    }
}

impl WhiteChars {
    pub fn new(min_count: usize) -> Self {
        Self { min_count }
    }
}

impl MemberReference {
    pub fn new(property: &str) -> Self {
        Self {
            member_name: Default::default(),
            property: property.into()
        }
    }
}