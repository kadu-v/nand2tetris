//! Lexer for Hack Assembler

use crate::lexerror::*;
use crate::loc::*;
use crate::token::*;
use std::collections::HashMap;

fn lex(input: &str, line: usize) -> Result<Vec<Token>, LexError> {
    let keywords = [
        ("A", TokenKind::A),
        ("M", TokenKind::M),
        ("D", TokenKind::D),
        ("AM", TokenKind::AM),
        ("AD", TokenKind::AD),
        ("AMD", TokenKind::AMD),
        ("JGT", TokenKind::JGT),
        ("JEQ", TokenKind::JEQ),
        ("JGE", TokenKind::JGE),
        ("JLT", TokenKind::JLT),
        ("JNE", TokenKind::JNE),
        ("JLE", TokenKind::JLE),
        ("JMP", TokenKind::JMP),
        ("SP", TokenKind::SP),
        ("LCL", TokenKind::LCL),
        ("ARG", TokenKind::ARG),
        ("THIS", TokenKind::THIS),
        ("THAT", TokenKind::THAT),
        ("R0", TokenKind::R0),
        ("R1", TokenKind::R1),
        ("R2", TokenKind::R2),
        ("R3", TokenKind::R3),
        ("R4", TokenKind::R4),
        ("R5", TokenKind::R5),
        ("R6", TokenKind::R6),
        ("R7", TokenKind::R7),
        ("R8", TokenKind::R8),
        ("R9", TokenKind::R9),
        ("R10", TokenKind::R10),
        ("R11", TokenKind::R11),
        ("R12", TokenKind::R12),
        ("R13", TokenKind::R13),
        ("R14", TokenKind::R14),
        ("R15", TokenKind::R15),
        ("SCREEN", TokenKind::SCREEN),
        ("KBD", TokenKind::KBD),
    ]
    .into_iter()
    .cloned()
    .collect::<HashMap<_, TokenKind>>();

    // 解析結果を保存するベクター
    let mut tokens = Vec::new();
    //　入力
    let input = input.as_bytes();

    // 現在読み込み中の位置を管理する値
    let mut pos = 0;

    // サブレキサーを呼んだ後にposを更新するマクロ
    macro_rules! lex_a_token {
        ($lexer:expr) => {{
            let (tok, p) = $lexer?;
            tokens.push(tok);
            pos = p;
        }};
    }

    while pos < input.len() {
        // ここでそれぞれの関数にinputとposを渡す
        match input[pos] {
            b'@' => lex_a_token!(lex_at_sign(input, line, pos)),
            b'=' => lex_a_token!(lex_equal(input, line, pos)),
            b'+' => lex_a_token!(lex_plus(input, line, pos)),
            b'-' => lex_a_token!(lex_minus(input, line, pos)),
            b'&' => lex_a_token!(lex_and(input, line, pos)),
            b'|' => lex_a_token!(lex_or(input, line, pos)),
            b'!' => lex_a_token!(lex_bang(input, line, pos)),
            b';' => lex_a_token!(lex_semicolon(input, line, pos)),
            b'(' => lex_a_token!(lex_lparen(input, line, pos)),
            b')' => lex_a_token!(lex_rparen(input, line, pos)),
            b' ' | b'\n' | b'\t' => {
                let ((), p) = skip_whitespaces(input, pos)?;
                pos = p;
            }
            b => {
                return Err(LexError::invalid_char(
                    b as char,
                    Loc::new(line, pos, pos + 1),
                ))
            }
        }
    }

    Ok(tokens)
}

// posのバイトが期待するものであれば１バイト消費してposを１進める
fn consume_byte(input: &[u8], line: usize, pos: usize, b: u8) -> Result<(u8, usize), LexError> {
    // posが入力サイズ以上なら入力が終わっている
    // 一ばいと期待しているのに終わっているのでエラー
    if input.len() <= pos {
        return Err(LexError::eof(Loc::new(line, pos, pos + 1)));
    }

    // 入力が期待するものでなければエラー
    if input[pos] != b {
        return Err(LexError::invalid_char(
            input[pos] as char,
            Loc::new(line, pos, pos + 1),
        ));
    }

    Ok((b, pos + 1))
}

fn skip_whitespaces(input: &[u8], pos: usize) -> Result<((), usize), LexError> {
    let pos = recoginize_many(input, pos, |b| b" \n\t".contains(&b));
    Ok(((), pos))
}

fn recoginize_many(input: &[u8], mut pos: usize, f: impl Fn(u8) -> bool) -> usize {
    while pos <= input.len() && f(input[pos]) {
        pos += 1;
    }
    pos
}

fn lookup_symbol<'a>(
    symbol: &'a str,
    loc: Loc,
    keywords: &HashMap<&str, TokenKind<'a>>,
) -> Token<'a> {
    if let Some(kind) = keywords.get(symbol) {
        return Token::to_token(kind.clone(), loc);
    } else {
        return Token::symbol(symbol, loc);
    }
}

fn peek_char(input: &[u8], pos: usize) -> Option<u8> {
    let nxt = pos + 1;
    if nxt >= input.len() {
        None
    } else {
        Some(input[nxt])
    }
}

// lexのヘルパーメソッド
fn lex_at_sign(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, line, start, b'@')
        .map(|(_, end)| (Token::at_sign(Loc::new(line, start, end)), end))
}

fn lex_number(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_symbol(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_equal(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, line, start, b'=')
        .map(|(_, end)| (Token::equal(Loc::new(line, start, end)), end))
}

fn lex_plus(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, line, start, b'+')
        .map(|(_, end)| (Token::plus(Loc::new(line, start, end)), end))
}

fn lex_minus(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, line, start, b'-')
        .map(|(_, end)| (Token::minus(Loc::new(line, start, end)), end))
}

fn lex_and(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, line, start, b'&')
        .map(|(_, end)| (Token::and(Loc::new(line, start, end)), end))
}

fn lex_or(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, line, start, b'|')
        .map(|(_, end)| (Token::or(Loc::new(line, start, end)), end))
}

fn lex_bang(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, line, start, b'!')
        .map(|(_, end)| (Token::bang(Loc::new(line, start, end)), end))
}

fn lex_semicolon(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, line, start, b';')
        .map(|(_, end)| (Token::semicolon(Loc::new(line, start, end)), end))
}

fn lex_lparen(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, line, start, b'(')
        .map(|(_, end)| (Token::lparen(Loc::new(line, start, end)), end))
}

fn lex_rparen(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, line, start, b')')
        .map(|(_, end)| (Token::rparen(Loc::new(line, start, end)), end))
}

fn lex_screen(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_kbd(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

///////////////////////////////////////////////////////////////////////////////////////////////////
///
///
///
///////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_lex() {
    let input = "@+-&|! @ ;()=";
    let res = lex(input, 0);
    assert!(res.is_ok());
    let tokens = res.ok().unwrap();
    let expect = vec![
        Token::at_sign(Loc::new(0, 0, 1)),
        Token::plus(Loc::new(0, 1, 2)),
        Token::minus(Loc::new(0, 2, 3)),
        Token::and(Loc::new(0, 3, 4)),
        Token::or(Loc::new(0, 4, 5)),
        Token::bang(Loc::new(0, 5, 6)),
        Token::at_sign(Loc::new(0, 7, 8)),
        Token::semicolon(Loc::new(0, 9, 10)),
        Token::lparen(Loc::new(0, 10, 11)),
        Token::rparen(Loc::new(0, 11, 12)),
        Token::equal(Loc::new(0, 12, 13)),
    ];
    assert_eq!(tokens.len(), expect.len());
    for (i, tok) in tokens.into_iter().enumerate() {
        assert_eq!(expect[i], tok);
    }
}
