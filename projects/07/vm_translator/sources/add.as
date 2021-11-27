//--add命令のアセンブラ 
@SP   // SPをAにセット
M=M-1 // RAM[SP]をデクリメント (RAM[SP] = n-1)
A=M   // RAM[SP]をAにセット (RAM[SP] = n-1)
D=M   // DレジスタにRAM[n-1]を退避
A=A-1 // Aをデクリメント
M=M+D // RAM[n-2] + RAM[n-1]
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
//--push CONST 0 命令のアセンブラ
@0  // 定数0をAレジスタにセット
D=A  // 定数0をDレジスタにセット
@SP   // SPをAレジスタにセット
A=M   // nをAレジスタにセット
M=D   // M(RAM[n])にDレジスタの値をセット
@SP   // SPをAレジスタにセット
M=M+1 // RAM[@SP]をインクリメント
