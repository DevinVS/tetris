tetris: $(wildcard src/*.rs)
	cargo build --target=x86_64-apple-darwin --release
	cargo build --target=x86_64-pc-windows-gnu --release
	cargo build --target=x86_64-unknown-linux-gnu --release
