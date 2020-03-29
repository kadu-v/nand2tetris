//! Token for Hack assembler

pub enum Token {
    // A命令のトークン
    At, // "@"

    Int(String),    // "1234"
    Symbol(String), // "SUM"

    // C命令のトークン
    // comp部分のトークン
    // レジスタ
    A, // "A"
    M, // "M"
    D, // "D"

    // 演算子
    Plus,  // "+"
    Minus, // "-"
    And,   // "&"
    Or,    // "|"
    Bang,  // "!"

    // dest部分のトークン
    Null,
    MD,
    AM,
    AD,
    AMD,

    // jump部分のトークン
    JGT,
    JEQ,
    JGE,
    JLT,
    JNE,
    JLE,
    JMP,

    Semi, // ";"

    Coment, // "//"

    LParen, // "("
    RParen, // ")"
}
