//! Commnda of Hack Assembler, A or C

use crate::annot::*;
use crate::loc::*;

#[derive(Debug, PartialEq, Eq)]
pub enum CommandKind {
    A(u16),
    // C命令 comp, dest, jump
    C(u16, u16, u16),
}

pub type Command = Annot<CommandKind>;

impl Command {
    pub fn a(n: u16, loc: Loc) -> Self {
        Self::new(CommandKind::A(n), loc)
    }

    pub fn c(comp: u16, dest: u16, jump: u16, loc: Loc) -> Self {
        Self::new(CommandKind::C(comp, dest, jump), loc)
    }
}
