//! Token for Hack assembler

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

    // 演算子
    Plus,  // "+"
    Minus, // "-"
    And,   // "&"
    Or,    // "|"
    Bang,  // "!"

    // dest部分のトークン
    MD,
    AM,
    AD,
    AMD,

    // jump部分のトークン
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,

    Semicoron, // ";"

    LParen, // "("
    RParen, // ")"
}

pub type Token = Annot<TokenKind>;

impl Token {
    pub fn at_sign(loc: Loc) -> Self {
        Self::new(TokenKind::AtSign, loc)
    }

    pub fn number(n: usize, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    pub fn symbol(s: String, loc: Loc) -> Self {
        Self::new(TokenKind::Symbol(s), loc)
    }

    pub fn a(loc: Loc) -> Self {
        Self::new(TokenKind::A, loc)
    }

    pub fn d(loc: Loc) -> Self {
        Self::new(TokenKind::D, loc)
    }

    pub fn m(loc: Loc) -> Self {
        Self::new(TokenKind::M, loc)
    }

    pub fn ad(loc: Loc) -> Self {
        Self::new(TokenKind::AD, loc)
    }

    pub fn am(loc: Loc) -> Self {
        Self::new(TokenKind::AM, loc)
    }

    pub fn md(loc: Loc) -> Self {
        Self::new(TokenKind::MD, loc)
    }

    pub fn amd(loc: Loc) -> Self {
        Self::new(TokenKind::AMD, loc)
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

    pub fn jgt(loc: Loc) -> Self {
        Self::new(TokenKind::JGT, loc)
    }

    pub fn jeq(loc: Loc) -> Self {
        Self::new(TokenKind::JEQ, loc)
    }

    pub fn jge(loc: Loc) -> Self {
        Self::new(TokenKind::JGE, loc)
    }

    pub fn jlt(loc: Loc) -> Self {
        Self::new(TokenKind::JLT, loc)
    }

    pub fn jne(loc: Loc) -> Self {
        Self::new(TokenKind::JNE, loc)
    }

    pub fn jle(loc: Loc) -> Self {
        Self::new(TokenKind::JLE, loc)
    }

    pub fn jmp(loc: Loc) -> Self {
        Self::new(TokenKind::JMP, loc)
    }

    pub fn semicoron(loc: Loc) -> Self {
        Self::new(TokenKind::Semicoron, loc)
    }

    pub fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }

    pub fn rpare(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
}
