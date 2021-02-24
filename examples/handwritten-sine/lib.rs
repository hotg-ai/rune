#![no_std]
#![feature(alloc_error_handler)]
#![allow(warnings)]

extern crate alloc;

use alloc::boxed::Box;
use runic_types::{
    debug,
    wasm32::{intrinsics, Model, Random, Serial},
    PipelineContext, Source, Transform,
};

static mut PIPELINE: Option<Box<FnMut()>> = None;

#[no_mangle]
pub extern "C" fn _manifest() -> u32 {
    unsafe {
        let mut rand: Random<i32, 1> = Random::new();

        let blob = include_bytes!("sine.tflite");
        let mut sine_model: Model<[i32; 1], [f32; 1]> = Model::load(blob);
        let mut serial = Serial::new();

        // We need a way to store the pipeline so it can be used by the call.
        // For now I'll just wrap it in a closure and store it as a global
        // variable, but ideally we'd pass ownership of the pipeline to the VM
        // and be given a pointer to it in _call().
        PIPELINE = Some(Box::new(move || {
            let mut ctx = PipelineContext::default();

            let random_bytes = rand.generate(&mut ctx);
            let sine_value = sine_model.transform(random_bytes, &mut ctx);
            serial.consume(sine_value, &mut ctx);

            debug!("Sine of {:?} is {:?}", random_bytes, sine_value);
        }));
    }

    1
}

#[no_mangle]
pub extern "C" fn _call(
    capability_type: i32,
    input_type: i32,
    capability_idx: i32,
) -> i32 {
    unsafe {
        let pipeline = PIPELINE
            .as_mut()
            .expect("You need to initialize the Rune before calling it");
        pipeline();

        0
    }
}
