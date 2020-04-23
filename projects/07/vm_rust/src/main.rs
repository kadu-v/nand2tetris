use vm_rust::lexer::lexer::*;

fn main() {
    println!("Hello, world!");

    let inputs = ["add", "add x y", "push", "pop"];

    for (i, input) in inputs.iter().enumerate() {
        let tokens = lexer(input, i).ok().unwrap();
        println!("{:?}", tokens);
    }
}
