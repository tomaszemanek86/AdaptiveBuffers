mod error;
mod factory;
mod is_a;
mod parse;
mod parser_data;
mod view_type_posibility;
mod default;
mod size_arithmetics;
mod value;

use std::fmt::Debug;
use std::str::FromStr;

use super::*;

#[derive(Debug)]
pub enum ParseError {
    NotEnoughChars(CodeView),
    ParseValueFailed(CodeView),
    NotInt(CodeView),
    NotWord(CodeView),
    OrFailed(CodeView, String),
    NotToken(String, CodeView),
    NotStr(CodeView),
    NotAType(CodeView),
    RetrieveDataFailed(CodeView),
    UnknownSyntaxToken(CodeView),
    ExpectedMemberSizeReferenceOrUsize(CodeView),
    ExpectedExpression(CodeView)
}

pub trait Parser {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>>;
}

pub trait ParserData<TData> {
    fn data(&self) -> Option<TData>;
}

struct Or<'a> {
    parsers: &'a mut [&'a mut dyn Parser],
    index: usize,
    error_message: &'a str,
}

#[derive(Default, Debug, Clone)]
pub struct Value<TData: FromStr + Debug + Clone> {
    pub value: Option<TData>
}

#[derive(Default, Clone)]
struct WhiteChars {
    min_count: usize,
    comments: Vec<String>
}

#[derive(Default)]
struct Token<'a> {
    token: &'a str,
    found: bool,
    produce_error: bool,
}

#[derive(Default)]
struct Sequence<'a> {
    parsers: &'a mut [&'a mut dyn Parser],
}

struct Repeat<TData, TParser: Parser + ParserData<TData>> {
    parser: TParser,
    parsed: Vec<TData>,
}

struct Str {
    beg_end: char,
    esc: char,
    string: Option<String>,
}

#[derive(Default, Clone)]
pub struct Endian {
    pub big: bool,
}

#[derive(Debug, Clone, variation::Variation)]
pub enum TypVariant {
    Int(DataView<Int>),
    Unknown(DataView<String>),
    UnknownType,
}

#[derive(Debug, Default, Clone)]
pub struct Typ {
    pub typ: TypVariant,
    pub array_size: ArraySize,
}

#[derive(Debug, Clone, Default)]
pub struct MemberReference {
    pub member_name: DataView<String>,
    property: String,
}

#[derive(variation::Variation, Debug, Clone)]
pub enum SizeArithmetics {
    MemberSizeReference(MemberReference),
    MemberValueReference(MemberReference),
    Plus,
    Minus,
    Usize(usize)
}

#[derive(variation::Variation, Debug, Clone)]
pub enum StructMemberConstant {
    No,
    ViewMemberKey(MemberReference),
    ArrayDimension(MemberReference),
    Usize(usize),
    Size(MemberReference),
    EnumMemberValue(EnumMemberRef),
    SizeArithmetics(Vec<DataView<SizeArithmetics>>),
}

#[derive(Debug, Default, Clone)]
pub struct StructMember {
    pub name: DataView<String>,
    pub typ: Typ,
    pub constant: StructMemberConstant,
}

#[derive(Default, Clone, Debug)]
pub struct Struct {
    pub name: DataView<String>,
    pub members: Vec<StructMember>,
}

struct Separated<'a, TData, TParser: Parser + ParserData<TData>> {
    parser: TParser,
    separator: &'a mut dyn Parser,
    data: Vec<TData>,
}

struct Optional<TParser: Parser> {
    parser: TParser,
    parsed: bool,
}

#[derive(Debug, Default, Clone)]
pub struct EnumMemberRef {
    pub enum_name: DataView<String>,
    pub enum_member: DataView<String>,
}

#[derive(Debug, Clone, variation::Variation)]
pub enum ViewConstantValue {
    Usize(DataView<usize>),
    EnumMemberRef(EnumMemberRef),
}

#[derive(Debug, Clone)]
pub struct ViewTypePosibility {
    pub typ: Typ,
    pub constant: Option<ViewConstantValue>,
}

#[derive(Debug, Default, Clone)]
pub struct View {
    pub name: String,
    pub types: Vec<DataView<ViewTypePosibility>>,
}

#[derive(Debug, Default, Clone)]
pub struct EnumConstant {
    pub name: String,
    pub typ: Value<usize>,
}

#[derive(Default, Clone, Debug)]
pub struct Enum {
    pub name: String,
    pub underlaying_int: Int,
    pub constants: Vec<DataView<EnumConstant>>,
}

#[derive(Default, Clone)]
pub struct RequiredVersion {
    pub version: [Value<u8>; 3],
}

pub enum SyntaxToken {
    RequiredVersion(DataView<RequiredVersion>),
    Endian(DataView<Endian>),
    Struct(DataView<Struct>),
    View(DataView<View>),
    Enum(DataView<Enum>),
    BitMask(DataView<BitMask>),
}

pub fn parse(text: String) -> Result<Vec<SyntaxToken>, ParseError> {
    let mut tokens = Vec::default();
    let mut res = CodeView::from(text);
    loop {
        let mut token = Option::<SyntaxToken>::default();
        match token.parse(&res) {
            Ok(view) => {
                if let Some(t) = token {
                    tokens.push(t);
                }
                res = view;
                if res.rest().len() == 0 {
                    return Ok(tokens);
                }
            }
            Err(err) => match err {
                Some(err) => return Err(err),
                None => panic!("unexpected error"),
            },
        }
    }
}
