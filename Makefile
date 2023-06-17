test_lib:
	cd game_lib && cargo test || true && cd -

clippy_lib:
	cd game_lib && cargo clippy && cd -
