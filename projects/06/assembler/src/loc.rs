//! Loc struct of Hack assembler

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Loc(usize, usize, usize);

impl Loc {
    pub fn new(line: usize, left: usize, right: usize) -> Self {
        Loc(line, left, right)
    }
}
