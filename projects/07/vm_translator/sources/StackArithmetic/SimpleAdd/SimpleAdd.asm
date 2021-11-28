//--push CONST 7 命令のアセンブラ
    @7  // 定数7をAレジスタにセット
    D=A  // 定数7をDレジスタにセット
    @SP   // SPをAレジスタにセット
    A=M   // nをAレジスタにセット
    M=D   // M(RAM[n])にDレジスタの値をセット
    @SP   // SPをAレジスタにセット
    M=M+1 // RAM[@SP]をインクリメント
//--push CONST 8 命令のアセンブラ
    @8  // 定数8をAレジスタにセット
    D=A  // 定数8をDレジスタにセット
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
