use crate::command::Command;
use crate::command::Command::*;

use crate::command::Segments;
use crate::command::Segments::*;
use crate::error::VmError;
use crate::lexer::*;
use crate::token::TokenKind::*;
use crate::token::*;

#[derive(Debug)]
pub struct Parser<'a> {
    l: &'a mut Lexer<'a>,
    cur_tok: Token,
    next_tok: Token,
}

impl<'a> Parser<'a> {
    // Parserのコンストラクター
    pub fn new(l: &'a mut Lexer<'a>) -> Self {
        let cur_tok = l.next_token();
        let next_tok = l.next_token();
        Parser {
            l,
            cur_tok,
            next_tok,
        }
    }

    // 次のtokenをセットするメソッド
    fn next_token(&mut self) {
        self.cur_tok = self.next_tok.clone();
        self.next_tok = self.l.next_token();
    }

    pub fn parse_one(&mut self) -> Result<Command, VmError> {
        match &self.cur_tok.kind {
            Eof => {
                self.next_token();
                Ok(EOTOK)
            }
            NewLine => {
                self.next_token();
                self.parse_one()
            }
            Add | Sub | Neg | Eq | Lt | Gt | And | Or | Not => self.parse_arithmatic(),
            Push => self.parse_push(),
            Pop => self.parse_pop(),
            k => Err(VmError::new(
                self.cur_tok.loc.clone(),
                format!("illegal token: {:?}", k),
            )),
        }
    }

    fn read_newline(&mut self) -> Result<(), VmError> {
        match self.cur_tok.kind {
            NewLine => {
                self.next_token();
                Ok(())
            }
            _ => Err(VmError::new(
                self.cur_tok.loc.clone(),
                "expected newline".to_string(),
            )),
        }
    }

    fn read_segment(&mut self) -> Result<Segments, VmError> {
        let seg = match self.cur_tok.kind {
            Argument => ARG,
            Local => LCL,
            Static => STC,
            Constant => CONST,
            This => THIS,
            That => THAT,
            Pointer => PTR,
            Temp => TMP,
            _ => {
                return Err(VmError::new(
                    self.cur_tok.loc.clone(),
                    "expected segments".to_string(),
                ))
            }
        };
        self.next_token();
        Ok(seg)
    }

    fn read_number(&mut self) -> Result<usize, VmError> {
        let num = match self.cur_tok.kind {
            Number(n) => n,
            _ => {
                return Err(VmError::new(
                    self.cur_tok.loc.clone(),
                    "expected number!".to_string(),
                ))
            }
        };
        self.next_token();
        Ok(num)
    }

    fn parse_arithmatic(&mut self) -> Result<Command, VmError> {
        let cmd = match self.cur_tok.kind {
            Add => ADD,
            Sub => SUB,
            Neg => NEG,
            Eq => EQ,
            Gt => GT,
            Lt => LT,
            And => AND,
            Or => OR,
            Not => NOT,
            _ => {
                return Err(VmError::new(
                    self.cur_tok.loc.clone(),
                    "expected arithmatic command".to_string(),
                ))
            }
        };
        self.next_token();
        self.read_newline()?;
        Ok(cmd)
    }

    fn parse_push(&mut self) -> Result<Command, VmError> {
        self.next_token();
        let seg = self.read_segment()?;
        let num = self.read_number()?;
        self.read_newline()?;
        Ok(PUSH(seg, num))
    }

    fn parse_pop(&mut self) -> Result<Command, VmError> {
        self.next_token();
        let seg = self.read_segment()?;
        let num = self.read_number()?;
        self.read_newline()?;
        Ok(POP(seg, num))
    }
}
