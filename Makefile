TARGET=

ifeq ($(OS),Windows_NT)
	TARGET=x86_64-pc-windows-gnu
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		TARGET=x86_64-unknown-linux-gnu
	endif
	ifeq ($(UNAME_S),Darwin)
		TARGET=x86_64-pc-windows-gnu
	endif
endif


tetris: $(wildcard src/*.rs)
	cargo build --target=$(TARGET) --release
