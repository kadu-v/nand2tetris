//! Loc struct of Hack assembler

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Loc(usize, usize, usize);

impl Loc {
    pub fn new(line: usize, left: usize, right: usize) -> Self {
        Loc(line, left, right)
    }

    pub fn merge(&self, other: &Loc) -> Self {
        use std::cmp::{max, min};
        assert!(self.0 == other.0);
        Loc::new(self.0, min(self.1, other.1), max(self.2, other.2))
    }
}
