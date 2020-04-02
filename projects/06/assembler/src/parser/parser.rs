//! The symbol table for Hack Assembler.

use crate::lexer::token::*;
use crate::loc::*;
use crate::parser::command::*;
use crate::parser::parseerror;
use crate::parser::parseerror::*;
use crate::symboltable::*;

pub fn parse(lines: &Vec<Vec<Token>>) -> Result<Vec<Command>, ParseError> {
    let mut symboltable = SymbolTable::make(&lines);
    let mut commands = Vec::new();
    let mut ram_address = 0;
    for line in lines {
        match line[0].value() {
            TokenKind::AtSign => {
                let (cmd, adr) = parse_a(ram_address, line, &mut symboltable)?;
                ram_address = adr;
                commands.push(cmd);
            }
            TokenKind::A
            | TokenKind::M
            | TokenKind::D
            | TokenKind::AM
            | TokenKind::AD
            | TokenKind::MD
            | TokenKind::AMD
            | TokenKind::Number(0)
            | TokenKind::Number(1) => {
                let (cmd, adr) = parse_c(ram_address, line)?;
                ram_address = adr;
                commands.push(cmd);
            }
            _ => unimplemented!(),
        }
    }
    Ok(commands)
}

fn parse_a(
    ram_address: u16,
    tokens: &Vec<Token>,
    symboltable: &mut SymbolTable,
) -> Result<(Command, u16), ParseError> {
    macro_rules! set_command {
        ($n:expr) => {{
            let loc = tokens[0].loc().merge(tokens[1].loc());
            Ok((Command::a($n, loc), ram_address + 1))
        }};
    }
    if tokens.len() == 1 {
        return Err(ParseError::UnexpectedToken(tokens[0].clone()));
    } else if tokens.len() >= 3 {
        return Err(ParseError::RedundantToken(tokens[2].clone()));
    }
    match tokens[1].value() {
        TokenKind::R0 | TokenKind::SP => set_command!(0x0000),
        TokenKind::R1 | TokenKind::LCL => set_command!(0x0001),
        TokenKind::R2 | TokenKind::ARG => set_command!(0x0002),
        TokenKind::R3 | TokenKind::THIS => set_command!(0x0003),
        TokenKind::R4 | TokenKind::THAT => set_command!(0x0004),
        TokenKind::R5 => set_command!(0x0005),
        TokenKind::R6 => set_command!(0x0006),
        TokenKind::R7 => set_command!(0x0007),
        TokenKind::R8 => set_command!(0x0008),
        TokenKind::R9 => set_command!(0x0009),
        TokenKind::R10 => set_command!(0x000a),
        TokenKind::R11 => set_command!(0x000b),
        TokenKind::R12 => set_command!(0x000c),
        TokenKind::R13 => set_command!(0x000d),
        TokenKind::R14 => set_command!(0x000e),
        TokenKind::R15 => set_command!(0x000f),
        TokenKind::SCREEN => set_command!(0x4000),
        TokenKind::KBD => set_command!(0x6000),
        TokenKind::Number(n) => set_command!(*n as u16),
        TokenKind::Symbol(s) => match symboltable.get_address(s) {
            Some(&adr) => set_command!(adr),
            _ => set_command!(ram_address),
        },
        _ => Err(ParseError::UnexpectedToken(tokens[1].clone())),
    }
}

fn parse_c(ram_address: u16, tokens: &Vec<Token>) -> Result<(Command, u16), ParseError> {
    if tokens.len() == 1 {
        return Err(ParseError::UnexpectedToken(tokens[0].clone()));
    } else if tokens.len() == 2 {
        return Err(ParseError::UnexpectedToken(tokens[1].clone()));
    }
    match tokens[1].value() {
        TokenKind::Equal => parse_c_asign(ram_address, tokens),
        TokenKind::Semicolon => parse_c_jump(ram_address, tokens),
        _ => Err(ParseError::UnexpectedToken(tokens[2].clone())),
    }
}

fn parse_c_asign(ram_address: u16, tokens: &Vec<Token>) -> Result<(Command, u16), ParseError> {
    let mut comp = 0;
    let mut dest = 0;
    macro_rules! set {
        ($id: ident, $dst: expr) => {
            $id = $dst;
        };
    }
    // dest
    match tokens[0].value() {
        TokenKind::M => set!(dest, 0b001),
        TokenKind::D => set!(dest, 0b010),
        TokenKind::MD => set!(dest, 0b011),
        TokenKind::A => set!(dest, 0b100),
        TokenKind::AM => set!(dest, 0b101),
        TokenKind::AD => set!(dest, 0b110),
        TokenKind::AMD => set!(dest, 0b1111),
        _ => {
            return Err(ParseError::UnexpectedToken(tokens[0].clone()));
        }
    }

    // comp
    match tokens.len() {
        3 => match tokens[2].value() {
            TokenKind::Number(0) => set!(comp, 0b0101010),
            TokenKind::Number(1) => set!(comp, 0b0111111),
            TokenKind::D => set!(comp, 0b0001100),
            TokenKind::A => set!(comp, 0b0110000),
            TokenKind::M => set!(comp, 0b1110000),
            _ => return Err(ParseError::UnexpectedToken(tokens[2].clone())),
        },
        4 => match (tokens[2].value(), tokens[3].value()) {
            (TokenKind::Bang, TokenKind::D) => set!(comp, 0b0001101),
            (TokenKind::Bang, TokenKind::A) => set!(comp, 0b0110001),
            (TokenKind::Minus, TokenKind::D) => set!(comp, 0b0001111),
            (TokenKind::Minus, TokenKind::A) => set!(comp, 0b0110011),
            (TokenKind::Bang, TokenKind::M) => set!(comp, 0b1110001),
            (TokenKind::Minus, TokenKind::M) => set!(comp, 0b1110011),
            _ => return Err(ParseError::UnexpectedToken(tokens[2].clone())),
        },
        5 => match (tokens[2].value(), tokens[3].value(), tokens[4].value()) {
            (TokenKind::D, TokenKind::Plus, TokenKind::Number(1)) => set!(comp, 0b0011111),
            (TokenKind::A, TokenKind::Plus, TokenKind::Number(1)) => set!(comp, 0b0110111),
            (TokenKind::D, TokenKind::Minus, TokenKind::Number(1)) => set!(comp, 0b0001110),
            (TokenKind::A, TokenKind::Minus, TokenKind::Number(1)) => set!(comp, 0b0110010),
            (TokenKind::D, TokenKind::Plus, TokenKind::A) => set!(comp, 0b0000010),
            (TokenKind::D, TokenKind::Minus, TokenKind::A) => set!(comp, 0b0010011),
            (TokenKind::A, TokenKind::Minus, TokenKind::D) => set!(comp, 0b0010011),
            (TokenKind::D, TokenKind::And, TokenKind::A) => set!(comp, 0b0000000),
            (TokenKind::D, TokenKind::Or, TokenKind::A) => set!(comp, 0b0010101),
            (TokenKind::M, TokenKind::Plus, TokenKind::Number(1)) => set!(comp, 0b1110111),
            (TokenKind::M, TokenKind::Minus, TokenKind::Number(1)) => set!(comp, 0b1110010),
            (TokenKind::D, TokenKind::Plus, TokenKind::M) => set!(comp, 0b1000010),
            (TokenKind::D, TokenKind::Minus, TokenKind::M) => set!(comp, 0b1000111),
            (TokenKind::M, TokenKind::Minus, TokenKind::D) => set!(comp, 0b1010011),
            (TokenKind::D, TokenKind::And, TokenKind::M) => set!(comp, 0b10000000),
            (TokenKind::D, TokenKind::Or, TokenKind::M) => set!(comp, 0b1010101),
            _ => return Err(ParseError::UnexpectedToken(tokens[2].clone())),
        },
        _ => return Err(ParseError::RedundantToken(tokens[4].clone())),
    }

    Ok((
        Command::c(comp, dest, 0, tokens[2].loc().clone()), // locが不十分　上のマッチ分の中でlocをmergeする必要がある
        ram_address + 1,
    ))
}

fn parse_c_jump(ram_address: u16, tokens: &Vec<Token>) -> Result<(Command, u16), ParseError> {
    let mut comp = 0;
    let mut jump = 0;
    macro_rules! set {
        ($id: ident, $dst: expr) => {
            $id = $dst;
        };
    }
    // comp
    match tokens.len() {
        3 => match tokens[0].value() {
            TokenKind::Number(0) => set!(comp, 0b0101010),
            TokenKind::Number(1) => set!(comp, 0b0111111),
            TokenKind::D => set!(comp, 0b0001100),
            TokenKind::A => set!(comp, 0b0110000),
            TokenKind::M => set!(comp, 0b1110000),
            _ => return Err(ParseError::UnexpectedToken(tokens[2].clone())),
        },
        4 => match (tokens[0].value(), tokens[1].value()) {
            (TokenKind::Bang, TokenKind::D) => set!(comp, 0b0001101),
            (TokenKind::Bang, TokenKind::A) => set!(comp, 0b0110001),
            (TokenKind::Minus, TokenKind::D) => set!(comp, 0b0001111),
            (TokenKind::Minus, TokenKind::A) => set!(comp, 0b0110011),
            (TokenKind::Bang, TokenKind::M) => set!(comp, 0b1110001),
            (TokenKind::Minus, TokenKind::M) => set!(comp, 0b1110011),
            _ => return Err(ParseError::UnexpectedToken(tokens[2].clone())),
        },
        5 => match (tokens[0].value(), tokens[1].value(), tokens[2].value()) {
            (TokenKind::D, TokenKind::Plus, TokenKind::Number(1)) => set!(comp, 0b0011111),
            (TokenKind::A, TokenKind::Plus, TokenKind::Number(1)) => set!(comp, 0b0110111),
            (TokenKind::D, TokenKind::Minus, TokenKind::Number(1)) => set!(comp, 0b0001110),
            (TokenKind::A, TokenKind::Minus, TokenKind::Number(1)) => set!(comp, 0b0110010),
            (TokenKind::D, TokenKind::Plus, TokenKind::A) => set!(comp, 0b0000010),
            (TokenKind::D, TokenKind::Minus, TokenKind::A) => set!(comp, 0b0010011),
            (TokenKind::A, TokenKind::Minus, TokenKind::D) => set!(comp, 0b0010011),
            (TokenKind::D, TokenKind::And, TokenKind::A) => set!(comp, 0b0000000),
            (TokenKind::D, TokenKind::Or, TokenKind::A) => set!(comp, 0b0010101),
            (TokenKind::M, TokenKind::Plus, TokenKind::Number(1)) => set!(comp, 0b1110111),
            (TokenKind::M, TokenKind::Minus, TokenKind::Number(1)) => set!(comp, 0b1110010),
            (TokenKind::D, TokenKind::Plus, TokenKind::M) => set!(comp, 0b1000010),
            (TokenKind::D, TokenKind::Minus, TokenKind::M) => set!(comp, 0b1000111),
            (TokenKind::M, TokenKind::Minus, TokenKind::D) => set!(comp, 0b1010011),
            (TokenKind::D, TokenKind::And, TokenKind::M) => set!(comp, 0b10000000),
            (TokenKind::D, TokenKind::Or, TokenKind::M) => set!(comp, 0b1010101),
            _ => return Err(ParseError::UnexpectedToken(tokens[2].clone())),
        },
        _ => return Err(ParseError::RedundantToken(tokens[4].clone())),
    }

    unimplemented!()
}

fn parse_label(tokens: &Vec<Token>) -> Result<(), ParseError> {
    unimplemented!("parse_label")
}

///////////////////////////////////////////////////////////////////////////////////////////////////
///
///
///
//////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_parser() {
    use crate::lexer::lexer::*;
    let mut input = "\
         @R0
         D=M              
         @R1
    //     D=D-M
    //     D;JEL          
    //     @OUTPUT_FIRST
    //     D;JGT            
    //     @R1
    //     D=M              
    //     @OUTPUT_D
    //     0;JMP           
    //   (OUTPUT_FIRST)
    //     @R0             
    //     D=M            
    //   (OUTPUT_D)
    //     @R2
    //      M=D                     
        "
    .as_bytes();
    let expect = [
        Command::a(0x0000, Loc::new(0, 0, 3)),
        Command::c(0b1110000, 0b010, 0, Loc::new(1, 11, 12)),
        Command::a(0x0001, Loc::new(2, 9, 12)),
        Command::c(0b1010011, 0b010, 0, Loc::new(3, 0, 0)),
        Command::c(0, 0b0001100, 0b100, Loc::new(4, 0, 0)),
        Command::a(0, Loc::new(5, 0, 0)),
        Command::c(0, 0, 0, Loc::new(6, 0, 0)),
        Command::a(0x0001, Loc::new(7, 0, 0)),
        Command::c(0, 0, 0, Loc::new(8, 0, 0)),
        Command::a(0, Loc::new(9, 0, 0)),
        Command::c(0, 0, 0, Loc::new(10, 0, 0)),
        Command::a(0x0000, Loc::new(12, 0, 0)),
        Command::c(0, 0, 0, Loc::new(13, 0, 0)),
        Command::a(0x0002, Loc::new(14, 0, 0)),
        Command::c(0, 0, 0, Loc::new(15, 0, 0)),
    ];

    let mut tokens = lex_all(&mut input).unwrap();
    println!("{:?}", tokens);
    let cmds = parse(&mut tokens).unwrap();
    for (i, cmd) in cmds.into_iter().enumerate() {
        assert_eq!(expect[i], cmd);
    }
}
