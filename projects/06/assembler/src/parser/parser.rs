//! The symbol table for Hack Assembler.

use crate::code::*;
use crate::lexer::token::*;
use crate::parser::command::*;
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
            TokenKind::LParen => continue,
            _ => return Err(ParseError::UnexpectedToken(line[0].clone())),
        }
    }
    Ok(commands)
}

fn parse_a(
    ram_address: u16,
    tokens: &Vec<Token>,
    symboltable: &mut SymbolTable,
) -> Result<(Command, u16), ParseError> {
    match tokens.len() {
        1 => return Err(ParseError::UnexpectedToken(tokens[0].clone())),
        2 => {
            match tokens[1].value() {
                TokenKind::Symbol(s) => {
                    if let Some(&adr) = symboltable.get_address(s) {
                        Ok((Command::a(adr, tokens[0].loc().clone()), ram_address))
                    } else {
                        symboltable.add_entry(s.clone(), ram_address); // clone()している
                        Ok((
                            Command::a(ram_address, tokens[0].loc().clone()), // clone()してる
                            ram_address + 1,
                        ))
                    }
                }
                tokkind => match value(tokkind) {
                    Some(adr) => Ok((Command::a(adr, tokens[0].loc().clone()), ram_address)),
                    _ => Err(ParseError::UnexpectedToken(tokens[0].clone())),
                },
            }
        }
        _ => Err(ParseError::RedundantToken(tokens[3].clone())),
    }
}

fn parse_c(ram_address: u16, tokens: &Vec<Token>) -> Result<(Command, u16), ParseError> {
    macro_rules! cmd_asign {
        ($tok1:expr, $tok2:expr, $tok3:expr, $tok4:expr, $n:expr) => {{
            let loc = tokens[0].loc().merge(tokens[$n].loc());
            match (dest($tok1), comp($tok2, $tok3, $tok4)) {
                (Some(dest), Some(comp)) => Ok((Command::c(comp, dest, 0, loc), ram_address + 1)),
                (Some(_), None) => Err(ParseError::UnexpectedToken(tokens[$n].clone())),
                _ => Err(ParseError::UnexpectedToken(tokens[0].clone())),
            }
        }};
    }

    macro_rules! cmd_jump {
        ($tok1:expr, $tok2:expr, $tok3:expr, $tok4:expr, $n: expr) => {{
            let loc = tokens[0].loc().merge(tokens[$n].loc());
            match (comp($tok1, $tok2, $tok3), jump($tok4)) {
                (Some(comp), Some(jump)) => Ok((Command::c(comp, 0, jump, loc), ram_address + 1)),
                (None, Some(_)) => Err(ParseError::UnexpectedToken(tokens[$n].clone())),
                _ => Err(ParseError::UnexpectedToken(tokens[0].clone())),
            }
        }};
    }
    match tokens.len() {
        1 | 2 => Err(ParseError::UnexpectedToken(tokens[0].clone())),
        3 => match (tokens[0].value(), tokens[1].value(), tokens[2].value()) {
            (tokkind1, TokenKind::Equal, tokkind2) => {
                cmd_asign!(tokkind1, Some(tokkind2), None, None, 2)
            }
            (tokkind1, TokenKind::Semicolon, tokkind2) => {
                cmd_jump!(Some(tokkind1), None, None, tokkind2, 2)
            }
            _ => Err(ParseError::UnexpectedToken(tokens[0].clone())),
        },

        4 => {
            match (
                tokens[0].value(),
                tokens[1].value(),
                tokens[2].value(),
                tokens[3].value(),
            ) {
                (tokkind1, TokenKind::Equal, tokkind2, tokkind3) => {
                    cmd_asign!(tokkind1, Some(tokkind2), Some(tokkind3), None, 3)
                }
                (tokkind1, tokkind2, TokenKind::Semicolon, tokkind3) => {
                    cmd_jump!(Some(tokkind1), Some(tokkind2), None, tokkind3, 3)
                }
                _ => Err(ParseError::UnexpectedToken(tokens[0].clone())),
            }
        }
        5 => {
            match (
                tokens[0].value(),
                tokens[1].value(),
                tokens[2].value(),
                tokens[3].value(),
                tokens[4].value(),
            ) {
                (tokkind1, TokenKind::Equal, tokkind2, tokkind3, tokkind4) => {
                    cmd_asign!(tokkind1, Some(tokkind2), Some(tokkind3), Some(tokkind4), 3)
                }
                (tokkind1, tokkind2, tokkind3, TokenKind::Semicolon, tokkind4) => {
                    cmd_jump!(Some(tokkind1), Some(tokkind2), Some(tokkind3), tokkind4, 3)
                }
                _ => Err(ParseError::UnexpectedToken(tokens[0].clone())),
            }
        }
        _ => Err(ParseError::UnexpectedToken(tokens[0].clone())),
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
///
///
///
//////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_parser() {
    use crate::annot::*;
    use crate::lexer::lexer::*;
    use crate::loc::*;
    let mut input = "
   @R0
   D=M              // D = first number
   @R1
   D=D-M            // D = first number - second number
   @OUTPUT_FIRST
   D;JGT            // if D>0 (first is greater) goto output_first
   @R1
   D=M              // D = second number
   @OUTPUT_D
   0;JMP            // goto output_d
(OUTPUT_FIRST)
   @R0             
   D=M              // D = first number
(OUTPUT_D)
   @R2
   M=D              // M[2] = D (greatest number)
(INFINITE_LOOP)
   @INFINITE_LOOP
   0;JMP            // infinite loop
        "
    .as_bytes();
    let expect = [
        CommandKind::A(0b000000000000000),
        CommandKind::C(0b1110000, 0b010, 0b000),
        CommandKind::A(0b00000000000001),
        CommandKind::C(0b1010011, 0b010, 0b00),
        CommandKind::A(0b00000000001010),
        CommandKind::C(0b0001100, 0b000, 0b001),
        CommandKind::A(0b000000000000001),
        CommandKind::C(0b1110000, 0b010, 0b000),
        CommandKind::A(0b000000000001100),
        CommandKind::C(0b0101010, 0b000, 0b111),
        CommandKind::A(0b000000000000000),
        CommandKind::C(0b1110000, 0b010, 0b000),
        CommandKind::A(0b000000000000010),
        CommandKind::C(0b0001100, 0b001, 0b000),
        CommandKind::A(0b000000000001110),
        CommandKind::C(0b0101010, 0b000, 0b111),
    ];

    let mut tokens = lex_all(&mut input).unwrap();
    let cmds = parse(&mut tokens).unwrap();
    for (i, cmd) in cmds.into_iter().enumerate() {
        let result = Annot::new(expect[i].clone(), Loc::new(0, 0, 0));
        assert!(
            result.value() == cmd.value(),
            "\nresult: {:?},\n\
             cmd:    {:?}",
            result,
            cmd
        )
    }
}
