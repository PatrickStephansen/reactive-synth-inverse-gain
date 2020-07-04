# reactive-synth-inverse-gain

WASM implementation of an inverse gain audio processing node compatible with the web audio API. Created for [reactive-synth](https://github.com/PatrickStephansen/reactive-synth), but usable elsewhere if I ever document how.

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

Run `npm link` from the worklet directory before trying to build the reactive-synth app (the dependent app not in this repo)

## usage example

Test harness page coming soon. For now see the real use in [reactive-synth](https://github.com/PatrickStephansen/reactive-synth).
