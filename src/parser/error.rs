use super::*;

impl ToString for ParseError {
    fn to_string(&self) -> String {
        match self {
            ParseError::NotEnoughChars(code_view) => {
                format!("Not enough chars at {}", code_view.pos())
            }
            ParseError::ParseValueFailed(code_view) => {
                format!("Parse value failed at {}", code_view.pos())
            }
            ParseError::NotInt(code_view) => format!("Not int at {}", code_view.pos()),
            ParseError::NotWord(code_view) => format!("Not word at {}", code_view.pos()),
            ParseError::OrFailed(code_view, message) => {
                format!("{message}. At {}", code_view.pos())
            }
            ParseError::NotToken(token, code_view) => {
                format!("Not token '{}' at {}", token, code_view.pos())
            }
            ParseError::NotStr(code_view) => format!("Not str at {}", code_view.pos()),
            ParseError::NotAType(code_view) => format!("Not a type at {}", code_view.pos()),
            ParseError::RetrieveDataFailed(code_view) => {
                format!("Retrieve data failed at {}", code_view.pos())
            }
            ParseError::UnknownSyntaxToken(code_view) => {
                format!("Unknown syntax token at {}", code_view.pos())
            }
            ParseError::ExpectedMemberSizeReference(code_view) => {
                format!("Expected member size reference at {}", code_view.pos())
            }
        }
    }
}
