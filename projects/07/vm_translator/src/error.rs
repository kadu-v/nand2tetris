use crate::loc::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmError {
    loc: Loc,
    msg: String,
}

impl VmError {
    pub fn new(loc: Loc, msg: String) -> Self {
        Self { loc, msg }
    }

    pub fn report_error(self, code: &str) {
        let Loc { line, left, width } = self.loc;
        let mut start = left;
        let mut end = left;

        while start > 0 && &code[start - 1..start] != "\n" {
            start -= 1;
        }

        while end < code.len() && &code[end..end + 1] != "\n" {
            end += 1;
        }

        eprintln!("line: {}", line);
        eprintln!("   {}", &code[start..end]);
        self.eprint_whitespaces(left - start + 3);
        self.eprintln_hat(width);
        self.eprint_whitespaces(left - start + 3);
        eprintln!("{}", self.msg);
    }

    fn eprint_whitespaces(&self, n: usize) {
        for _ in 0..n {
            print!(" ");
        }
    }

    fn eprintln_hat(&self, n: usize) {
        for _ in 0..n {
            print!("^");
        }
        println!();
    }
}
