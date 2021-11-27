use crate::loc::*;

use once_cell::sync::Lazy;
use std::collections::HashMap;

static KEYWORDS: Lazy<HashMap<&str, TokenKind>> = Lazy::new(|| {
    [
        ("add", TokenKind::Add),
        ("sub", TokenKind::Sub),
        ("neg", TokenKind::Neg),
        ("eq", TokenKind::Neg),
        ("gt", TokenKind::Gt),
        ("lt", TokenKind::Lt),
        ("and", TokenKind::And),
        ("or", TokenKind::Or),
        ("not", TokenKind::Not),
        ("push", TokenKind::Push),
        ("pop", TokenKind::Pop),
        ("argument", TokenKind::Argument),
        ("local", TokenKind::Local),
        ("constant", TokenKind::Constant),
        ("this", TokenKind::This),
        ("that", TokenKind::That),
        ("pointer", TokenKind::Temp),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<&str, TokenKind>>()
});

// ident が予約後の場合は対応する TokenKind を返す
pub fn lookup_keyword(ident: &str) -> Option<TokenKind> {
    if let Some(kind) = KEYWORDS.get(ident) {
        return Some(kind.clone());
    }
    None
}

// TokenKind for Hack VM
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Eof,
    NewLine,
    Illegal,
    // 数字とシンボル
    Number(usize),
    Symbol(String),

    // 算術演算コマンドと論理演算コマンド
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,

    // メモリアクセスコマンド
    Push,
    Pop,

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: Loc,
}
