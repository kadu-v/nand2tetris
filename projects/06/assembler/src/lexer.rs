//! Lexer for Hack Assembler

use crate::lexerror::*;
use crate::loc::*;
use crate::token::*;

fn lex(input: &str, line: usize) -> Result<Vec<Token>, LexError> {
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
            b'+' => lex_a_token!(lex_plus(input, line, pos)),
            b'-' => lex_a_token!(lex_minus(input, line, pos)),
            b'&' => lex_a_token!(lex_and(input, line, pos)),
            b'|' => lex_a_token!(lex_or(input, line, pos)),
            b'!' => lex_a_token!(lex_bang(input, line, pos)),
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

fn lex_a(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_m(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_d(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_am(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_ad(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_md(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_amd(iinput: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
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

fn lex_jgt(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_jeq(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_jge(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_jlt(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_jne(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_jle(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_jmp(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_semicolon(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_lparen(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_rparen(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_sp(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_lcl(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_arg(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_this(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_that(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
}

fn lex_r(input: &[u8], line: usize, start: usize) -> Result<(Token, usize), LexError> {
    unimplemented!()
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
    let input = "@+-&|! @";
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
    ];
    assert_eq!(tokens.len(), expect.len());
    for (i, tok) in tokens.into_iter().enumerate() {
        assert_eq!(expect[i], tok);
    }
}
