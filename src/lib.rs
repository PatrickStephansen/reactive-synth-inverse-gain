// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const RENDER_QUANTUM: usize = 128;

fn clamp(min_value: f32, max_value: f32, value: f32) -> f32 {
    if value < min_value {
        return min_value;
    } else {
        if value > max_value {
            return max_value;
        } else {
            return value;
        }
    };
}

fn get_parameter(param: &Vec<f32>, min_value: f32, max_value: f32, index: usize) -> f32 {
    if param.len() > 1 {
        clamp(min_value, max_value, param[index])
    } else {
        if param.len() == 0 {
            clamp(min_value, max_value, 0.0)
        } else {
            clamp(min_value, max_value, param[0])
        }
    }
}

// #[link(wasm_import_module = "console")]
// extern "C" {
//     fn log(s: f32);
// }

pub struct InverseGain {
    quotient: Vec<f32>,
    divisor: Vec<f32>,
    zero_divisor_fallback: Vec<f32>,
    max_quotient: f32,
    min_quotient: f32,
    max_divisor: f32,
    min_divisor: f32,
    max_divisor_fallback: f32,
    min_divisor_fallback: f32,
    output: Vec<f32>,
}

impl InverseGain {
    pub fn new(
        min_quotient: f32,
        max_quotient: f32,
        min_divisor: f32,
        max_divisor: f32,
        min_divisor_fallback: f32,
        max_divisor_fallback: f32,
    ) -> InverseGain {
        let mut output = Vec::with_capacity(RENDER_QUANTUM);
        output.resize(RENDER_QUANTUM, 0.0);
        InverseGain {
            quotient: Vec::with_capacity(RENDER_QUANTUM),
            divisor: Vec::with_capacity(RENDER_QUANTUM),
            zero_divisor_fallback: Vec::with_capacity(RENDER_QUANTUM),
            output,
            min_quotient,
            max_quotient,
            min_divisor,
            max_divisor,
            min_divisor_fallback,
            max_divisor_fallback,
        }
    }

    pub fn process(&mut self) {
        for i in 0..RENDER_QUANTUM {
            if get_parameter(&self.divisor, self.min_divisor, self.max_divisor, i) == 0.0 {
                self.output[i] =
                    get_parameter(&self.quotient, self.min_quotient, self.max_quotient, i)
                        / get_parameter(
                            &self.zero_divisor_fallback,
                            self.min_divisor_fallback,
                            self.max_divisor_fallback,
                            i,
                        );
            }
            self.output[i] = get_parameter(&self.quotient, self.min_quotient, self.max_quotient, i)
                / get_parameter(&self.divisor, self.min_divisor, self.max_divisor, i);
        }
    }

    pub fn set_inputs(
        &mut self,
        quotient: Vec<f32>,
        divisor: Vec<f32>,
        zero_divisor_fallback: Vec<f32>,
    ) {
        self.quotient = quotient;
        self.divisor = divisor;
        self.zero_divisor_fallback = zero_divisor_fallback;
    }

    pub fn get_output(&self) -> *const f32 {
        self.output.as_ptr()
    }
}

#[no_mangle]
pub unsafe extern "C" fn init(
    min_quotient: f32,
    max_quotient: f32,
    min_divisor: f32,
    max_divisor: f32,
    min_divisor_fallback: f32,
    max_divisor_fallback: f32,
) -> *mut InverseGain {
    Box::into_raw(Box::new(InverseGain::new(
        min_quotient,
        max_quotient,
        min_divisor,
        max_divisor,
        min_divisor_fallback,
        max_divisor_fallback,
    )))
}

#[no_mangle]
pub unsafe extern "C" fn process_quantum(
    me: *mut InverseGain,
    quotient_len: usize,
    divistor_len: usize,
    fallback_len: usize,
) -> *mut f32 {
    // the expectation is that the parameters are copied directly into memory before this is called
    // so fix the length if it changed
    (*me).quotient.set_len(quotient_len);
    (*me).divisor.set_len(divistor_len);
    (*me).zero_divisor_fallback.set_len(fallback_len);
    (*me).process();
    (*me).output.as_mut_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn get_quotient_ptr(me: *mut InverseGain) -> *mut f32 {
    (*me).quotient.as_mut_ptr()
}
#[no_mangle]
pub unsafe extern "C" fn get_divisor_ptr(me: *mut InverseGain) -> *mut f32 {
    (*me).divisor.as_mut_ptr()
}
#[no_mangle]
pub unsafe extern "C" fn get_divisor_fallback_ptr(me: *mut InverseGain) -> *mut f32 {
    (*me).zero_divisor_fallback.as_mut_ptr()
}
// There's nothing in the wasm spec that allows memory to ever be freed
// There's also no hook for destruction of AudioWorkletProcessor
// 2 specs would have to change for this to be of any use
#[no_mangle]
pub unsafe extern "C" fn free(me: *mut InverseGain) {
    drop(Box::from_raw(me))
}
