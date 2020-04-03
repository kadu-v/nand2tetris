use crate::lexer::lexerror::*;
use crate::parser::parseerror::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Lexer(LexError),
    Parser(ParseError),
    File(String),
}

impl From<LexError> for Error {
    fn from(e: LexError) -> Self {
        Error::Lexer(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parser(e)
    }
}
