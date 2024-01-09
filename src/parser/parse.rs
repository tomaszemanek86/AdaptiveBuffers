use std::ops::DerefMut;
use std::str::FromStr;

use super::*;

fn take<'a>(text: &CodeView, chars: usize) -> Result<CodeView, Option<ParseError>> {
    if text.rest().len() >= chars {
        return Ok(text.offset(chars));
    }
    Err(Some(ParseError::NotEnoughChars(text.offset(0))))
}

impl<TData: FromStr + TryFrom<usize> + Debug + Clone> Parser for Value<TData> {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        if let Some(c) = text.rest().chars().nth(0) {
            if c == 'h' {
                let count = text
                    .rest()
                    .chars()
                    .skip(1)
                    .into_iter()
                    .take_while(|c| is_a::is_digit(*c) || ('A'..'F').contains(c))
                    .count();
                if let Ok(hex) = usize::from_str_radix(&text.rest()[1..(count + 1)], 16) {
                    if let Ok(value) = TData::try_from(hex) {
                        self.value = Some(value);
                        return Ok(text.offset(count + 1));
                    }
                }
            }
            if c == 'b' {
                let count = text
                    .rest()
                    .chars()
                    .skip(1)
                    .into_iter()
                    .take_while(|c| is_a::is_digit(*c) || ('A'..'F').contains(c))
                    .count();
                if let Ok(hex) = usize::from_str_radix(&text.rest()[1..(count + 1)], 2) {
                    if let Ok(value) = TData::try_from(hex) {
                        self.value = Some(value);
                        return Ok(text.offset(count + 1));
                    }
                }
            }
            if c == 'B' {
                let count = text
                    .rest()
                    .chars()
                    .skip(1)
                    .into_iter()
                    .take_while(|c| is_a::is_digit(*c))
                    .count();
                if let Ok(bitset) = usize::from_str(&text.rest()[1..(count + 1)]) {
                    let abs_hex = 1usize << bitset;
                    if let Ok(value) = TData::try_from(abs_hex) {
                        self.value = Some(value);
                        return Ok(text.offset(count + 1));
                    }
                }
            }
        };
        let count = text
            .rest()
            .chars()
            .into_iter()
            .take_while(|c| is_a::is_digit(*c))
            .count();
        if count != 0 {
            if let Ok(value) = TData::from_str(&text.rest()[0..count]) {
                self.value = Some(value);
                return Ok(text.offset(count));
            }
        }
        Err(Some(ParseError::ParseValueFailed(text.offset(0))))
    }
}

impl<'b> Parser for String {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        if let Some(c) = text.rest().chars().nth(0) {
            if is_a::is_letter(c) || is_a::is_underscore(c) {
                let count = text
                    .rest()
                    .chars()
                    .into_iter()
                    .take_while(|c| is_a::is_word_mid(*c))
                    .count();
                *self = String::from(&text.rest()[..count]);
                return Ok(text.offset(count));
            }
        }
        Err(Some(ParseError::NotWord(text.offset(0))))
    }
}

impl Parser for WhiteChars {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let count = text
            .rest()
            .chars()
            .into_iter()
            .take_while(|c| is_a::is_white_space(*c))
            .count();
        if count >= self.min_count {
            return Ok(text.offset(count));
        }
        Err(Some(ParseError::NotAType(text.offset(0))))
    }
}

impl<TParser: Parser> Parser for Optional<TParser> {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        if let Ok(res) = self.parser.parse(text) {
            self.parsed = true;
            return Ok(res);
        }
        Ok(text.clone())
    }
}

impl<'b> Parser for Sequence<'b> {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let mut count = 0;
        for parser in self.parsers.iter_mut() {
            let res = parser.deref_mut().parse(&text.offset(count))?;
            count += res.view().len();
        }
        Ok(text.offset(count))
    }
}

impl<'b> Parser for Or<'b> {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        for i in 0..self.parsers.len() {
            if let Ok(res) = self.parsers[i].parse(text) {
                self.index = i;
                return Ok(res);
            }
        }
        Err(Some(ParseError::OrFailed(
            text.offset(0),
            String::from(self.error_message),
        )))
    }
}

impl<'b> Parser for Token<'b> {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let res = take(text, self.token.len());
        if res.is_err() {
            if self.produce_error {
                return Err(Some(ParseError::NotToken(
                    self.token.to_string(),
                    text.offset(0),
                )));
            } else {
                return Err(None);
            }
        }
        let res = res.unwrap();
        if res.view() == self.token {
            self.found = true;
            return Ok(res);
        }
        if self.produce_error {
            Err(Some(ParseError::NotToken(
                self.token.to_string(),
                text.offset(0),
            )))
        } else {
            Err(None)
        }
    }
}

impl Parser for Str {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        if text.rest().len() > 0 {
            if text.rest().chars().nth(0).unwrap() == self.beg_end {
                for i in 1..text.rest().len() {
                    if text.rest().chars().nth(i - 1).unwrap() == self.esc {
                        continue;
                    } else if text.rest().chars().nth(i).unwrap() == self.beg_end {
                        self.string = Some(String::from(&text.rest()[1..i]));
                        return Ok(text.offset(i + 1));
                    }
                }
            }
        }
        Err(Some(ParseError::NotStr(text.offset(0))))
    }
}

impl Parser for RequiredVersion {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let mut major = Value::<u8>::default();
        let mut minor = Value::<u8>::default();
        let mut patch = Value::<u8>::default();
        let res = Sequence::new(&mut [
            &mut Token::new("required_version", false),
            &mut WhiteChars::default(),
            &mut major,
            &mut Token::new(".", true),
            &mut minor,
            &mut Token::new(".", true),
            &mut patch,
        ])
        .parse(text)?;
        self.version[0] = major;
        self.version[1] = minor;
        self.version[2] = patch;
        Ok(res)
    }
}

impl<TData, TParser: Parser + ParserData<TData>> Parser for Repeat<TData, TParser> {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let mut count = 0;
        while let Ok(res) = self.parser.parse(&text.offset(count)) {
            let data = self
                .parser
                .data()
                .ok_or(ParseError::RetrieveDataFailed(text.offset(0)))?;
            self.parsed.push(data);
            count += res.view().len();
        }
        Ok(text.offset(count))
    }
}

impl<'b, TData, TParser: Parser + ParserData<TData>> Parser for Separated<'b, TData, TParser> {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let mut count = 0;
        let res = self.parser.parse(text)?;
        let data = self
            .parser
            .data()
            .ok_or(ParseError::RetrieveDataFailed(text.offset(0)))?;
        count += res.view().len();
        self.data.push(data);
        while let Ok(res) = self.separator.parse(&text.offset(count)) {
            count += res.view().len();
            let res = self.parser.parse(&text.offset(count))?;
            let data = self
                .parser
                .data()
                .ok_or(ParseError::RetrieveDataFailed(text.offset(0)))?;
            self.data.push(data);
            count += res.view().len();
        }
        Ok(text.offset(count))
    }
}

impl Parser for Endian {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let mut big = Token::new("big", true);
        let mut little = Token::new("little", true);
        let result = Sequence::new(&mut [
            &mut Token::new("endian", false),
            &mut WhiteChars::default(),
            &mut Or::new(
                &mut [&mut big, &mut little],
                "Expect 'big' or 'little' keyword.",
            ),
        ])
        .parse(text)?;
        self.big = big.found;
        Ok(result)
    }
}

impl Default for TypVariant {
    fn default() -> Self {
        Self::UnknownType
    }
}

impl Parser for u8 {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let count = text
            .rest()
            .chars()
            .into_iter()
            .take_while(|c| is_a::is_digit(*c))
            .count()
            .min(3);
        if count != 0 {
            let value = text.rest()[0..count].parse::<usize>().unwrap();
            if let Ok(u8_value) = u8::try_from(value) {
                *self = u8_value;
                return Ok(text.offset(count));
            }
        }
        Err(Some(ParseError::NotInt(text.offset(0))))
    }
}

impl Parser for Int {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        if let Some(char0) = text.rest().chars().nth(0) {
            match char0 {
                'u' => self.signed = false,
                'i' => self.signed = true,
                _ => return Err(Some(ParseError::NotInt(text.offset(0)))),
            }
            if text.rest().len() > 1 {
                let mut u8 = Value::<u8>::default();
                if let Ok(res) = u8.parse(&text.offset(1)) {
                    self.bytes = u8.value.unwrap();
                    let count = 1 + res.view().len();
                    return Ok(text.offset(count));
                }
            }
        }
        Err(Some(ParseError::NotInt(text.offset(0))))
    }
}

impl Parser for ViewConstantValue {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let mut unsigned_value = DataView::<Value<usize>>::default();
        let mut enum_member_ref = EnumMemberRef::default();
        let mut or_posibilities: [&mut dyn Parser; 2] = [
            &mut unsigned_value, 
            &mut enum_member_ref
        ];
        let mut or = Or::new(&mut or_posibilities, "Expect unsigned value or enum member");
        let res = Sequence::new(&mut [
            &mut WhiteChars::default(),
            &mut Token::new("=", true),
            &mut WhiteChars::default(),
            &mut or,
        ]).parse(text)?;
        if or.index == 0 {
            *self = ViewConstantValue::Usize(DataView::new(unsigned_value.value.unwrap(), unsigned_value.code_view.clone()));
        } else {
            *self = ViewConstantValue::EnumMemberRef(enum_member_ref);
        }
        Ok(res)
    }
}

impl Parser for ViewTypePosibility {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        self.constant = Some(ViewConstantValue::Usize(Default::default()));
        Sequence::new(&mut [
            &mut WhiteChars::default(),
            &mut self.typ,
            &mut WhiteChars::default(),
            &mut self.constant
        ]).parse(text)
    }
}

impl Parser for View {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        Sequence::new(&mut [
            &mut Token::new("view", false),
            &mut WhiteChars::default(),
            &mut self.name,
            &mut WhiteChars::default(),
            &mut Token::new("{", true),
            &mut WhiteChars::default(),
            &mut self.types,
            &mut WhiteChars::default(),
            &mut Token::new("}", true),
        ])
        .parse(text)
    }
}

impl Parser for TypVariant {
    fn parse<'b>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let mut int = DataView::<Int>::default();
        if let Ok(res) = int.parse(text) {
            *self = TypVariant::Int(int);
            return Ok(res);
        }
        let mut word = DataView::<String>::default();
        if let Ok(res) = word.parse(text) {
            *self = TypVariant::Unknown(DataView::new(word.data, word.code_view));
            return Ok(res);
        }
        return Err(Some(ParseError::NotAType(text.offset(0))));
    }
}

impl Parser for Typ {
    fn parse<'b>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        self.array_size = ArraySize::No;
        let mut size = Optional::new(Value::<u32>::default());
        let res = Sequence::new(&mut [
            &mut Token::new("[", true),
            &mut WhiteChars::default(),
            &mut self.typ,
            &mut WhiteChars::default(),
            &mut Or::new(&mut [
                &mut Sequence::new(&mut [
                    &mut Or::new(&mut [
                        &mut Token::new(";", false),
                        &mut Token::new(",", false)
                    ], "epected ';' or ','"),
                    &mut WhiteChars::default(),
                    &mut size,
                    &mut WhiteChars::default()
                ]),
                &mut WhiteChars::default()
            ], "epected 'size' or 'nothing'"),
            &mut Token::new("]", true),
        ]).parse(text);
        if res.is_ok() {
            if size.parsed {
                self.array_size = ArraySize::Exact(size.parser.value.unwrap());
            } else {
                self.array_size = ArraySize::Dyn;
            }
            return res
        }
        self.typ.parse(text)
    }
}

impl<'b> Parser for MemberReference {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        Sequence::new(&mut [
            &mut self.member_name,
            &mut WhiteChars::default(),
            &mut Token::new(".", true),
            &mut WhiteChars::default(),
            &mut Token::new(&self.property, true)
        ]).parse(text)
    }
}

impl<'b> Parser for StructMemberConstant {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let mut view_reference = MemberReference::new("key");
        let mut array_dimension = MemberReference::new("dimension");
        let mut or_posibilities: [&mut dyn Parser; 2] = [&mut view_reference, &mut array_dimension];
        let mut or = Or::new(
            &mut or_posibilities,
            "View reference or size of struct member",
        );
        let res: CodeView = or.parse(text)?;
        match or.index {
            0 => *self = StructMemberConstant::ViewMemberKey(view_reference),
            1 => *self = StructMemberConstant::ArrayDimension(array_dimension),
            _ => panic!("Unexpected index"),
        }
        Ok(res)
    }
}

impl<TData: Parser> Parser for Option<TData> {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        if let Some(parser) = self {
            if let Ok(res) = parser.parse(text) {
                return Ok(res);
            }
        } else {
            panic!("Option parser is None")
        }
        *self = None;
        Ok(text.offset(0))
    }
}

impl<'b> Parser for StructMember {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        self.constant = StructMemberConstant::No;
        Sequence::new(&mut [
                &mut self.name,
                &mut WhiteChars::default(),
                &mut Token::new(":", true),
                &mut WhiteChars::default(),
                &mut self.typ,
                &mut WhiteChars::default(),
                &mut Some(Sequence::new(&mut [
                    &mut Token::new("=", true),
                    &mut WhiteChars::default(),
                    &mut self.constant,
                ])),
            ],
        )
        .parse(text)
    }
}

impl<'b> Parser for Struct {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let mut struct_keyword = Token::new("struct", false);
        let res = Sequence {
            parsers: &mut [
                &mut struct_keyword,
                &mut WhiteChars::default(),
                &mut self.name,
                &mut WhiteChars::default(),
                &mut Token::new("{", true),
                &mut WhiteChars::default(),
                &mut self.members,
                &mut WhiteChars::default(),
                &mut Token::new("}", true),
            ],
        }
        .parse(text)?;
        Ok(res)
    }
}

impl<'a, TData: 'a + Parser + ParserData<TData> + Default> Parser for Vec<TData> {
    fn parse<'b>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let mut white_comma_white: [&mut dyn Parser; 3] = [
            &mut WhiteChars::default(),
            &mut Token::new(",", true),
            &mut WhiteChars::default(),
        ];
        let mut separator = Sequence::new(&mut white_comma_white);
        let mut parser = Separated::new(TData::default(), &mut separator);
        let res = parser.parse(text)?;
        *self = parser.data;
        Ok(res)
    }
}

impl Parser for EnumMemberRef {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        Sequence::new(&mut [
            &mut self.enum_name,
            &mut WhiteChars::default(),
            &mut Token::new("::", true),
            &mut WhiteChars::default(),
            &mut self.enum_member,
        ])
        .parse(text)
    }
}

impl<TData: Parser + Default + Clone> Parser for DataView<TData> {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let res = self.data.parse(text)?;
        self.code_view = res.clone();
        Ok(res)
    }
}

impl Parser for Option<SyntaxToken> {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        let mut parser = DataView::<RequiredVersion>::default();
        match parser.parse(text) {
            Ok(res) => {
                *self = Some(SyntaxToken::RequiredVersion(parser));
                return Ok(res);
            }
            Err(e) => {
                if e.is_some() {
                    return Err(e);
                }
            }
        }

        let mut parser = DataView::<Endian>::default();
        match parser.parse(text) {
            Ok(res) => {
                *self = Some(SyntaxToken::Endian(parser));
                return Ok(res);
            }
            Err(e) => {
                if e.is_some() {
                    return Err(e);
                }
            }
        }

        let mut parser = DataView::<Struct>::default();
        match parser.parse(text) {
            Ok(res) => {
                *self = Some(SyntaxToken::Struct(parser));
                return Ok(res);
            }
            Err(e) => {
                if e.is_some() {
                    return Err(e);
                }
            }
        }

        let mut parser = DataView::<Enum>::default();
        match parser.parse(text) {
            Ok(res) => {
                *self = Some(SyntaxToken::Enum(parser));
                return Ok(res);
            }
            Err(e) => {
                if e.is_some() {
                    return Err(e);
                }
            }
        }

        let mut parser = DataView::<View>::default();
        match parser.parse(text) {
            Ok(res) => {
                *self = Some(SyntaxToken::View(parser));
                return Ok(res);
            }
            Err(e) => {
                if e.is_some() {
                    return Err(e);
                }
            }
        }

        let mut parser = WhiteChars::new(1);
        match parser.parse(text) {
            Ok(res) => {
                *self = None;
            return Ok(res);
            }
            Err(e) => {
                if e.is_some() {
                    return Err(e);
                }
            }
        }

        return Err(Some(ParseError::UnknownSyntaxToken(text.offset(0))));
    }
}

impl Parser for EnumConstant {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        Sequence::new(&mut [
            &mut self.name,
            &mut WhiteChars::default(),
            &mut Token::new("=", true),
            &mut WhiteChars::default(),
            &mut self.typ,
        ])
        .parse(text)
    }
}

impl Parser for Enum {
    fn parse<'a>(&mut self, text: &CodeView) -> Result<CodeView, Option<ParseError>> {
        Sequence::new(&mut [
            &mut Token::new("enum", false),
            &mut WhiteChars::new(1),
            &mut self.name,
            &mut WhiteChars::default(),
            &mut Token::new(":", true),
            &mut WhiteChars::default(),
            &mut self.underlaying_int,
            &mut WhiteChars::default(),
            &mut Token::new("{", true),
            &mut WhiteChars::default(),
            &mut self.constants,
            &mut WhiteChars::default(),
            &mut Token::new("}", true),
        ])
        .parse(text)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn typ() {
        let mut typ_parser = TypVariant::default();
        let code_i8 = CodeView::from("i8");
        let res_code_i8 = typ_parser.parse(&code_i8);
        assert!(res_code_i8.is_ok());
        assert!(typ_parser.is_int());
        assert_eq!(typ_parser.as_int().unwrap().signed, true);
        assert_eq!(typ_parser.as_int().unwrap().bytes, 8);
    }

    #[test]
    fn dyn_array() {
        let mut typ_parser = Typ::default();
        let code = CodeView::from("[i16]");
        let res_code = typ_parser.parse(&code);
        assert!(res_code.is_ok());
        assert_eq!(typ_parser.array_size.is_dyn(), true);
        assert!(typ_parser.typ.is_int());
        assert_eq!(typ_parser.typ.as_int().unwrap().signed, true);
        assert_eq!(typ_parser.typ.as_int().unwrap().bytes, 16);
    }

    #[test]
    fn exact_array() {
        let mut parser = Typ::default();
        let res = parser.parse(&CodeView::from(
            "[u16, 3]",
        ));
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.array_size.is_exact(), true);
        assert_eq!(*parser.array_size.as_exact().unwrap(), 3);
    }

    #[test]
    fn u8() {
        let mut u8_parser = Value::<u8>::default();
        let text = CodeView::from("10000");
        let res = u8_parser.parse(&text);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn u8_from_hex() {
        let mut u8_parser = Value::<u8>::default();
        let text = CodeView::from("hC");
        let res = u8_parser.parse(&text);
        assert_eq!(res.is_ok(), true);
        assert_eq!(u8_parser.value.unwrap(), 12);
    }

    #[test]
    fn u8_from_bin() {
        let mut u8_parser = Value::<u8>::default();
        let text = CodeView::from("b1001001");
        let res = u8_parser.parse(&text);
        assert_eq!(res.is_ok(), true);
        assert_eq!(u8_parser.value.unwrap(), 73);
    }

    #[test]
    fn u8_from_bitset() {
        let mut u8_parser = Value::<u8>::default();
        let text = CodeView::from("B3");
        let res = u8_parser.parse(&text);
        assert_eq!(res.is_ok(), true);
        assert_eq!(u8_parser.value.unwrap(), 8);
    }

    #[test]
    fn word() {
        let mut parser = String::default();
        let text = CodeView::from("_abc2");
        let res = parser.parse(&text);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.as_ref().unwrap().view(), "_abc2");
        assert_eq!(parser, "_abc2");
    }

    #[test]
    fn white_chars() {
        let mut wc_parser = WhiteChars::default();
        let text = CodeView::from(" \n\tXXX");
        let res = wc_parser.parse(&text);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.as_ref().unwrap().view(), " \n\t");
        assert_eq!(res.as_ref().unwrap().rest(), "XXX");
    }

    #[test]
    fn sequence() {
        let mut u8_1 = Value::<u8>::default();
        let mut u8_2 = Value::<u8>::default();
        let mut seq_parser = Sequence {
            parsers: &mut [&mut u8_1, &mut WhiteChars::default(), &mut u8_2],
        };
        let text = CodeView::from("5 \n\n6AAA");
        let res = seq_parser.parse(&text);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.as_ref().unwrap().view(), "5 \n\n6");
        assert_eq!(res.as_ref().unwrap().rest(), "AAA");
    }

    #[test]
    fn or() {
        let mut u8 = Value::<u8>::default();
        let mut wc = WhiteChars::default();
        let mut or = Or {
            parsers: &mut [&mut u8, &mut wc],
            index: 0,
            error_message: "Error",
        };
        let text1 = CodeView::from("6");
        let text2 = CodeView::from("  ");
        let res1 = or.parse(&text1);
        assert_eq!(res1.is_ok(), true);
        assert_eq!(or.index, 0);
        assert_eq!(res1.as_ref().unwrap().view(), "6");
        let res2 = or.parse(&text2);
        assert_eq!(res2.is_ok(), true);
        assert_eq!(or.index, 1);
        assert_eq!(res2.as_ref().unwrap().view(), "  ");
    }

    #[test]
    fn token() {
        let mut token = Token::new("token", false);
        let text = CodeView::from("token...");
        let res = token.parse(&text);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.as_ref().unwrap().view(), "token");
        assert_eq!(res.as_ref().unwrap().rest(), "...");
    }

    #[test]
    fn str() {
        let mut string = Str {
            beg_end: '\'',
            esc: '\\',
            string: None,
        };
        let text = CodeView::from("'\\'x'c");
        let res = string.parse(&text);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.as_ref().unwrap().view(), "'\\'x'");
        assert_eq!(res.as_ref().unwrap().rest(), "c");
        assert_eq!(string.string.as_ref().unwrap(), "\\'x");
    }

    #[test]
    fn required_version() {
        let mut parser = RequiredVersion::default();
        let res = parser.parse(&CodeView::from("required_version   5.1.12"));
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.version[0].value.unwrap(), 5);
        assert_eq!(parser.version[1].value.unwrap(), 1);
        assert_eq!(parser.version[2].value.unwrap(), 12);
    }

    #[test]
    fn endian() {
        let mut parser = Endian::default();
        let res = parser.parse(&CodeView::from("endian   big"));
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.big, true);
        let res = parser.parse(&CodeView::from("endian   little"));
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.big, false);
    }

    #[test]
    fn member() {
        let mut parser: StructMember = Default::default();
        let res = parser.parse(&CodeView::from("member: u8"));
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.name.data, "member");
        assert_eq!(parser.typ.typ.is_int(), true);
    }

    #[test]
    fn repeat() {
        let mut parser = Repeat::<String, Token>::new(Token::new("REP", false));
        let res = parser.parse(&CodeView::from("REPREPREP"));
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap().view(), "REPREPREP");
        assert_eq!(parser.parsed.len(), 3);
        assert_eq!(parser.parsed[0], "REP");
        assert_eq!(parser.parsed[1], "REP");
        assert_eq!(parser.parsed[2], "REP");
    }

    #[test]
    fn parse_struct_member() {
        let mut parser = StructMember::default();
        let res = parser.parse(&CodeView::from("member1: u8"));
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.name.data, "member1");
        assert_eq!(parser.typ.typ.is_int(), true);
    }

    #[test]
    fn parse_struct_member_with_constant() {
        let mut parser = StructMember::default();
        let res = parser.parse(&CodeView::from("member1: u8 = $size"));
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.name.data, "member1");
        assert_eq!(parser.typ.typ.is_int(), true);
    }

    #[test]
    fn parse_struct_with_2_members() {
        let mut parser = Struct::default();
        let res = parser.parse(&CodeView::from(
            "struct XX {
            member1: u8,
            member2: u16
        }",
        ));
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.members.len(), 2);
        assert_eq!(parser.members[0].name.data, "member1");
        assert_eq!(parser.members[0].typ.typ.is_int(), true);
        assert_eq!(parser.members[1].name.data, "member2");
        assert_eq!(parser.members[1].typ.typ.is_int(), true);
    }

    #[test]
    fn parse_struct_with_1_members() {
        let mut parser = Struct::default();
        let res = parser.parse(&CodeView::from(
            "struct XX {
            member: u8
        }",
        ));
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.members.len(), 1);
        assert_eq!(parser.members[0].name.data, "member");
        assert_eq!(parser.members[0].typ.typ.is_int(), true);
    }

    #[test]
    fn parse_struct_with_constant() {
        let mut parser = Struct::default();
        let res = parser.parse(&CodeView::from(
            "struct Test {
            packet_size: u8 = view_member.key
        }",
        ));
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.members.len(), 1);
        assert_eq!(parser.members[0].name.data, "packet_size");
        assert_eq!(parser.members[0].typ.typ.is_int(), true);
        assert_eq!(parser.members[0].constant.is_view_member_key(), true);
    }

    #[test]
    fn parse_enum() {
        let mut parser = Enum::default();
        let res = parser.parse(&CodeView::from(
            "enum XXX : u16 {
            member_a = 1,
            member_b = 2
        }",
        ));
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.name, "XXX");
        assert_eq!(parser.underlaying_int.bytes, 16);
        assert_eq!(parser.constants.len(), 2);
        assert_eq!(parser.constants[0].name, "member_a");
        assert_eq!(parser.constants[1].name, "member_b");
        assert_eq!(*parser.constants[0].typ.value.as_ref().unwrap(), 1);
        assert_eq!(*parser.constants[1].typ.value.as_ref().unwrap(), 2);
    }

    #[test]
    fn parse_enum_2() {
        let mut parser = Enum::default();
        let res = parser.parse(&CodeView::from(
            "enum AnEnum : u8 {
            EvnumValue1 = 1,
            EvnumValue2 = 2,
            EvnumValue3 = 3
        }",
        ));
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.name, "AnEnum");
        assert_eq!(parser.underlaying_int.bytes, 8);
        assert_eq!(parser.constants.len(), 3);
        assert_eq!(parser.constants[0].name, "EvnumValue1");
        assert_eq!(parser.constants[1].name, "EvnumValue2");
        assert_eq!(parser.constants[2].name, "EvnumValue3");
        assert_eq!(*parser.constants[0].typ.value.as_ref().unwrap(), 1);
        assert_eq!(*parser.constants[1].typ.value.as_ref().unwrap(), 2);
        assert_eq!(*parser.constants[2].typ.value.as_ref().unwrap(), 3);
    }

    #[test]
    fn parse_enum_3() {
        let mut parser = Enum::default();
        let res = parser.parse(&CodeView::from(
            "enum TestEnum: u8 {
            TestValue1 = 100,
            TestValue2 = 200
        }",
        ));
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.name, "TestEnum");
        assert_eq!(parser.underlaying_int.bytes, 8);
        assert_eq!(parser.constants.len(), 2);
        assert_eq!(parser.constants[0].name, "TestValue1");
        assert_eq!(parser.constants[1].name, "TestValue2");
        assert_eq!(*parser.constants[0].typ.value.as_ref().unwrap(), 100);
        assert_eq!(*parser.constants[1].typ.value.as_ref().unwrap(), 200);
    }

    #[test]
    fn view() {
        let mut parser = View::default();
        let res = parser.parse(&CodeView::from(
            "view AnView {
            u8, u16, UninterpretedStruct
        }",
        ));
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.name, "AnView");
        assert_eq!(parser.types.len(), 3);
        assert_eq!(parser.types[0].typ.typ.is_int(), true);
        assert_eq!(parser.types[1].typ.typ.is_int(), true);
        assert_eq!(parser.types[2].typ.typ.is_unknown(), true);
    }
}