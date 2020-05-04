//! code Writer for Hack VM
use crate::code_writer::code_error::*;
use crate::code_writer::commands::*;
use crate::code_writer::segments::*;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

pub struct CodeWriter<'a> {
    buf: &'a mut BufReader<std::fs::File>,
}

impl<'a> CodeWriter<'a> {
    // コンストラクター
    // ファイルのbufferを受け取って，CodeWriter構造体を生成する
    pub fn new(buf: &mut BufReader<std::fs::File>) -> CodeWriter {
        CodeWriter { buf: buf }
    }

    /// 算術コマンドと論理コマンドを受け取って，対応するアセンブリコードを生成する．
    pub fn write_arithmatic(&mut self, cmd: Commands) -> Result<(), CodeError> {
        match cmd {
            Commands::Add => CodeWriter::write_add(self),
            Commands::Sub => CodeWriter::write_sub(),
            Commands::Neg => CodeWriter::write_neg(),
            Commands::Eq => CodeWriter::write_eq(),
            Commands::Gt => CodeWriter::write_gt(),
            Commands::Lt => CodeWriter::write_lt(),
            Commands::And => CodeWriter::write_and(),
            Commands::Or => CodeWriter::write_or(),
            Commands::Not => CodeWriter::write_not(),
            _ => Err(CodeError::invalid_command(cmd)),
        }
    }

    /// push または pop　コマンドを受け取って，対応するアセンブリコードを生成する．
    pub fn write_push_pop(cmd: Commands) -> Result<(), CodeError> {
        match cmd {
            Commands::Push(seg, idx) => CodeWriter::write_push(seg, idx),
            Commands::Pop(seg, idx) => CodeWriter::write_pop(seg, idx),
            _ => Err(CodeError::invalid_command(cmd)),
        }
    }

    /// Add に対応するアセンブラ を生成するメヘルパーメソッド
    fn write_add(&mut self) -> Result<(), CodeError> {
        unimplemented!()
    }

    /// Sub に対応するアセンブラ を生成するヘルパーメソッド
    fn write_sub() -> Result<(), CodeError> {
        unimplemented!()
    }

    /// Neg に対応するアセンブラ を生成するヘルパーメソッド
    fn write_neg() -> Result<(), CodeError> {
        unimplemented!()
    }

    /// Eqに対応するアセンブラ を生成するヘルパーメソッド
    fn write_eq() -> Result<(), CodeError> {
        unimplemented!()
    }

    /// Gtに対応するアセンブラ を生成するヘルパーメソッド
    fn write_gt() -> Result<(), CodeError> {
        unimplemented!()
    }

    /// Ltに対応するアセンブラ を生成するヘルパーメソッド
    fn write_lt() -> Result<(), CodeError> {
        unimplemented!()
    }

    /// Andに対応するアセンブラ を生成するヘルパーメソッド
    fn write_and() -> Result<(), CodeError> {
        unimplemented!()
    }

    /// Or に対応するアセンブラ を生成するヘルパーメソッド
    fn write_or() -> Result<(), CodeError> {
        unimplemented!()
    }

    /// Not に対応するアセンブラ を生成するヘルパーメソッド
    fn write_not() -> Result<(), CodeError> {
        unimplemented!()
    }

    /// Pushに対するアセンブラ を生成するヘルパーメソッド
    fn write_push(seg: Segments, idx: u16) -> Result<(), CodeError> {
        unimplemented!()
    }

    /// Popに対するアセンブラ を生成するヘルパーメソッド
    fn write_pop(seg: Segments, idx: u16) -> Result<(), CodeError> {
        unimplemented!()
    }
}
