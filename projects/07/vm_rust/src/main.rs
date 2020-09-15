use std::io::{stdout, BufWriter};
use vm_rust::code_writer::code_writer::*;
use vm_rust::lexer::lexer::*;

fn main() {
    println!("Hello, world!");

    let inputs = ["add", "add x y", "push", "pop"];

    for tok in inputs
        .iter()
        .enumerate()
        .map(|(i, input)| lexer(input, i).ok().unwrap())
    {
        println!("{:?}", tok);
    }

    let mut buf = BufWriter::new(stdout());
    let mut cdw = CodeWriter::new(&mut buf);
    cdw.write_add();
}
