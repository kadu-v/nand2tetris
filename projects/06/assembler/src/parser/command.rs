//! Commnda of Hack Assembler, A or C

pub enum Command {
    A(u16),
    // C命令 comp, dest, jump
    C(u16, u16, u16),
}

impl Command {
    pub fn a(n: u16) -> Command {
        Command::A(n)
    }

    pub fn c(comp: u16, dest: u16, jump: u16) -> Command {
        Command::C(comp, dest, jump)
    }
}
