# reactive-synth-inverse-gain

WASM implementation of an inverse gain audio processing node compatible with the web audio API

## build

build command:

```bash
cargo build --features wee_alloc --release --target=wasm32-unknown-unknown && \
wasm-opt -Oz --strip-debug -o target/wasm32-unknown-unknown/release/reactive_synth_inverse_gain_opt.wasm \
target/wasm32-unknown-unknown/release/reactive_synth_inverse_gain.wasm
```
Inspect size with:

```bash
twiggy top -n 20 target/wasm32-unknown-unknown/release/reactive_synth_inverse_gain_opt.wasm
```

## package

from worklet directory:

```bash
npm run package
```