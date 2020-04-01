//! The symbol table for Hack Assmbler

use crate::lexer::token::*;
use std::collections::HashMap;

/// シンボルテーブルを表す構造体
#[derive(Debug, PartialEq, Eq)]
pub struct SymbolTable<'a> {
    table: HashMap<&'a TokenKind, u16>,
    address: u16,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> SymbolTable<'a> {
        SymbolTable {
            table: HashMap::new(),
            address: 0, // 次の命令のアドレス
        }
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
            address: 0
        },
        SymbolTable::new()
    );
}

#[test]
fn test_symbol_table() {
    let input = "@R0
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
}
