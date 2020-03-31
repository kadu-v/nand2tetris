//! Lexer for Hack Assembler

use crate::lexerror::*;
use crate::loc::*;
use crate::token::*;
use std::collections::HashMap;

pub fn lex(input: &str, line: usize) -> Result<Vec<Token>, LexError> {
    let keywords = [
        ("A", TokenKind::A),
        ("M", TokenKind::M),
        ("D", TokenKind::D),
        ("AM", TokenKind::AM),
        ("AD", TokenKind::AD),
        ("MD", TokenKind::MD),
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
            b'0'..=b'9' => lex_a_token!(lex_number(input, line, pos)),
            b'a'..=b'z' | b'A'..=b'Z' | b'.' | b'_' | b'$' => {
                lex_a_token!(lex_symbol_or_keywords(input, line, pos, &keywords))
            }
            b'/' => {
                if peek_char(input, pos).map_or(false, |c| c == b'/') {
                    break;
                } else {
                    return Err(LexError::invalid_char('/', Loc::new(line, pos, pos + 1)));
                }
            }
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
    while pos < input.len() && f(input[pos]) {
        pos += 1;
    }
    pos
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
    use std::str::from_utf8;
    // 入力に数字が続くかぎり進める
    let end = recoginize_many(input, start, |b| b"1234567890".contains(&b));
    let n = from_utf8(&input[start..end]).unwrap().parse().unwrap();
    Ok((Token::number(n, Loc::new(line, start, end)), end))
}

fn lex_symbol(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    use std::str::from_utf8;
    let end = recoginize_many(input, start, |b| {
        let c = b as char;
        c.is_alphabetic() || c.is_digit(10) || b"_.$".contains(&b)
    });
    match from_utf8(&input[start..end]) {
        Ok(s) => Ok((Token::symbol(s, Loc::new(line, start, end)), end)),
        _ => Err(LexError::invalid_char(
            input[end] as char,
            Loc::new(line, start, end),
        )),
    }
}

// ここのライフタイムを完全には理解していない
fn lex_symbol_or_keywords<'a>(
    input: &'a [u8],
    line: usize,
    start: usize,
    keywords: &HashMap<&str, TokenKind<'a>>,
) -> Result<(Token<'a>, usize), LexError> {
    let (tok, p) = lex_symbol(input, line, start)?;
    let symbol = tok.get_symbol().unwrap();
    match keywords.get(symbol) {
        Some(kind) => Ok((Token::new(kind.clone(), Loc::new(line, start, p)), p)),
        _ => Ok((tok, p)),
    }
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
    let input = "@+-&|! @ ;()= 1234 SUM \
                 i Loop __register__ _x. $y \
                 A M D AM AD MD AMD \
                 JGT JEQ JGE JLT JNE JLE JMP \
                 SP LCL ARG THIS THAT \
                 R0 R1 R2 R3 R4 R5 R6 R7 R8 R9 R10 R11 R12 R13 R14 R15 \
                 SCREEN KBD // this is a comment, so it must be ignored.";
    let res = lex(input, 0);
    //assert_eq!(res, Err(LexError::eof(Loc::new(0, 1, 2))));
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
        Token::number(1234, Loc::new(0, 14, 18)),
        Token::symbol("SUM", Loc::new(0, 19, 22)),
        Token::symbol("i", Loc::new(0, 23, 24)),
        Token::symbol("Loop", Loc::new(0, 25, 29)),
        Token::symbol("__register__", Loc::new(0, 30, 42)),
        Token::symbol("_x.", Loc::new(0, 43, 46)),
        Token::symbol("$y", Loc::new(0, 47, 49)),
        Token::to_token(TokenKind::A, Loc::new(0, 50, 51)),
        Token::to_token(TokenKind::M, Loc::new(0, 52, 53)),
        Token::to_token(TokenKind::D, Loc::new(0, 54, 55)),
        Token::to_token(TokenKind::AM, Loc::new(0, 56, 58)),
        Token::to_token(TokenKind::AD, Loc::new(0, 59, 61)),
        Token::to_token(TokenKind::MD, Loc::new(0, 62, 64)),
        Token::to_token(TokenKind::AMD, Loc::new(0, 65, 68)),
        Token::to_token(TokenKind::JGT, Loc::new(0, 69, 72)),
        Token::to_token(TokenKind::JEQ, Loc::new(0, 73, 76)),
        Token::to_token(TokenKind::JGE, Loc::new(0, 77, 80)),
        Token::to_token(TokenKind::JLT, Loc::new(0, 81, 84)),
        Token::to_token(TokenKind::JNE, Loc::new(0, 85, 88)),
        Token::to_token(TokenKind::JLE, Loc::new(0, 89, 92)),
        Token::to_token(TokenKind::JMP, Loc::new(0, 93, 96)),
        Token::to_token(TokenKind::SP, Loc::new(0, 97, 99)),
        Token::to_token(TokenKind::LCL, Loc::new(0, 100, 103)),
        Token::to_token(TokenKind::ARG, Loc::new(0, 104, 107)),
        Token::to_token(TokenKind::THIS, Loc::new(0, 108, 112)),
        Token::to_token(TokenKind::THAT, Loc::new(0, 113, 117)),
        Token::to_token(TokenKind::R0, Loc::new(0, 118, 120)),
        Token::to_token(TokenKind::R1, Loc::new(0, 121, 123)),
        Token::to_token(TokenKind::R2, Loc::new(0, 124, 126)),
        Token::to_token(TokenKind::R3, Loc::new(0, 127, 129)),
        Token::to_token(TokenKind::R4, Loc::new(0, 130, 132)),
        Token::to_token(TokenKind::R5, Loc::new(0, 133, 135)),
        Token::to_token(TokenKind::R6, Loc::new(0, 136, 138)),
        Token::to_token(TokenKind::R7, Loc::new(0, 139, 141)),
        Token::to_token(TokenKind::R8, Loc::new(0, 142, 144)),
        Token::to_token(TokenKind::R9, Loc::new(0, 145, 147)),
        Token::to_token(TokenKind::R10, Loc::new(0, 148, 151)),
        Token::to_token(TokenKind::R11, Loc::new(0, 152, 155)),
        Token::to_token(TokenKind::R12, Loc::new(0, 156, 159)),
        Token::to_token(TokenKind::R13, Loc::new(0, 160, 163)),
        Token::to_token(TokenKind::R14, Loc::new(0, 164, 167)),
        Token::to_token(TokenKind::R15, Loc::new(0, 168, 171)),
        Token::to_token(TokenKind::SCREEN, Loc::new(0, 172, 178)),
        Token::to_token(TokenKind::KBD, Loc::new(0, 179, 182)),
    ];
    assert_eq!(tokens.len(), expect.len());
    for (i, tok) in tokens.into_iter().enumerate() {
        assert_eq!(expect[i], tok);
    }
}
