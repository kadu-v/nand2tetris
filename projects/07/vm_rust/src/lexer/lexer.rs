//! Lexer for Hack VM

use crate::lexer::lexerror::*;
use crate::lexer::token::*;
use crate::loc::*;

/// 一行を字句解析するメソッド
pub fn lexer(input: &str, line: usize) -> Result<Vec<Token>, LexError> {
    // 解析結果を保存するベクター
    let mut tokens = Vec::new();
    //　入力
    let input = input.as_bytes();

    // 現在読み込み中の位置を管理する値
    let mut pos = 0;

    while pos < input.len() {
        match input[pos] {
            b'a'..=b'z' | b'A'..=b'Z' | b'.' | b'_' | b'$' => {
                let (tok, p) = lex_symbol(input, line, pos)?;
                let tok = match &input[pos..p] {
                    b"add" => Token::add(tok.loc().clone()),
                    b"sub" => Token::sub(tok.loc().clone()),
                    b"neg" => Token::neg(tok.loc().clone()),
                    b"eq" => Token::eq(tok.loc().clone()),
                    b"gt" => Token::gt(tok.loc().clone()),
                    b"lt" => Token::lt(tok.loc().clone()),
                    b"and" => Token::and(tok.loc().clone()),
                    b"or" => Token::or(tok.loc().clone()),
                    b"not" => Token::not(tok.loc().clone()),
                    b"push" => Token::push(tok.loc().clone()),
                    b"pop" => Token::pop(tok.loc().clone()),
                    b"argument" => Token::argument(tok.loc().clone()),
                    b"local" => Token::local(tok.loc().clone()),
                    b"static" => Token::static_(tok.loc().clone()),
                    b"constant" => Token::constant(tok.loc().clone()),
                    b"this" => Token::this(tok.loc().clone()),
                    b"that" => Token::that(tok.loc().clone()),
                    b"pointer" => Token::pointer(tok.loc().clone()),
                    b"temp" => Token::temp(tok.loc().clone()),
                    _ => tok,
                };
                pos = p;
                tokens.push(tok);
            }
            b'0'..=b'9' => {
                let (tok, p) = lex_number(input, line, pos)?;
                pos = p;
                tokens.push(tok);
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

/// posのバイトが期待するものであれば１バイト消費してposを１進めるヘルパーメソッド
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

/// 空白を飛ばすヘルパーメソッド
fn skip_whitespaces(input: &[u8], pos: usize) -> Result<((), usize), LexError> {
    let pos = recognize_many(input, pos, |b| b" \n\t".contains(&b));
    Ok(((), pos))
}

///　クロージャfを満たさなくなるまで，posを増やし続けるヘルパーメソッド
fn recognize_many(input: &[u8], mut pos: usize, f: impl Fn(u8) -> bool) -> usize {
    while pos < input.len() && f(input[pos]) {
        pos += 1;
    }
    pos
}

///　１文字先読みするヘルパーメソッド
fn peek_char(input: &[u8], pos: usize) -> Option<u8> {
    let nxt = pos + 1;
    if nxt >= input.len() {
        None
    } else {
        Some(input[nxt])
    }
}

/// symbolを字句解析するヘルパーメソッド
fn lex_symbol(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    use std::str::from_utf8;
    let end = recognize_many(input, start, |b| {
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

/// 数字を字句解析するヘルパーメソッド
fn lex_number(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    use std::str::from_utf8;
    // 入力に数字が続くかぎり進める
    let end = recognize_many(input, start, |b| b"1234567890".contains(&b));
    let n = from_utf8(&input[start..end]).unwrap().parse().unwrap();
    Ok((Token::number(n, Loc::new(line, start, end)), end))
}

///////////////////////////////////////////////////////////////////////////////////////////////////
///
/// UNIT TEST
///
///////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_lexer() {
    let input = "add sub neg eq gt lt and or not \
                 push pop \
                 argument local static constant this that pointer temp \
                 1234 identifier symbol.symbol // this is comment";
    let res = lexer(input, 0);
    let tokens = res.ok().unwrap();
    let expect = vec![
        Token::add(Loc::new(0, 0, 3)),
        Token::sub(Loc::new(0, 4, 7)),
        Token::neg(Loc::new(0, 8, 11)),
        Token::eq(Loc::new(0, 12, 14)),
        Token::gt(Loc::new(0, 15, 17)),
        Token::lt(Loc::new(0, 18, 20)),
        Token::and(Loc::new(0, 21, 24)),
        Token::or(Loc::new(0, 25, 27)),
        Token::not(Loc::new(0, 28, 31)),
        Token::push(Loc::new(0, 32, 36)),
        Token::pop(Loc::new(0, 37, 40)),
        Token::argument(Loc::new(0, 41, 49)),
        Token::local(Loc::new(0, 50, 55)),
        Token::static_(Loc::new(0, 56, 62)),
        Token::constant(Loc::new(0, 63, 71)),
        Token::this(Loc::new(0, 72, 76)),
        Token::that(Loc::new(0, 77, 81)),
        Token::pointer(Loc::new(0, 82, 89)),
        Token::temp(Loc::new(0, 90, 94)),
        Token::number(1234, Loc::new(0, 95, 99)),
        Token::symbol("identifier", Loc::new(0, 100, 110)),
        Token::symbol("symbol.symbol", Loc::new(0, 111, 124)),
    ];
    assert_eq!(tokens.len(), expect.len());
    for (i, tok) in tokens.into_iter().enumerate() {
        assert_eq!(expect[i], tok);
    }
}
