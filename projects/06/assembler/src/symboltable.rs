//! The symbol table for Hack Assmbler

use crate::lexer::token::*;
use std::collections::HashMap;

/// シンボルテーブルを表す構造体
#[derive(Debug, PartialEq, Eq)]
pub struct SymbolTable<'a> {
    table: HashMap<&'a str, u16>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> SymbolTable<'a> {
        SymbolTable {
            table: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, symbol: &'a str, address: u16) {
        self.table.entry(symbol).or_insert(address);
    }

    pub fn get_address(&self, symbol: &'a str) -> Option<&u16> {
        self.table.get(symbol)
    }

    pub fn make(tokens: &Vec<Vec<Token>>) -> SymbolTable {
        let mut table = SymbolTable::new();
        let mut address = 0;
        for line in tokens {
            if line.len() < 2 {
                address += 1;
                continue;
            }
            match (line[0].get_value(), line[1].get_value()) {
                (TokenKind::LParen, TokenKind::Symbol(s)) => {
                    table.add_entry(s, address);
                }
                _ => {
                    address += 1;
                }
            }
        }
        table
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//
//
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_new() {
    assert_eq!(
        SymbolTable {
            table: HashMap::new(),
        },
        SymbolTable::new()
    );
}

#[test]
fn test_symbol_table() {
    use crate::lexer::lexer::*;
    let mut input = "@R0
        D=M              
        @R1
        D=D-M            
        @OUTPUT_FIRST
        D;JGT            
        @R1
        D=M              
        @OUTPUT_D
        0;JMP           
      (OUTPUT_FIRST)
        @R0             
        D=M            
      (OUTPUT_D)
        @R2
         M=D            
      (INFINITE_LOOP)
        @INFINITE_LOOP
        0;JMP           
        "
    .as_bytes();
    let expect = [
        ("OUTPUT_FIRST", 0x000a),
        ("OUTPUT_D", 0x000c),
        ("INFINITE_LOOP", 0x000e),
    ];

    let tokens = lex_all(&mut input).unwrap();
    let table = SymbolTable::make(&tokens);
    for (key, value) in expect.iter() {
        assert_eq!(value, table.get_address(key).unwrap());
    }
}
