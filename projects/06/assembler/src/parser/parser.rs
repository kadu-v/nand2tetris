//! The symbol table for Hack Assembler.

use crate::code::*;
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
                _ => match value(&tokens[1]) {
                    Some(adr) => Ok((Command::a(adr, tokens[0].loc().clone()), ram_address)),
                    _ => Err(ParseError::UnexpectedToken(tokens[0].clone())),
                },
            }
        }
        _ => Err(ParseError::RedundantToken(tokens[3].clone())),
    }
}

fn parse_c(ram_address: u16, tokens: &Vec<Token>) -> Result<(Command, u16), ParseError> {
    //     let mut comp = 0;
    // let mut dest = 0;
    // // dest
    // match tokens.len() {
    //     1 | 2 => {return Err(ParseError::UnexpectedToken(tokens[0].clone()))},
    //     3 => {
    //         match tokens[1].value() {
    //             TokenKind::Equal => {
    //             },
    //             _  => ,
    //         }
    //     }
    //     _  => ,
    // }
    unimplemented!()
}

///////////////////////////////////////////////////////////////////////////////////////////////////
///
///
///
//////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_parser() {
    fn eq(cmd: Command, other: Command) -> bool {
        match (cmd.value(), other.value()) {
            (CommandKind::A(_), CommandKind::A(_)) => true,
            (CommandKind::C(_, _, _), CommandKind::C(_, _, _)) => true,
            _ => false,
        }
    }
    use crate::annot::*;
    use crate::lexer::lexer::*;
    let mut input = "\
         @R0
        D=M              
     //    @R1
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
        CommandKind::A(0),
        CommandKind::C(0, 0, 0),
        CommandKind::A(0),
        CommandKind::C(0, 0, 0),
        CommandKind::A(0),
        CommandKind::C(0, 0, 0),
        CommandKind::A(0),
        CommandKind::C(0, 0, 0),
        CommandKind::A(0),
        CommandKind::C(0, 0, 0),
        CommandKind::A(0),
        CommandKind::C(0, 0, 0),
    ];

    let mut tokens = lex_all(&mut input).unwrap();
    println!("{:?}", tokens);
    let cmds = parse(&mut tokens).unwrap();
    for (i, cmd) in cmds.into_iter().enumerate() {
        let result = Annot::new(expect[i].clone(), Loc::new(0, 0, 0));
        assert!(
            eq(result.clone(), cmd.clone()),
            "result: {:?}, cmd: {:?}",
            result,
            cmd
        )
    }
}
