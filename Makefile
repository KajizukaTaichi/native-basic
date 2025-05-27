# Makefile

# ターゲットファイル名
ASM_FILE=output.asm
BIN_FILE=output.bin

# デフォルトターゲット
all: run

# Rustでアセンブリを出力
$(ASM_FILE):
	cargo run > $(ASM_FILE)

# NASMでバイナリに変換
$(BIN_FILE): $(ASM_FILE)
	nasm -f bin $(ASM_FILE) -o $(BIN_FILE)

# QEMUで実行
run: $(BIN_FILE)
	qemu-system-x86_64 -drive format=raw,file=$(BIN_FILE)

# 中間ファイルとバイナリを削除
clean:
	rm -f $(ASM_FILE) $(BIN_FILE)
