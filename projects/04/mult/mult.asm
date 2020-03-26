// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// Put your code here.

    @R2 
    M=0 // 初期値の書き込み
    @R1
    D=M
    @i
    M=D // カウンタの設定
(Loop)
    @i
    D=M // iの初期値を設定
    @END
    D;JEQ // if D = 0 then jmp
    @i
    M=M-1
    @R0
    D=M
    @R2
    M=D+M
    @Loop
    0;JMP
(END)
    @END
    0;JMP