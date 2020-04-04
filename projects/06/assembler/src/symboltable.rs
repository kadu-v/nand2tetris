//! The symbol table for Hack Assmbler

use crate::lexer::token::*;
use std::collections::HashMap;

/// シンボルテーブルを表す構造体
#[derive(Debug, PartialEq, Eq)]
pub struct SymbolTable {
    table: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            table: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, symbol: String, address: u16) {
        self.table.entry(symbol).or_insert(address);
    }

    pub fn get_address(&self, symbol: &String) -> Option<&u16> {
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
            match (line[0].value(), line[1].value()) {
                (TokenKind::LParen, TokenKind::Symbol(s)) => {
                    table.add_entry(s.clone(), address); //Stringをclone()してる
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
        @counter     
        "
    .as_bytes();
    let expect = [
        ("OUTPUT_FIRST", 0x000a),
        ("OUTPUT_D", 0x000c),
        ("INFINITE_LOOP", 0x000e),
    ];

    let tokens = lex_all(&mut input).unwrap();
    let table = SymbolTable::make(&tokens);
    for (key, value) in expect.into_iter() {
        let key = key.clone().to_string();
        assert_eq!(value, table.get_address(&key).unwrap());
    }
}
