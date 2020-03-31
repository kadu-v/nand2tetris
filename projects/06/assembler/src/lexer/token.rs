//! Token for Hack assembler

use crate::annot::*;
use crate::loc::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // A命令のトークン
    AtSign,         // "@"
    Number(usize),  // "1234"
    Symbol(String), // "SUM"

    // C命令のトークン
    // comp部分のトークン
    // レジスタ
    A, // "A"
    M, // "M"
    D, // "D"
    // dest部分のトークン
    AM,
    AD,
    MD,
    AMD,

    // 演算子
    Equal, // =
    Plus,  // "+"
    Minus, // "-"
    And,   // "&"
    Or,    // "|"
    Bang,  // "!"

    // jump部分のトークン
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,

    Semicolon, // ";"

    LParen, // "("
    RParen, // ")"

    /// 定義済みシンボル
    SP,
    LCL,
    ARG,
    THIS,
    THAT,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    SCREEN,
    KBD,
}

pub type Token = Annot<TokenKind>;

impl Token {
    pub fn at_sign(loc: Loc) -> Self {
        Self::new(TokenKind::AtSign, loc)
    }

    pub fn number(n: usize, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    pub fn symbol(s: impl Into<String>, loc: Loc) -> Self {
        Self::new(TokenKind::Symbol(s.into()), loc)
    }

    pub fn get_symbol(&self) -> Option<String> {
        match self.get_value() {
            TokenKind::Symbol(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn equal(loc: Loc) -> Self {
        Self::new(TokenKind::Equal, loc)
    }

    pub fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }

    pub fn minus(loc: Loc) -> Self {
        Self::new(TokenKind::Minus, loc)
    }

    pub fn and(loc: Loc) -> Self {
        Self::new(TokenKind::And, loc)
    }

    pub fn or(loc: Loc) -> Self {
        Self::new(TokenKind::Or, loc)
    }

    pub fn bang(loc: Loc) -> Self {
        Self::new(TokenKind::Bang, loc)
    }

    pub fn semicolon(loc: Loc) -> Self {
        Self::new(TokenKind::Semicolon, loc)
    }

    pub fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }

    pub fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }

    pub fn to_token(kind: TokenKind, loc: Loc) -> Self {
        Self::new(kind, loc)
    }
}
