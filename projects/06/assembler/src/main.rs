use assembler::lexer::lexer::lex_all;

fn main() {
    let mut input = "@R0
        D=M              // D = first number
        @R1
        D=D-M            // D = first number - second number
        @OUTPUT_FIRST
        D;JGT            // if D>0 (first is greater) goto output_first
        @R1
        D=M              // D = second number
        @OUTPUT_D
        0;JMP            // goto output_d
      (OUTPUT_FIRST)
        @R0             
        D=M              // D = first number
      (OUTPUT_D)
        @R2
         M=D              // M[2] = D (greatest number)
      (INFINITE_LOOP)
        @INFINITE_LOOP
        0;JMP            // infinite loop
        "
    .as_bytes();

    println!("In the case of input that succeeds in lexical analysis:\n");
    match lex_all(&mut input) {
        Err(lexerr) => println!("{:?}", lexerr),
        Ok(lines) => {
            for line in lines {
                println!("{:?}", line);
            }
        }
    }

    input = "@R0
        D=M              // D = first number
        @R1
        D=D-M            // D = first number - second number
        @OUTPUT_FIRST
        D;JGT            / if D>0 (first is greater) goto output_first
        @R1
        D=M              // D = second number
        @OUTPUT_D
        0;JMP            // goto output_d
      (OUTPUT_FIRST)
        @R0             
        D=M              // D = first number
      (OUTPUT_D)
        @R2
         M=D              // M[2] = D (greatest number)
      (INFINITE_LOOP)
        @INFINITE_LOOP
        0;JMP            // infinite loop
        "
    .as_bytes();

    println!("\n\n\nIn case of input that fails lexical analysis:\n");
    match lex_all(&mut input) {
        Err(lexerr) => println!("{:?}", lexerr),
        Ok(lines) => {
            for line in lines {
                println!("{:?}", line);
            }
        }
    };
}
