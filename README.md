# WASM audio nodes

Rust project targeting WebAssembly for audio processing. The wasm binaries should be wrapped in a node package, but there's no js wrapping code included at this point. It will be included in the future once I'm sure this approach can actually solve my performance issues.

## build

build command:

```bash
cargo build --release --target=wasm32-unknown-unknown && \
wasm-opt -Oz --strip-debug -o target/wasm32-unknown-unknown/release/wasm_audio_nodes_opt.wasm target/wasm32-unknown-unknown/release/wasm_audio_nodes.wasm
```
Inspect size with:

```bash
twiggy top -n 20 target/wasm32-unknown-unknown/release/wasm_audio_nodes_opt.wasm
```


## package

Coming soon.