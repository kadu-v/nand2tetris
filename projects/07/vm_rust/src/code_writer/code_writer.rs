//! code Writer for Hack VM
use crate::code_writer::commands::*;
use crate::code_writer::segments::*;

/// 算術コマンドと論理コマンドを受け取って，対応するアセンブリコードを生成する．
pub fn write_arithmatic(cmd: Commands) -> String {
    match cmd {
        Commands::Add => write_add(),
        Commands::Sub => write_sub(),
        Commands::Neg => write_neg(),
        Commands::Eq => write_eq(),
        Commands::Gt => write_gt(),
        Commands::Lt => write_lt(),
        Commands::And => write_and(),
        Commands::Or => write_or(),
        Commands::Not => write_not(),
        _ => "".to_string(),
    }
}

/// push または pop　コマンドを受け取って，対応するアセンブリコードを生成する．
pub fn write_push_pop(cmd: Commands) -> String {
    match cmd {
        Commands::Push(seg, idx) => write_push(seg, idx),
        Commands::Pop(seg, idx) => write_pop(seg, idx),
        _ => "".into(),
    }
}

/// Add に対応するアセンブラ を生成するメヘルパーメソッド
fn write_add() -> String {
    unimplemented!()
}

/// Sub に対応するアセンブラ を生成するヘルパーメソッド
fn write_sub() -> String {
    unimplemented!()
}

/// Neg に対応するアセンブラ を生成するヘルパーメソッド
fn write_neg() -> String {
    unimplemented!()
}

/// Eqに対応するアセンブラ を生成するヘルパーメソッド
fn write_eq() -> String {
    unimplemented!()
}

/// Gtに対応するアセンブラ を生成するヘルパーメソッド
fn write_gt() -> String {
    unimplemented!()
}

/// Ltに対応するアセンブラ を生成するヘルパーメソッド
fn write_lt() -> String {
    unimplemented!()
}

/// Andに対応するアセンブラ を生成するヘルパーメソッド
fn write_and() -> String {
    unimplemented!()
}

/// Or に対応するアセンブラ を生成するヘルパーメソッド
fn write_or() -> String {
    unimplemented!()
}

/// Not に対応するアセンブラ を生成するヘルパーメソッド
fn write_not() -> String {
    unimplemented!()
}

/// Pushに対するアセンブラ を生成するヘルパーメソッド
fn write_push(seg: Segments, idx: u16) -> String {
    unimplemented!()
}

/// Popに対するアセンブラ を生成するヘルパーメソッド
fn write_pop(seg: Segments, idx: u16) -> String {
    unimplemented!()
}
