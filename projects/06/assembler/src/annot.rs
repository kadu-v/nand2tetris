use crate::loc::Loc;

/// アノテーション
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annot<T> {
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    pub fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }

    pub fn loc(&self) -> &Loc {
        &self.loc
    }
}
