use crate::loc::Loc;

pub enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

pub type LexError = Annot<LexErrorKind>

impl LexError {
    pub fn invalid_char(c: char, loc: Loc) -> Self {
        Self::new(LexErrorKind::InvalidChar, loc)
    }

    pub fn eof(loc: Loc) -> Self {
        Self::new(LexErrorKind::Eof, loc)
    }
}