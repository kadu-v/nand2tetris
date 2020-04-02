//! Code
use crate::lexer::token::*;
use crate::loc::*;
use crate::parser::command::*;
use crate::parser::parseerror::*;

/// comp to binary
pub fn comp(token1: &Option<Token>, token2: &Option<Token>, token3: &Option<Token>) -> Option<u16> {
    match (token1, token2, token3) {
        (Some(tok), None, None) => match tok.value() {
            TokenKind::Number(0) => Some(0b0101010),
            TokenKind::Number(1) => Some(0b0111111),
            TokenKind::D => Some(0b0001100),
            TokenKind::A => Some(0b0110000),
            TokenKind::M => Some(0b1110000),
            _ => None,
        },
        (Some(tok1), Some(tok2), None) => match (tok1.value(), tok2.value()) {
            (TokenKind::Minus, TokenKind::Number(1)) => Some(0b0111010),
            (TokenKind::Bang, TokenKind::D) => Some(0b0001101),
            (TokenKind::Bang, TokenKind::A) => Some(0b0110001),
            (TokenKind::Minus, TokenKind::D) => Some(0b0001111),
            (TokenKind::Minus, TokenKind::A) => Some(0b0110011),
            (TokenKind::Bang, TokenKind::M) => Some(0b1110001),
            (TokenKind::Minus, TokenKind::M) => Some(0b1110011),
            _ => None,
        },
        (Some(tok1), Some(tok2), Some(tok3)) => match (tok1.value(), tok2.value(), tok3.value()) {
            (TokenKind::D, TokenKind::Plus, TokenKind::Number(1)) => Some(0b0011111),
            (TokenKind::A, TokenKind::Plus, TokenKind::Number(1)) => Some(0b0110111),
            (TokenKind::D, TokenKind::Minus, TokenKind::Number(1)) => Some(0b0001110),
            (TokenKind::A, TokenKind::Minus, TokenKind::Number(1)) => Some(0b0110010),
            (TokenKind::D, TokenKind::Plus, TokenKind::A) => Some(0b0000010),
            (TokenKind::D, TokenKind::Minus, TokenKind::A) => Some(0b0010011),
            (TokenKind::A, TokenKind::Minus, TokenKind::D) => Some(0b0000111),
            (TokenKind::D, TokenKind::And, TokenKind::A) => Some(0b0000000),
            (TokenKind::D, TokenKind::Or, TokenKind::A) => Some(0b0010101),
            (TokenKind::M, TokenKind::Plus, TokenKind::Number(1)) => Some(0b1110111),
            (TokenKind::M, TokenKind::Minus, TokenKind::Number(1)) => Some(0b1110010),
            (TokenKind::D, TokenKind::Plus, TokenKind::M) => Some(0b1000010),
            (TokenKind::D, TokenKind::Minus, TokenKind::M) => Some(0b1010011),
            (TokenKind::M, TokenKind::Minus, TokenKind::D) => Some(0b1000111),
            (TokenKind::D, TokenKind::And, TokenKind::M) => Some(0b1000000),
            (TokenKind::D, TokenKind::Or, TokenKind::M) => Some(0b1010101),
            _ => None,
        },
        _ => None,
    }
}

/// dest
pub fn dest(token: &Token) -> Option<u16> {
    match token.value() {
        TokenKind::R0 | TokenKind::SP => Some(0x0000),
        TokenKind::R1 | TokenKind::LCL => Some(0x0001),
        TokenKind::R2 | TokenKind::ARG => Some(0x0002),
        TokenKind::R3 | TokenKind::THIS => Some(0x0003),
        TokenKind::R4 | TokenKind::THAT => Some(0x0004),
        TokenKind::R5 => Some(0x0005),
        TokenKind::R6 => Some(0x0006),
        TokenKind::R7 => Some(0x0007),
        TokenKind::R8 => Some(0x0008),
        TokenKind::R9 => Some(0x0009),
        TokenKind::R10 => Some(0x000a),
        TokenKind::R11 => Some(0x000b),
        TokenKind::R12 => Some(0x000c),
        TokenKind::R13 => Some(0x000d),
        TokenKind::R14 => Some(0x000e),
        TokenKind::R15 => Some(0x000f),
        TokenKind::SCREEN => Some(0x4000),
        TokenKind::KBD => Some(0x6000),
        TokenKind::Number(n) => Some(*n as u16),
        _ => None,
    }
}

// jump
pub fn jump(token: &Token) -> Option<u16> {
    match token.value() {
        TokenKind::JGT => Some(0b001),
        TokenKind::JEQ => Some(0b010),
        TokenKind::JGE => Some(0b011),
        TokenKind::JLT => Some(0b100),
        TokenKind::JNE => Some(0b101),
        TokenKind::JLE => Some(0b110),
        TokenKind::JMP => Some(0b111),
        _ => None,
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//
//
//
///////////////////////////////////////////////////////////////////////////////////////////////////
#[test]
fn test_comp() {
    let tokens = [
        (Some(TokenKind::Number(0)), None, None),
        (Some(TokenKind::Number(1)), None, None),
        (Some(TokenKind::Minus), Some(TokenKind::Number(1)), None),
        (Some(TokenKind::D), None, None),
        (Some(TokenKind::A), None, None),
        (Some(TokenKind::Bang), Some(TokenKind::D), None),
        (Some(TokenKind::Bang), Some(TokenKind::A), None),
        (Some(TokenKind::Minus), Some(TokenKind::D), None),
        (Some(TokenKind::Minus), Some(TokenKind::A), None),
        (
            Some(TokenKind::D),
            Some(TokenKind::Plus),
            Some(TokenKind::Number(1)),
        ),
        (
            Some(TokenKind::A),
            Some(TokenKind::Plus),
            Some(TokenKind::Number(1)),
        ),
        (
            Some(TokenKind::D),
            Some(TokenKind::Minus),
            Some(TokenKind::Number(1)),
        ),
        (
            Some(TokenKind::A),
            Some(TokenKind::Minus),
            Some(TokenKind::Number(1)),
        ),
        (
            Some(TokenKind::D),
            Some(TokenKind::Plus),
            Some(TokenKind::A),
        ),
        (
            Some(TokenKind::D),
            Some(TokenKind::Minus),
            Some(TokenKind::A),
        ),
        (
            Some(TokenKind::A),
            Some(TokenKind::Minus),
            Some(TokenKind::D),
        ),
        (Some(TokenKind::D), Some(TokenKind::And), Some(TokenKind::A)),
        (Some(TokenKind::D), Some(TokenKind::Or), Some(TokenKind::A)),
        (Some(TokenKind::M), None, None),
        (Some(TokenKind::Bang), Some(TokenKind::M), None),
        (Some(TokenKind::Minus), Some(TokenKind::M), None),
        (
            Some(TokenKind::M),
            Some(TokenKind::Plus),
            Some(TokenKind::Number(1)),
        ),
        (
            Some(TokenKind::M),
            Some(TokenKind::Minus),
            Some(TokenKind::Number(1)),
        ),
        (
            Some(TokenKind::D),
            Some(TokenKind::Plus),
            Some(TokenKind::M),
        ),
        (
            Some(TokenKind::D),
            Some(TokenKind::Minus),
            Some(TokenKind::M),
        ),
        (
            Some(TokenKind::M),
            Some(TokenKind::Minus),
            Some(TokenKind::D),
        ),
        (Some(TokenKind::D), Some(TokenKind::And), Some(TokenKind::M)),
        (Some(TokenKind::D), Some(TokenKind::Or), Some(TokenKind::M)),
    ];

    let expect: [Option<u16>; 28] = [
        Some(0b0101010),
        Some(0b0111111),
        Some(0b0111010),
        Some(0b0001100),
        Some(0b0110000),
        Some(0b0001101),
        Some(0b0110001),
        Some(0b0001111),
        Some(0b0110011),
        Some(0b0011111),
        Some(0b0110111),
        Some(0b0001110),
        Some(0b0110010),
        Some(0b0000010),
        Some(0b0010011),
        Some(0b0000111),
        Some(0b0000000),
        Some(0b0010101),
        Some(0b1110000),
        Some(0b1110001),
        Some(0b1110011),
        Some(0b1110111),
        Some(0b1110010),
        Some(0b1000010),
        Some(0b1010011),
        Some(0b1000111),
        Some(0b1000000),
        Some(0b1010101),
    ];

    for (i, token) in tokens.into_iter().enumerate() {
        let (tok1, tok2, tok3) = token;
        let tok1 = tok1
            .clone()
            .map(|tok| Token::to_token(tok, Loc::new(0, 0, 0)));
        let tok2 = tok2
            .clone()
            .map(|tok| Token::to_token(tok, Loc::new(0, 0, 0)));
        let tok3 = tok3
            .clone()
            .map(|tok| Token::to_token(tok, Loc::new(0, 0, 0)));
        let result = comp(&tok1, &tok2, &tok3);
        assert!(
            result == expect[i],
            "tok1: {:?}, tok2: {:?}, tok3: {:?}, expect: {:07b}, result: {:07b}",
            tok1,
            tok2,
            tok3,
            expect[i].unwrap_or(0),
            result.unwrap_or(0)
        )
    }
}

#[test]
fn test_dest() {
    let tokens = [
        TokenKind::R0,
        TokenKind::R1,
        TokenKind::R2,
        TokenKind::R3,
        TokenKind::R4,
        TokenKind::R5,
        TokenKind::R6,
        TokenKind::R7,
        TokenKind::R8,
        TokenKind::R9,
        TokenKind::R10,
        TokenKind::R11,
        TokenKind::R12,
        TokenKind::R13,
        TokenKind::R14,
        TokenKind::R15,
        TokenKind::SP,
        TokenKind::LCL,
        TokenKind::ARG,
        TokenKind::THIS,
        TokenKind::THAT,
        TokenKind::SCREEN,
        TokenKind::KBD,
    ];

    let expect = [
        Some(0x0000),
        Some(0x0001),
        Some(0x0002),
        Some(0x0003),
        Some(0x0004),
        Some(0x0005),
        Some(0x0006),
        Some(0x0007),
        Some(0x0008),
        Some(0x0009),
        Some(0x000a),
        Some(0x000b),
        Some(0x000c),
        Some(0x000d),
        Some(0x000e),
        Some(0x000f),
        Some(0x0000),
        Some(0x0001),
        Some(0x0002),
        Some(0x0003),
        Some(0x0004),
        Some(0x4000),
        Some(0x6000),
    ];
    use crate::lexer::token::*;

    for (i, kind) in tokens.into_iter().enumerate() {
        let tok = Token::to_token(kind.clone(), Loc::new(0, 0, 0));
        assert_eq!(expect[i], dest(&tok));
    }
}

#[test]
fn test_jump() {
    let tokens = [
        Token::jgt(Loc::new(0, 0, 0)),
        Token::jeq(Loc::new(0, 0, 0)),
        Token::jge(Loc::new(0, 0, 0)),
        Token::jlt(Loc::new(0, 0, 0)),
        Token::jne(Loc::new(0, 0, 0)),
        Token::jle(Loc::new(0, 0, 0)),
        Token::jmp(Loc::new(0, 0, 0)),
    ];

    let expect = [
        Some(0b001),
        Some(0b010),
        Some(0b011),
        Some(0b100),
        Some(0b101),
        Some(0b110),
        Some(0b111),
    ];
    for (i, tok) in tokens.into_iter().enumerate() {
        assert_eq!(expect[i], jump(tok));
    }
}
