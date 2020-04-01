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
                let cmd = parse_a(&mut ram_address, line, &mut symboltable)?;
                commands.push(cmd);
            }
            _ => unimplemented!(),
        }
    }
    Ok(commands)
}

fn parse_a(
    ram_address: &mut u16,
    tokens: &Vec<Token>,
    symboltable: &mut SymbolTable,
) -> Result<Command, ParseError> {
    macro_rules! set_command {
        ($n:expr) => {{
            let loc = tokens[0].loc().merge(tokens[1].loc());
            Ok(Command::a($n, loc))
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
        TokenKind::Symbol(s) => match symboltable.get_address(s) {
            Some(&adr) => set_command!(adr),
            _ => {
                let adr = *ram_address;
                *ram_address += 1;
                set_command!(adr)
            }
        },
        _ => Err(ParseError::UnexpectedToken(tokens[1].clone())),
    }
}

fn parse_c(tokens: &Vec<Token>) -> Result<Command, ParseError> {
    unimplemented!()
}

fn parse_label(tokens: &Vec<Token>) -> Result<(), ParseError> {
    unimplemented!()
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
    //     @R1
    //     D=D-M            
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
        Command::c(0, 0, 0, Loc::new(1, 0, 0)),
        Command::a(0x0001, Loc::new(2, 0, 0)),
        Command::c(0, 0, 0, Loc::new(3, 0, 0)),
        Command::a(0, Loc::new(4, 0, 0)),
        Command::c(0, 0, 0, Loc::new(5, 0, 0)),
        Command::a(0x0001, Loc::new(6, 0, 0)),
        Command::c(0, 0, 0, Loc::new(7, 0, 0)),
        Command::a(0, Loc::new(8, 0, 0)),
        Command::c(0, 0, 0, Loc::new(9, 0, 0)),
        Command::a(0x0000, Loc::new(11, 0, 0)),
        Command::c(0, 0, 0, Loc::new(12, 0, 0)),
        Command::a(0x0002, Loc::new(13, 0, 0)),
    ];

    let mut tokens = lex_all(&mut input).unwrap();
    let cmds = parse(&mut tokens).unwrap();
    for (i, cmd) in cmds.into_iter().enumerate() {
        assert_eq!(expect[i], cmd);
    }
}
