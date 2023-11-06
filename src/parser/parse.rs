use std::default;
use std::ops::DerefMut;
use std::str::FromStr;

use super::*;

fn take<'a>(text: &'a str, chars: usize) -> Result<ParseResult, ParseError> {
    if text.len() >= chars {
        return Ok(ParseResult { parsed: &text[0..chars], rest: &text[chars..] })
    }
    Err(ParseError::NotEnoughChars)
}

impl<'a, 'b: 'a> Parser<'a, 'b> for U8 {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        let count = text.chars().into_iter().take_while(|c| is_a::is_digit(*c)).count().min(3);
        if count != 0 {
            let value = text[0..count].parse::<usize>().unwrap();
            if let Ok(u8_value) = u8::try_from(value) {
                self.u8 = Some(u8_value);
                return Ok(ParseResult { parsed: &text[..count], rest: &text[count..] }); 
            }
        }
        Err(ParseError::NotU8)
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for String {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        if let Some(c) = text.chars().nth(0) {
            if is_a::is_letter(c) || is_a::is_underscore(c) {
                let count = text.chars().into_iter().take_while(|c| is_a::is_word_mid(*c)).count();
                *self = String::from(&text[..count]);
                return Ok(ParseResult { parsed: &text[..count], rest: &text[count..] }); 
            }
        }
        Err(ParseError::NotWord)
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for WhiteChars {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        let count = text.chars().into_iter().take_while(|c| is_a::is_white_space(*c)).count();
        if count >= self.min_count {
            return Ok(ParseResult { parsed: &text[..count], rest: &text[count..] });
        }
        Err(ParseError::NotU8)
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for Sequence<'a, 'b> {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        let mut count = 0;
        for parser in self.parsers.iter_mut() {
            let res = parser.deref_mut().parse(&text[count..])?;
        count += res.parsed.len();
}
        Ok(ParseResult { parsed: &text[..count], rest: &text[count..] })
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for Or<'a, 'b> {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        for i in 0..self.parsers.len() {
            if let Ok(res) = self.parsers[i].parse(text) {
                self.index = i;
                return Ok(res)
            }
        }
        Err(ParseError::OrFailed)
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for Token<'a> {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        take(text, self.token.len()).and_then(|res| {
            if res.parsed == self.token {
                self.found = true;
                return Ok(res)
            }
            Err(ParseError::NotToken)
        })
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for Str {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        if text.len() > 0 {
            if text.chars().nth(0).unwrap() == self.beg_end {
                for i in 1..text.len() {
                    if text.chars().nth(i - 1).unwrap() == self.esc {
                        continue
                    } else if text.chars().nth(i).unwrap() == self.beg_end {
                        self.string = Some(String::from(&text[1..i]));
                        return Ok(ParseResult { parsed: &text[0..(i + 1)], rest: &text[(i + 1)..] })
                    }
                }
            }
        }
        Err(ParseError::NotStr)
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for RequiredVersion {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        let mut major = U8::default();
        let mut minor = U8::default();
        let mut patch = U8::default();
        let res = Sequence::new(&mut [
            &mut Token::new("required_version"), 
            &mut WhiteChars::default(),
            &mut major,
            &mut Token::new("."),
            &mut minor,
            &mut Token::new("."),
            &mut patch
        ]).parse(text)?;
        self.version[0] = major;
        self.version[1] = minor;
        self.version[2] = patch;
        Ok(res)
    }
}

impl<'a, 'b: 'a, TData> Parser<'a, 'b> for Repeat<'a, TData> {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        let mut count = 0;
        while let Ok((res, data)) = (*self.parse_fn)(&text[count..]) {
            self.parsed.push(data);
            count += res.parsed.len();
        }
        Ok(ParseResult { parsed: &text[0..count], rest: &text[count..] })
    }
}

impl<'a, 'b: 'a, TData> Parser<'a, 'b> for Separated<'a, TData> {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        let mut count = 0;
        let mut binding: [&mut dyn Parser; 3] = [
            &mut WhiteChars::default(), 
            &mut Token::new(self.separator),
            &mut WhiteChars::default()];
        let mut separator_parser = Sequence::new(&mut binding);
        
        let (res, data) = (*self.parse_fn)(&text[count..])?;
        count += res.parsed.len();
        self.data.push(data);
        while let Ok(res) = separator_parser.parse(&text[count..]) {
            count += res.parsed.len();
let (res, data) = (*self.parse_fn)(&text[count..])?;
            self.data.push(data);
                            count += res.parsed.len();
            }
                    Ok(ParseResult { parsed: &text[..count], rest: &text[count..] })
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for Endian {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        let mut big = Token::new("big");
        let mut little = Token::new("little");
        let result = Sequence::new(&mut [
            &mut Token::new("endian"), 
            &mut WhiteChars::default(),
            &mut Or::new(&mut[&mut big, &mut little])
        ]).parse(text)?;
        self.big = big.found;
        Ok(result)
    }
}

impl Default for Typ {
    fn default() -> Self {
        Self::UnknownType
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for u8 {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        let count = text.chars().into_iter().take_while(|c| is_a::is_digit(*c)).count().min(3);
        if count != 0 {
            let value = text[0..count].parse::<usize>().unwrap();
            if let Ok(u8_value) = u8::try_from(value) {
                *self = u8_value;
                return Ok(ParseResult { parsed: &text[..count], rest: &text[count..] }); 
            }
        }
        Err(ParseError::NotInt)
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for Int {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        if let Some(char0) = text.chars().nth(0) {
            if char0 == 'u' {
                self.signed = false;
            } else if char0 == 'i' {
                self.signed = false;
            } else {
                return Err(ParseError::NotInt);
            }
            if text.len() > 1 {
                let mut u8 = U8::default();
                if let Ok(res) = u8.parse(&text[1..]) {
                    self.bytes = u8.u8.unwrap();
                    let count = 1 + res.parsed.len();
                    return Ok(ParseResult { parsed: &text[..count], rest: &text[count..] })
                }
            }
        }
        Err(ParseError::NotInt)
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for Typ {
    fn parse(&mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        let mut words = Vec::<String>::default();
        let mut seq = Sequence {
            parsers: &mut [
                &mut Token::new("View"),
                &mut WhiteChars::default(),
                &mut Token::new("{"),
                &mut WhiteChars::default(),
                &mut words,
                &mut WhiteChars::default(),
                &mut Token::new("}"),
            ]
        };
        if let Ok(res) = seq.parse(text) {
            *self = Typ::View(words);
            return Ok(res);
        }
        let mut int = Int::default();
        if let Ok(res) = int.parse(text) {
            *self = Typ::Int(int);
            return Ok(res);
        }
        let mut word = String::default();
        if let Ok(res) = word.parse(text) {
            *self = Typ::UserDefined(String::from(res.parsed));
            return Ok(res);
        }
        return Err(ParseError::NotAType)
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for StructMember {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
                let res = Sequence {
            parsers: &mut [
                &mut self.name, 
                &mut WhiteChars::default(),
                &mut Token::new(":"),
                &mut WhiteChars::default(),
                &mut self.typ
            ]
        }.parse(text)?;
        Ok(res)
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for Struct {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        Sequence {
            parsers: &mut [
            &mut Token::new("struct"),
            &mut WhiteChars::default(),
            &mut self.name,
            &mut WhiteChars::default(),
            &mut Token::new("{"),
            &mut WhiteChars::default(),
            &mut self.members,
            &mut WhiteChars::default(),
            &mut Token::new("}"),
        ]
        }.parse(text)
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for VariantItem {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        Sequence::new(&mut [ 
            &mut self.ident,
            &mut WhiteChars::default(),
            &mut Token::new("("),
            &mut WhiteChars::default(),
            &mut self.typ,
            &mut WhiteChars::default(),
            &mut Token::new(")")
        ]).parse(text)
    }
}

impl<'a, 'b: 'a, TData: Parser<'a, 'b> + Default> Parser<'a, 'b> for Vec<TData> {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        let mut parser = Separated::new(&|text: &str| {
            let mut item = TData::default();
            let res = item.parse(text)?;
            Ok((res, item))
        }, &",");
        let res = parser.parse(text)?;
        *self = parser.data;
        Ok(res)
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for Variant {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        Sequence::new(&mut [
            &mut Token::new("variant"),
            &mut WhiteChars::new(1),
            &mut self.name,
            &mut WhiteChars::default(),
            &mut Token::new(":"),
            &mut WhiteChars::default(),
            &mut self.typ,
            &mut WhiteChars::default(),
            &mut Token::new("{"),
            &mut WhiteChars::default(),
            &mut self.variants,
            &mut WhiteChars::default(),
            &mut Token::new("}")
        ]).parse(text)
    }
}

impl<'a, 'b: 'a, TData: Parser<'a, 'b> + Default> Parser<'a, 'b> for ParsedData<'a, 'b, TData> {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        let res = self.data.parse(text)?;
        self.result = Some(res);
        Ok(res)
    }
}

impl<'a, 'b: 'a> Parser<'a, 'b> for Option<SyntaxToken> {
    fn parse(&'a mut self, text: &'b str) -> Result<ParseResult<'a>, ParseError> {
        let mut parser = RequiredVersion::default(); 
        if let Ok(res) = parser.parse(text) {
            *self = Some(SyntaxToken::RequiredVersion(parser));
            return Ok(res);
        }
        let mut parser = Endian::default(); 
        if let Ok(res) = parser.parse(text) {
            *self = Some(SyntaxToken::Endian(parser));
            return Ok(res);
        }
        let mut parser = Struct::default(); 
        if let Ok(res) = parser.parse(text) {
            *self = Some(SyntaxToken::Struct(parser));
            return Ok(res);
        }
        let mut parser = Variant::default(); 
        if let Ok(res) = parser.parse(text) {
            *self = Some(SyntaxToken::Variant(parser));
            return Ok(res);
        }
        return Err(ParseError::UnknownSyntaxToken)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn u8() {
        let mut u8_parser = U8::default();
        let text = "10000";
        let res = u8_parser.parse(&text[..]);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.as_ref().unwrap().parsed, "100");
        assert_eq!(res.as_ref().unwrap().rest, "00");
        assert_eq!(u8_parser.u8.is_some(), true);
        assert_eq!(u8_parser.u8.unwrap(), 100);
    }

    #[test]
    fn word() {
        let mut parser = String::default();
        let text = "_abc2";
        let res = parser.parse(&text[..]);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.as_ref().unwrap().parsed, "_abc2");
        assert_eq!(parser, "_abc2");
    }

    #[test]
    fn white_chars() {
        let mut wc_parser = WhiteChars::default();
        let text = " \n\tXXX";
        let res = wc_parser.parse(&text[..]);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.as_ref().unwrap().parsed, " \n\t");
        assert_eq!(res.as_ref().unwrap().rest, "XXX");
    }

    #[test]
    fn sequence() {
        let mut u8_1 = U8::default();
        let mut u8_2 = U8::default();
        let mut seq_parser = Sequence{ parsers: &mut [&mut u8_1, &mut WhiteChars::default(), &mut u8_2] };
        let text = "5 \n\n6AAA";
        let res = seq_parser.parse(text);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.as_ref().unwrap().parsed, "5 \n\n6");
        assert_eq!(res.as_ref().unwrap().rest, "AAA");
    }

    #[test]
    fn or() {
        let mut u8 = U8::default();
        let mut wc = WhiteChars::default();
        let mut or = Or{ parsers: &mut [&mut u8, &mut wc], index: 0 };
        let text1 = "6";
        let text2 = "  ";
        let res1 = or.parse(text1);
        assert_eq!(res1.is_ok(), true);
        assert_eq!(or.index, 0);
        assert_eq!(res1.as_ref().unwrap().parsed, "6");
        let res2 = or.parse(text2);
        assert_eq!(res2.is_ok(), true);
        assert_eq!(or.index, 1);
        assert_eq!(res2.as_ref().unwrap().parsed, "  ");
    }

    #[test]
    fn token() {
        let mut token = Token::new("token");
        let text = "token...";
        let res = token.parse(text);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.as_ref().unwrap().parsed, "token");
        assert_eq!(res.as_ref().unwrap().rest, "...");
    }

    #[test]
    fn str() {
        let mut string = Str { beg_end: '\'', esc: '\\', string: None };
        let text = "'\\'x'c";
        let res = string.parse(text);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.as_ref().unwrap().parsed, "'\\'x'");
        assert_eq!(res.as_ref().unwrap().rest, "c");
        assert_eq!(string.string.as_ref().unwrap(), "\\'x");
    }

    #[test]
    fn required_version() {
        let mut parser = RequiredVersion::default();
        let res = parser.parse("required_version   5.1.12");
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.version[0].u8.unwrap(), 5);
        assert_eq!(parser.version[1].u8.unwrap(), 1);
        assert_eq!(parser.version[2].u8.unwrap(), 12);
    }

    #[test]
    fn endian() {
        let mut parser = Endian::default();
        let res = parser.parse("endian   big");
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.big, true);
        let res = parser.parse("endian   little");
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.big, false);
    }

    #[test]
    fn member() {
        let mut parser: StructMember = Default::default();
        let res = parser.parse("member: u8");
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.name, "member");
        assert_eq!(parser.typ.is_int(), true);
    }

    #[test]
    fn repeat() {
        let mut parser = Repeat::<String>::new(&|text: &str| {
            let res = Token::new("REP").parse(text)?;
            let out = res.parsed.to_string();
            Ok((res, out))
        });
        let res = parser.parse("REPREPREP");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap().parsed, "REPREPREP");
        assert_eq!(parser.parsed.len(), 3);
        assert_eq!(parser.parsed[0], "REP");
        assert_eq!(parser.parsed[1], "REP");
        assert_eq!(parser.parsed[2], "REP");
    }

    #[test]
    fn parse_struct_member() {
        let mut parser = StructMember::default();
        let res = parser.parse(&"member1: u8");
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.name, "member1");
        assert_eq!(parser.typ.is_int(), true);
    }

    #[test]
    fn parse_struct() {
        let mut parser = Struct::default();
        let res = parser.parse(&"struct XX {
            member1: u8,
            member2: u16
        }");
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.members.len(), 2);
        assert_eq!(parser.members[0].name, "member1");
        assert_eq!(parser.members[0].typ.is_int(), true);
        assert_eq!(parser.members[1].name, "member2");
        assert_eq!(parser.members[1].typ.is_int(), true);
    }

    #[test]
    fn parse_variant_item() {
        let mut parser = VariantItem::default();
        let res = parser.parse("X(u16)");
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.ident, "X");
        assert_eq!(parser.typ.is_int(), true);
    }

    #[test]
    fn parse_variant() {
        let mut parser = Variant::default();
        let res = parser.parse("variant AorB : u16 {
            A(String),
            B(UserDefinedStruct)
        }");
        assert_eq!(res.is_ok(), true);
        assert_eq!(parser.name, "AorB");
        assert_eq!(parser.variants.len(), 2);
        assert_eq!(parser.typ.is_int(), true);
    }
}
