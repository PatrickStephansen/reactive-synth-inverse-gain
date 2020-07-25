const defaultMin = -1e9;
const defaultMax = 1e9;
const bytesPerSample = 4;

registerProcessor(
  "reactive-synth-inverse-gain",
  class InverseGain extends AudioWorkletProcessor {
    static get parameterDescriptors() {
      return [
        {
          name: "divisor",
          defaultValue: 10,
          automationRate: "a-rate",
        },
        {
          name: "zeroDivisorFallback",
          defaultValue: 0,
          automationRate: "a-rate",
        },
      ];
    }

    constructor() {
      super();
      this.defaultInput = [0];
      this.port.onmessage = (event) => {
        if (event.data.type === "wasm") {
          this.initWasmModule(event.data.wasmModule).then(() =>
            this.port.postMessage({ type: "module-ready", value: true })
          );
        }
      };
    }

    async initWasmModule(wasmModule) {
      this.wasmModule = await WebAssembly.instantiate(wasmModule, {});
      this.internalProcessorPtr = this.wasmModule.exports.init(
        defaultMin,
        defaultMax,
        defaultMin,
        defaultMax,
        defaultMin,
        defaultMax
      );
      this.float32WasmMemory = new Float32Array(
        this.wasmModule.exports.memory.buffer
      );
    }

    process(inputs, outputs, parameters) {
      if (this.wasmModule) {
        this.float32WasmMemory.set(
          (inputs && inputs[0] && inputs[0][0]) || this.defaultInput,
          this.wasmModule.exports.get_quotient_ptr(this.internalProcessorPtr) /
            bytesPerSample
        );
        this.float32WasmMemory.set(
          parameters.divisor,
          this.wasmModule.exports.get_divisor_ptr(this.internalProcessorPtr) /
            bytesPerSample
        );
        this.float32WasmMemory.set(
          parameters.zeroDivisorFallback,
          this.wasmModule.exports.get_divisor_fallback_ptr(
            this.internalProcessorPtr
          ) / bytesPerSample
        );
        const outputPointer =
          this.wasmModule.exports.process_quantum(
            this.internalProcessorPtr,
            ((inputs && inputs[0] && inputs[0][0]) || this.defaultInput).length,
            parameters.divisor.length,
            parameters.zeroDivisorFallback.length
          ) / bytesPerSample;
        for (
          let channelIndex = 0;
          channelIndex < outputs[0].length;
          channelIndex++
        ) {
          // TODO: can this not be done with some array util that's faster?
          for (
            let sample = 0;
            sample < outputs[0][channelIndex].length;
            sample++
          ) {
            outputs[0][channelIndex][sample] = this.float32WasmMemory[
              outputPointer + sample
            ];
          }
        }
      }
      return true;
    }
  }
);
