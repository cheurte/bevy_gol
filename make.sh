cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --out-name wasm_example \
	--out-dir examples/wasm/target \
	--target web target/wasm32-unknown-unknown/release/bevy_test.wasm
