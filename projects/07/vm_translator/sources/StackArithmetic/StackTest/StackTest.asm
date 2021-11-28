//--push CONST 17 命令のアセンブラ
    @17  // 定数17をAレジスタにセット
    D=A  // 定数17をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--push CONST 17 命令のアセンブラ
    @17  // 定数17をAレジスタにセット
    D=A  // 定数17をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--eq命令のアセンブラ 
    @SP
    AM=M-1
    D=M
    @SP
    A=M-1
    D=M-D
    @TRUE0
    D;JEQ
    @SP
    A=M
    A=A-1
    M=0
    @NEXT0
    0;JMP
(TRUE0)
    @SP
    A=M
    A=A-1
    M=1
(NEXT0)
//--push CONST 17 命令のアセンブラ
    @17  // 定数17をAレジスタにセット
    D=A  // 定数17をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--push CONST 16 命令のアセンブラ
    @16  // 定数16をAレジスタにセット
    D=A  // 定数16をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--eq命令のアセンブラ 
    @SP
    AM=M-1
    D=M
    @SP
    A=M-1
    D=M-D
    @TRUE1
    D;JEQ
    @SP
    A=M
    A=A-1
    M=0
    @NEXT1
    0;JMP
(TRUE1)
    @SP
    A=M
    A=A-1
    M=1
(NEXT1)
//--push CONST 16 命令のアセンブラ
    @16  // 定数16をAレジスタにセット
    D=A  // 定数16をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--push CONST 17 命令のアセンブラ
    @17  // 定数17をAレジスタにセット
    D=A  // 定数17をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--eq命令のアセンブラ 
    @SP
    AM=M-1
    D=M
    @SP
    A=M-1
    D=M-D
    @TRUE2
    D;JEQ
    @SP
    A=M
    A=A-1
    M=0
    @NEXT2
    0;JMP
(TRUE2)
    @SP
    A=M
    A=A-1
    M=1
(NEXT2)
//--push CONST 892 命令のアセンブラ
    @892  // 定数892をAレジスタにセット
    D=A  // 定数892をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--push CONST 891 命令のアセンブラ
    @891  // 定数891をAレジスタにセット
    D=A  // 定数891をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--lt命令のアセンブラ 
    @SP
    AM=M-1
    D=M
    @SP
    A=M-1
    D=M-D
    @TRUE3
    D;JLT
    @SP
    A=M
    A=A-1
    M=0
    @NEXT3
    0;JMP
(TRUE3)
    @SP
    A=M
    A=A-1
    M=-1
(NEXT3)
//--push CONST 891 命令のアセンブラ
    @891  // 定数891をAレジスタにセット
    D=A  // 定数891をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--push CONST 892 命令のアセンブラ
    @892  // 定数892をAレジスタにセット
    D=A  // 定数892をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--lt命令のアセンブラ 
    @SP
    AM=M-1
    D=M
    @SP
    A=M-1
    D=M-D
    @TRUE4
    D;JLT
    @SP
    A=M
    A=A-1
    M=0
    @NEXT4
    0;JMP
(TRUE4)
    @SP
    A=M
    A=A-1
    M=-1
(NEXT4)
//--push CONST 891 命令のアセンブラ
    @891  // 定数891をAレジスタにセット
    D=A  // 定数891をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--push CONST 891 命令のアセンブラ
    @891  // 定数891をAレジスタにセット
    D=A  // 定数891をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--lt命令のアセンブラ 
    @SP
    AM=M-1
    D=M
    @SP
    A=M-1
    D=M-D
    @TRUE5
    D;JLT
    @SP
    A=M
    A=A-1
    M=0
    @NEXT5
    0;JMP
(TRUE5)
    @SP
    A=M
    A=A-1
    M=-1
(NEXT5)
//--push CONST 32767 命令のアセンブラ
    @32767  // 定数32767をAレジスタにセット
    D=A  // 定数32767をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--push CONST 32766 命令のアセンブラ
    @32766  // 定数32766をAレジスタにセット
    D=A  // 定数32766をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
