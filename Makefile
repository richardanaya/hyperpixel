build:
	@cargo build --target wasm32-unknown-unknown
lint:
	@cargo fmt
serve:
	@python3 -m http.server 8080