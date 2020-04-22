//! Token for Hack VM

use crate::annot::*;
use crate::loc::*;

// TokenKind for Hack VM
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // 数字とシンボル
    Number(u16),
    Symbol(String),

    // 算術演算コマンドと論理演算コマンド
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT,

    // メモリアクセスコマンド
    PUSH,
    POP,

    // segments
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

pub type Token = Annot<TokenKind>;

impl Token {
    pub fn number(n: u16, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    pub fn symbol(s: impl Into<String>, loc: Loc) -> Self {
        Self::new(TokenKind::Symbol(s.into()), loc)
    }

    pub fn add(loc: Loc) -> Self {
        Self::new(TokenKind::ADD, loc)
    }

    pub fn sub(loc: Loc) -> Self {
        Self::new(TokenKind::SUB, loc)
    }

    pub fn neg(loc: Loc) -> Self {
        Self::new(TokenKind::NEG, loc)
    }

    pub fn eq(loc: Loc) -> Self {
        Self::new(TokenKind::EQ, loc)
    }

    pub fn gt(loc: Loc) -> Self {
        Self::new(TokenKind::GT, loc)
    }

    pub fn lt(loc: Loc) -> Self {
        Self::new(TokenKind::LT, loc)
    }

    pub fn and(loc: Loc) -> Self {
        Self::new(TokenKind::AND, loc)
    }

    pub fn or(loc: Loc) -> Self {
        Self::new(TokenKind::OR, loc)
    }

    pub fn not(loc: Loc) -> Self {
        Self::new(TokenKind::NOT, loc)
    }

    pub fn push(loc: Loc) -> Self {
        Self::new(TokenKind::PUSH, loc)
    }

    pub fn pop(loc: Loc) -> Self {
        Self::new(TokenKind::POP, loc)
    }

    pub fn argument(loc: Loc) -> Self {
        Self::new(TokenKind::Argument, loc)
    }

    pub fn local(loc: Loc) -> Self {
        Self::new(TokenKind::Local, loc)
    }

    pub fn static_(loc: Loc) -> Self {
        Self::new(TokenKind::Static, loc)
    }

    pub fn constant(loc: Loc) -> Self {
        Self::new(TokenKind::Constant, loc)
    }

    pub fn this(loc: Loc) -> Self {
        Self::new(TokenKind::This, loc)
    }

    pub fn that(loc: Loc) -> Self {
        Self::new(TokenKind::That, loc)
    }

    pub fn pointer(loc: Loc) -> Self {
        Self::new(TokenKind::Pointer, loc)
    }

    pub fn temp(loc: Loc) -> Self {
        Self::new(TokenKind::Temp, loc)
    }
}
