all:
	cargo build --target wasm32-wasi --release

run:
	wasmedge target/wasm32-wasi/release/wasi-tcp-server.wasm
