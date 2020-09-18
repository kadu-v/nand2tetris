//! code Writer for Hack VM
use crate::code_writer::commands::*;
use crate::code_writer::segments::*;
use std::io::{BufWriter, Write};

pub struct CodeWriter<'a, T: Write> {
    buf: &'a mut BufWriter<T>,
    label_counter: usize,
}

impl<'a, T: Write> CodeWriter<'a, T> {
    // コンストラクター
    // ファイルのbufferを受け取って，CodeWriter構造体を生成する
    pub fn new(buf: &mut BufWriter<T>) -> CodeWriter<T> {
        CodeWriter {
            buf,
            label_counter: 0,
        }
    }

    /// 算術コマンドと論理コマンドを受け取って，対応するアセンブリコードを生成する．
    pub fn write_arithmatic(&mut self, cmd: Commands) -> Result<(), std::io::Error> {
        match cmd {
            Commands::Add => self.write_add(),
            Commands::Sub => self.write_sub(),
            Commands::Neg => self.write_neg(),
            Commands::Eq => self.write_eq(),
            Commands::Gt => self.write_gt(),
            Commands::Lt => self.write_lt(),
            Commands::And => self.write_and(),
            Commands::Or => self.write_or(),
            Commands::Not => self.write_not(),
            _ => todo!("Error型をstd::io::Errorと統合して，unreachableなError型を作る"),
        }
    }

    /// push または pop　コマンドを受け取って，対応するアセンブリコードを生成する．
    pub fn write_push_pop(&mut self, cmd: Commands) -> Result<(), std::io::Error> {
        match cmd {
            Commands::Push(seg, idx) => self.write_push(seg, idx),
            Commands::Pop(seg, idx) => self.write_pop(seg, idx),
            _ => todo!("Error型をstd::io::Errorと統合して，unreachableなError型を作る"),
        }
    }

    /// Add に対応するアセンブラ を生成するメヘルパーメソッド
    fn write_add(&mut self) -> Result<(), std::io::Error> {
        // RAM[@SP] == nとする
        writeln!(self.buf, "//--add命令のアセンブラ ")?;
        writeln!(self.buf, "@SP   // SPをAにセット")?;
        writeln!(self.buf, "M=M-1 // RAM[SP]をデクリメント (RAM[SP] = n-1)")?;
        writeln!(self.buf, "A=M   // RAM[SP]をAにセット (RAM[SP] = n-1)")?;
        writeln!(self.buf, "D=M   // DレジスタにRAM「nー1]を退避")?;
        writeln!(self.buf, "A=A-1 // Aをデクリメント")?;
        writeln!(self.buf, "M=M+D // RAM[n-2] + RAM[n-1]")
    }

    /// Sub に対応するアセンブラ を生成するヘルパーメソッド
    fn write_sub(&mut self) -> Result<(), std::io::Error> {
        // RAM[@SP] == nとする
        writeln!(self.buf, "//--sub命令のアセンブラ ")?;
        writeln!(self.buf, "@SP   // SPをAにセット")?;
        writeln!(self.buf, "M=M-1 // RAM[SP]をデクリメント (RAM[SP] = n-1)")?;
        writeln!(self.buf, "A=M   // RAM[SP]をAにセット (RAM[SP] = n-1)")?;
        writeln!(self.buf, "D=M   // DレジスタにRAM「nー1]を退避")?;
        writeln!(self.buf, "A=A-1 // Aをデクリメント")?;
        writeln!(self.buf, "M=M-D // RAM[n-2] - RAM[n-1]")
    }

    /// Neg に対応するアセンブラ を生成するヘルパーメソッド
    fn write_neg(&mut self) -> Result<(), std::io::Error> {
        // RAM[@SP] == nとする
        writeln!(self.buf, "//--neg命令のアセンブラ ")?;
        writeln!(self.buf, "@SP   // SPをAにセット")?;
        writeln!(self.buf, "A=M-1 // Aレジスタにn-1をセット")?;
        writeln!(self.buf, "M=-M  // RAM[n-1]の符号を反転")
    }

    /// Eqに対応するアセンブラ を生成するヘルパーメソッド
    fn write_eq(&mut self) -> Result<(), std::io::Error> {
        // RAM[@SP] == nとする
        writeln!(self.buf, "//--eq命令のアセンブラ ")?;
        writeln!(self.buf, "@SP")?;
        writeln!(self.buf, "M=M-1")?;
        writeln!(self.buf, "D=M")?;
        writeln!(self.buf, "A=A-1")?;
        writeln!(self.buf, "D=M-D")?;
        writeln!(self.buf, "@TRUE{}", self.label_counter)?;
        writeln!(self.buf, "D;JEQ")?;
        writeln!(self.buf, "@SP")?;
        writeln!(self.buf, "A=M")?;
        writeln!(self.buf, "A=A-1")?;
        writeln!(self.buf, "M=0")?;
        writeln!(self.buf, "@NEXT{}", self.label_counter)?;
        writeln!(self.buf, "0;JMP")?;
        writeln!(self.buf, "(TRUE{})", self.label_counter)?;
        writeln!(self.buf, "@SP")?;
        writeln!(self.buf, "A=M")?;
        writeln!(self.buf, "A=A-1")?;
        writeln!(self.buf, "M=-1")?;
        let ret = writeln!(self.buf, "(NEXT{})", self.label_counter);
        self.label_counter += 1;
        ret
    }

    /// Gtに対応するアセンブラ を生成するヘルパーメソッド
    fn write_gt(&mut self) -> Result<(), std::io::Error> {
        // RAM[@SP] == nとする
        writeln!(self.buf, "//--gt命令のアセンブラ ")?;
        writeln!(self.buf, "@SP")?;
        writeln!(self.buf, "M=M-1")?;
        writeln!(self.buf, "D=M")?;
        writeln!(self.buf, "A=A-1")?;
        writeln!(self.buf, "D=M-D")?;
        writeln!(self.buf, "@TRUE{}", self.label_counter)?;
        writeln!(self.buf, "D;JGT")?;
        writeln!(self.buf, "@SP")?;
        writeln!(self.buf, "A=M")?;
        writeln!(self.buf, "A=A-1")?;
        writeln!(self.buf, "M=0")?;
        writeln!(self.buf, "@NEXT{}", self.label_counter)?;
        writeln!(self.buf, "0;JMP")?;
        writeln!(self.buf, "(TRUE{})", self.label_counter)?;
        writeln!(self.buf, "@SP")?;
        writeln!(self.buf, "A=M")?;
        writeln!(self.buf, "A=A-1")?;
        writeln!(self.buf, "M=-1")?;
        let ret = writeln!(self.buf, "(NEXT{})", self.label_counter);
        self.label_counter += 1;
        ret
    }

    /// Ltに対応するアセンブラ を生成するヘルパーメソッド
    fn write_lt(&mut self) -> Result<(), std::io::Error> {
        // RAM[@SP] == nとする
        writeln!(self.buf, "//--lt命令のアセンブラ ")?;
        writeln!(self.buf, "@SP")?;
        writeln!(self.buf, "M=M-1")?;
        writeln!(self.buf, "D=M")?;
        writeln!(self.buf, "A=A-1")?;
        writeln!(self.buf, "D=M-D")?;
        writeln!(self.buf, "@TRUE{}", self.label_counter)?;
        writeln!(self.buf, "D;JLT")?;
        writeln!(self.buf, "@SP")?;
        writeln!(self.buf, "A=M")?;
        writeln!(self.buf, "A=A-1")?;
        writeln!(self.buf, "M=0")?;
        writeln!(self.buf, "@NEXT{}", self.label_counter)?;
        writeln!(self.buf, "0;JMP")?;
        writeln!(self.buf, "(TRUE{})", self.label_counter)?;
        writeln!(self.buf, "@SP")?;
        writeln!(self.buf, "A=M")?;
        writeln!(self.buf, "A=A-1")?;
        writeln!(self.buf, "M=-1")?;
        let ret = writeln!(self.buf, "(NEXT{})", self.label_counter);
        self.label_counter += 1;
        ret
    }

    /// Andに対応するアセンブラ を生成するヘルパーメソッド
    fn write_and(&mut self) -> Result<(), std::io::Error> {
        // RAM[@SP] == nとする
        writeln!(self.buf, "//--and命令のアセンブラ ")?;
        writeln!(self.buf, "@SP   // SPをAにセット")?;
        writeln!(self.buf, "M=M-1 // RAM[SP]をデクリメント (RAM[SP] = n-1)")?;
        writeln!(self.buf, "A=M   // RAM[SP]をAにセット (RAM[SP] = n-1)")?;
        writeln!(self.buf, "D=M   // DレジスタにRAM「nー1]を退避")?;
        writeln!(self.buf, "A=A-1 // Aをデクリメント")?;
        writeln!(self.buf, "M=M&D // RAM[n-2] & RAM[n-1]")
    }

    /// Or に対応するアセンブラ を生成するヘルパーメソッド
    fn write_or(&mut self) -> Result<(), std::io::Error> {
        // RAM[@SP] == nとする
        writeln!(self.buf, "//--or命令のアセンブラ ")?;
        writeln!(self.buf, "@SP   // SPをAにセット")?;
        writeln!(self.buf, "M=M-1 // RAM[SP]をデクリメント (RAM[SP] = n-1)")?;
        writeln!(self.buf, "A=M   // RAM[SP]をAにセット (RAM[SP] = n-1)")?;
        writeln!(self.buf, "D=M   // DレジスタにRAM「nー1]を退避")?;
        writeln!(self.buf, "A=A-1 // Aをデクリメント")?;
        writeln!(self.buf, "M=M|D // RAM[n-2] | RAM[n-1]")
    }

    /// Not に対応するアセンブラ を生成するヘルパーメソッド
    fn write_not(&mut self) -> Result<(), std::io::Error> {
        // RAM[@SP] == nとする
        writeln!(self.buf, "//--not命令のアセンブラ ")?;
        writeln!(self.buf, "@SP   // SPをAにセット")?;
        writeln!(self.buf, "A=M-1 // Aレジスタにn-1をセット")?;
        writeln!(self.buf, "M=!M  // RAM[n-1]をビット反転")
    }

    /// Pushに対するアセンブラ を生成するヘルパーメソッド
    fn write_push(&mut self, seg: Segments, idx: u16) -> Result<(), std::io::Error> {
        writeln!(self.buf, "//--push {:?} {:?} 命令のアセンブラ", seg, idx)?;
        unimplemented!()
    }

    /// Popに対するアセンブラ を生成するヘルパーメソッド
    fn write_pop(&mut self, seg: Segments, idx: u16) -> Result<(), std::io::Error> {
        writeln!(self.buf, "//--pop {:?} {:?} 命令のアセンブラ", seg, idx)?;
        unimplemented!()
    }
}
