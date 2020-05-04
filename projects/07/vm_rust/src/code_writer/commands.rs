//! Commands for Hack VM
use crate::code_writer::segments::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Commands {
    // Arithmatic commands
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,

    // Push, Pop
    Push(Segments, u16),
    Pop(Segments, u16),
}
