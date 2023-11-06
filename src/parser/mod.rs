use std::marker::PhantomData;

mod parse;
mod is_a;
mod factory;

#[derive(Debug)]
pub enum ParseError {
    NotEnoughChars,
    NotU8,
    NotInt,
    NotWord,
    OrFailed,
    NotToken,
    NotStr,
    NotAType,
    UnknownType,
    UnknownSyntaxToken
}

pub struct ParseResult<'a> {
    pub parsed: &'a str,
    pub rest: &'a str,
}

pub trait Parser<'a, 'b: 'a> {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError>;
}

struct Or<'a, 'b: 'a> {
    parsers: &'a mut [&'a mut dyn Parser<'a, 'b>],
    index: usize
}

#[derive(Default)]
pub struct U8 {
    pub u8: Option<u8>
}

#[derive(Default)]
struct WhiteChars {
    min_count: usize
}

#[derive(Default)]
struct Token<'a> {
    token: &'a str,
    found: bool
}

struct Sequence<'a, 'b: 'a> {
    parsers: &'a mut [&'a mut dyn Parser<'a, 'b> ]
}

struct Repeat<'a, TData> {
    parse_fn: &'a dyn Fn(&str) -> Result<(ParseResult<'a>, TData), ParseError>,
    parsed: Vec<TData>
}

struct Str {
    beg_end: char,
    esc: char,
    string: Option<String>
}

#[derive(Default)]
pub struct Endian {
    pub big: bool
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Int {
    pub signed: bool,
    pub bytes: u8
}

#[derive(Debug, PartialEq, is_variant::IsVariant)]
pub enum Typ {
    Int(Int),
    View(Vec<String>),
    UserDefined(String),
    UnknownType
}

pub struct MemberInit {
    name: String,
    value: Value
}

pub enum Value {
    U8(u8),
    String(String),
    StructInit(Vec<MemberInit>)
}

#[derive(Debug, Default)]
pub struct StructMember {
    pub name: String,
    pub typ: Typ,
}

#[derive(Default)]
pub struct Struct {
    pub name: String,
    pub members: Vec<StructMember>
}

struct Separated<'a, TData> {
    parse_fn: &'a dyn Fn(&str) -> Result<(ParseResult, TData), ParseError>,
    separator: &'a str,
    data: Vec<TData>
}

#[derive(Debug, Default)]
pub struct VariantItem {
    ident: String,
    typ: Typ
}

#[derive(Debug, Default)]
pub struct Variant {
    name: String,
    typ: Typ,
    variants: Vec<VariantItem>
}

#[derive(Default)]
pub struct RequiredVersion {
    pub version: [U8; 3]
}

#[derive(Default)]
struct ParsedData<'a, 'b: 'a, TData: Parser<'a, 'b> + Default> {
    data: TData,
    result: Option<ParseResult<'a>>,
    phantom: PhantomData<&'b TData>,
}

pub enum SyntaxToken {
    RequiredVersion(RequiredVersion),
    Endian(Endian),
    Struct(Struct),
    Variant(Variant),
}

pub fn parse(text: &str) -> Result<Vec<SyntaxToken>, ParseError> {
    let mut tokens = Vec::default();
    let mut res = ParseResult { parsed: &text[..0], rest: &text[..] };
    loop {
        let mut token = Option::<SyntaxToken>::default();
        res = token.parse(res.rest)?;
        tokens.push(token.unwrap());
        if res.rest.len() == 0 {
            return Ok(tokens);
        }
    }
}
