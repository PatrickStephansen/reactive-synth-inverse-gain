# reactive-synth-inverse-gain

WASM implementation of an inverse gain audio processing node compatible with the web audio API

## build

build command:

```bash
cargo build --features wee_alloc --release --target=wasm32-unknown-unknown && \
wasm-opt -Oz --strip-debug -o worklet/reactive_synth_inverse_gain.wasm \
target/wasm32-unknown-unknown/release/reactive_synth_inverse_gain.wasm
```
Inspect size with:

```bash
twiggy top -n 20 worklet/reactive_synth_inverse_gain.wasm
```
