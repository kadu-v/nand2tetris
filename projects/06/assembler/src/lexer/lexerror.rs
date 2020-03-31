use crate::annot::Annot;
use crate::loc::Loc;

#[derive(Debug, PartialEq, Eq)]
pub enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

pub type LexError = Annot<LexErrorKind>;

impl LexError {
    pub fn invalid_char(c: char, loc: Loc) -> Self {
        Self::new(LexErrorKind::InvalidChar(c), loc)
    }

    pub fn eof(loc: Loc) -> Self {
        Self::new(LexErrorKind::Eof, loc)
    }
}
