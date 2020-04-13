# WASM audio nodes

Rust project targeting WebAssembly for audio processing. The wasm binaries should be wrapped in a node package, but there's no js wrapping code included at this point. It will be included in the future once I'm sure this approach can actually solve my performance issues.

## build

`cargo build --release --target=wasm32-unknown-unknown`

## package

Coming soon.