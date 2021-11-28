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
//--gt命令のアセンブラ 
    @SP
    AM=M-1
    D=M
    @SP
    A=M-1
    D=M-D
    @TRUE6
    D;JGT
    @SP
    A=M
    A=A-1
    M=0
    @NEXT6
    0;JMP
(TRUE6)
    @SP
    A=M
    A=A-1
    M=-1
(NEXT6)
//--push CONST 32766 命令のアセンブラ
    @32766  // 定数32766をAレジスタにセット
    D=A  // 定数32766をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--push CONST 32767 命令のアセンブラ
    @32767  // 定数32767をAレジスタにセット
    D=A  // 定数32767をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--gt命令のアセンブラ 
    @SP
    AM=M-1
    D=M
    @SP
    A=M-1
    D=M-D
    @TRUE7
    D;JGT
    @SP
    A=M
    A=A-1
    M=0
    @NEXT7
    0;JMP
(TRUE7)
    @SP
    A=M
    A=A-1
    M=-1
(NEXT7)
//--push CONST 32766 命令のアセンブラ
    @32766  // 定数32766をAレジスタにセット
    D=A  // 定数32766をDレジスタにセット
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
//--gt命令のアセンブラ 
    @SP
    AM=M-1
    D=M
    @SP
    A=M-1
    D=M-D
    @TRUE8
    D;JGT
    @SP
    A=M
    A=A-1
    M=0
    @NEXT8
    0;JMP
(TRUE8)
    @SP
    A=M
    A=A-1
    M=-1
(NEXT8)
//--push CONST 57 命令のアセンブラ
    @57  // 定数57をAレジスタにセット
    D=A  // 定数57をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--push CONST 31 命令のアセンブラ
    @31  // 定数31をAレジスタにセット
    D=A  // 定数31をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--push CONST 53 命令のアセンブラ
    @53  // 定数53をAレジスタにセット
    D=A  // 定数53をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--add命令のアセンブラ 
    @SP   // SPをAにセット
    M=M-1 // RAM[SP]をデクリメント (RAM[SP] = n-1)
    A=M   // RAM[SP]をAにセット (RAM[SP] = n-1)
    D=M   // DレジスタにRAM[n-1]を退避
    A=A-1 // Aをデクリメント
    M=M+D // RAM[n-2] + RAM[n-1]
//--push CONST 112 命令のアセンブラ
    @112  // 定数112をAレジスタにセット
    D=A  // 定数112をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--sub命令のアセンブラ 
    @SP   // SPをAにセット
    M=M-1 // RAM[SP]をデクリメント (RAM[SP] = n-1)
    A=M   // RAM[SP]をAにセット (RAM[SP] = n-1)
    D=M   // DレジスタにRAM「nー1]を退避
    A=A-1 // Aをデクリメント
    M=M-D // RAM[n-2] - RAM[n-1]
//--neg命令のアセンブラ 
    @SP   // SPをAにセット
    A=M-1 // Aレジスタにn-1をセット
    M=-M  // RAM[n-1]の符号を反転
//--and命令のアセンブラ 
    @SP   // SPをAにセット
    M=M-1 // RAM[SP]をデクリメント (RAM[SP] = n-1)
    A=M   // RAM[SP]をAにセット (RAM[SP] = n-1)
    D=M   // DレジスタにRAM「nー1]を退避
    A=A-1 // Aをデクリメント
    M=M&D // RAM[n-2] & RAM[n-1]
//--push CONST 82 命令のアセンブラ
    @82  // 定数82をAレジスタにセット
    D=A  // 定数82をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--or命令のアセンブラ 
    @SP   // SPをAにセット
    M=M-1 // RAM[SP]をデクリメント (RAM[SP] = n-1)
    A=M   // RAM[SP]をAにセット (RAM[SP] = n-1)
    D=M   // DレジスタにRAM「nー1]を退避
    A=A-1 // Aをデクリメント
    M=M|D // RAM[n-2] | RAM[n-1]
//--not命令のアセンブラ 
    @SP   // SPをAにセット
    A=M-1 // Aレジスタにn-1をセット
    M=!M  // RAM[n-1]をビット反転
