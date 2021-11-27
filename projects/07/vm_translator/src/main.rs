use std::ffi::OsStr;
use std::fs::*;
use std::path::Path;
use std::process;
use std::{env, io::BufWriter};
use vm_translator::{code_writer::CodeWriter, command::*, error::VmError, lexer::*, parser::*};

use std::io::{BufReader, Read, Write};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("please input file !");
        process::exit(1);
    }

    let input_file_path = Path::new(&args[1]);
    let output_file_path = match input_file_path.extension().and_then(OsStr::to_str) {
        Some("vm") => input_file_path.with_extension("as"),
        _ => {
            eprintln!("file extension is not *.vm !");
            process::exit(1);
        }
    };

    let input = match read_to_string(input_file_path) {
        Ok(src) => src,
        Err(e) => {
            eprintln!("{}", e.to_string());
            process::exit(1);
        }
    };
    let mut output_file = match File::create(output_file_path) {
        Ok(output_file) => BufWriter::new(output_file),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    println!("{:?}", &input);
    let mut l = Lexer::new(&input);
    let mut p = Parser::new(&mut l);
    let mut cdw = CodeWriter::new(&mut output_file);

    loop {
        let cmd = match p.parse_one() {
            Err(e) => {
                e.report_error(&input);
                process::exit(1)
            }
            Ok(c) => c,
        };

        println!("{:?}", cmd);
        if cmd == Command::EOTOK {
            break;
        }

        match cdw.write(cmd) {
            Ok(_) => continue,
            Err(e) => {
                eprintln!("{}", e.to_string());
                process::exit(1);
            }
        }
    }
}
