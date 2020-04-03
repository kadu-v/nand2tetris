use crate::assembler::error::*;
use crate::lexer::lexer::lex_all;
use crate::parser::command::*;
use crate::parser::parser::parse;

use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

use std::io::{BufReader, Write};

pub fn assembler(file: &Path) -> Result<(), Error> {
    // エラー処理
    let to_file = match file.extension().and_then(OsStr::to_str) {
        Some("asm") => file.with_extension("hack"),
        _ => return Err(Error::File("extension is not .asm".to_string())),
    };
    let f = match File::open(file) {
        Ok(f) => f,
        Err(e) => return Err(Error::File(e.to_string())),
    };
    let mut buf = BufReader::new(f);
    let tokens = lex_all(&mut buf)?;
    let commands = parse(&tokens)?;
    let mut to_f = match File::create(to_file) {
        Ok(to_f) => to_f,
        Err(e) => return Err(Error::File(e.to_string())),
    };

    for cmd in commands {
        match cmd.value() {
            CommandKind::A(val) => {
                if let Err(e) = writeln!(to_f, "0{:015b}", val) {
                    return Err(Error::File(e.to_string()));
                }
            }
            CommandKind::C(comp, dest, jump) => {
                if let Err(e) = writeln!(to_f, "111{:07b}{:03b}{:03b}", comp, dest, jump) {
                    return Err(Error::File(e.to_string()));
                }
            }
        }
    }
    Ok(())
}
