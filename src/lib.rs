mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct InverseGain {
    input: Vec<f32>,
    output: Vec<f32>,
    divisor: Vec<f32>,
    zero_divisor_fallback: Vec<f32>,
}

#[wasm_bindgen]
impl InverseGain {
    pub fn new(
        input: Vec<f32>,
        output: Vec<f32>,
        divisor: Vec<f32>,
        zero_divisor_fallback: Vec<f32>,
    ) -> InverseGain {
        InverseGain {
            input,
            output,
            divisor,
            zero_divisor_fallback,
        }
    }

    pub fn process(&mut self) {
        let length = self.input.len();
        self.output = (0..length)
            .map(|i| {
                if self.divisor[i] == 0.0 {
                    return self.input[i] / self.zero_divisor_fallback[i];
                }
                return self.input[i] / self.divisor[i];
            })
            .collect();
    }

    pub fn get_output(&self)-> *const f32{
        self.output.as_ptr()
    }
}
