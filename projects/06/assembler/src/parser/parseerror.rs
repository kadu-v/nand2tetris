//! Parse Error for Hack Parser
use crate::lexer::token::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseError {
    // 予期せぬトークンがきている
    UnexpactedToken(Token),
    // オペレータでないものがきている
    NotOperator(Token),
    // カッコが閉じられていない．
    UnclosedOpenParen(Token),
    // 解析が終わったのに，トークンが残っている．
    RedundantCommand(Token),
}
