test_lib:
	cd game_lib && cargo test || true && cd -

clippy_lib:
	cd game_lib && cargo clippy && cd -

build_lib_for_wasm:
	cd game_lib && wasm-pack build --target web -- --features wasm
