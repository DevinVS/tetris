SRC=$(wildcard src/*.rs)
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

tetris: $(SRC)
	cargo build --target=$(TARGET) --release
	mkdir -p releases/$(TARGET)
	zip -j releases/$(TARGET)/tetris.zip ./target/$(TARGET)/release/*

.PHONY: all
all:
	cargo build --target=x86_64-unknown-linux-gnu --release
	cargo build --target=x86_64-pc-windows-gnu --release
	mkdir -p releases/x86_64-unknown-linux-gnu
	mkdir -p releases/x86_64-pc-windows-gnu
	zip -j releases/x86_64-unknown-linux-gnu/tetris.zip ./target/x86_64-unknown-linux-gnu/release/*
	zip -j releases/x86_64-pc-windows-gnu/tetris.zip ./target/x86_64-pc-windows-gnu/release/*
