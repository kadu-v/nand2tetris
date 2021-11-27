#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    EOTOK,
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT,
    PUSH(Segments, usize),
    POP(Segments, usize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Segments {
    ARG,
    LCL,
    STC,
    CONST,
    THIS,
    THAT,
    PTR,
    TMP,
}
