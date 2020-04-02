//! Commnda of Hack Assembler, A or C
use crate::annot::*;
use crate::loc::*;
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
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

impl fmt::Debug for CommandKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandKind::A(v) => write!(f, "{:015b}", v),
            CommandKind::C(comp, dest, jump) => write!(
                f,
                "C(comp: {:07b}, dest: {:03b}, jump: {:03b})",
                comp, dest, jump
            ),
        }
    }
}
