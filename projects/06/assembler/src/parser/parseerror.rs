//! Parse Error for Hack Parser
use crate::lexer::token::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    // 予期せぬトークンがきている
    UnexpectedToken(Token),
    // オペレータでないものがきている
    NotOperator(Token),
    // カッコが閉じられていない．
    UnclosedOpenParen(Token),
    // 解析が終わったのに，トークンが残っている．
    RedundantToken(Token),
}
