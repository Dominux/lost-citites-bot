test_lib:
	cd game_lib && cargo test || true && cd -

clippy_lib:
	cd game_lib && cargo clippy && cd -

build_lib:
	cd game_lib && cargo build

build_lib_for_wasm:
	cd game_lib \
	&& wasm-pack build --target web -- --features wasm \
	&& rm -rf ../webapp/pkg || true \
	&& mv ./pkg ../webapp/ \
	&& cd -

run_dev:
	cd webapp && pnpm run dev || true && cd -
