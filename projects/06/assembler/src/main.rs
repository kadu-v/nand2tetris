use assembler::assembler::assembler::*;
use std::env;
use std::path::Path;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("nothing input file name");
  } else {
    let path = Path::new(&args[1]);
    match assembler(path) {
      Ok(_) => println!("Sucess!, finish hack binary!"),
      Err(e) => eprintln!("{:?}", e),
    }
  }
}
