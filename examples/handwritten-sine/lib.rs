#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::boxed::Box;
use modulo::Modulo;
use runic_types::{
    debug,
    wasm32::{Model, Random, Serial},
    Sink, Source, Transform,
};

static mut PIPELINE: Option<Box<dyn FnMut()>> = None;

#[no_mangle]
pub extern "C" fn _manifest() -> u32 {
    unsafe {
        debug!("Initializing");

        let mut rand: Random<f32, 1> = Random::new();

        let mut mod360 = Modulo::default().with_modulus(360.0);

        let mut sine_model: Model<[f32; 1], [f32; 1]> =
            Model::load(include_bytes!("sine.tflite"));

        let mut serial = Serial::new();

        // We need a way to store the pipeline so it can be used by the call.
        // For now I'll just wrap it in a closure and store it as a global
        // variable, but ideally we'd pass ownership of the pipeline to the VM
        // and be given a pointer to it in _call().
        PIPELINE = Some(Box::new(move || {
            let input = rand.generate();
            let input = mod360.transform(input);
            let input = sine_model.transform(input);
            serial.consume(input);
        }));
    }

    1
}

fn type_name<T: 'static>(_: &T) -> &'static str { core::any::type_name::<T>() }

#[no_mangle]
pub extern "C" fn _call(
    _capability_type: i32,
    _input_type: i32,
    _capability_idx: i32,
) -> i32 {
    unsafe {
        // load the pipeline, blowing up if it hasn't been initialized
        let pipeline = PIPELINE
            .as_mut()
            .expect("You need to initialize the Rune before calling it");

        // then run it
        pipeline();

        0
    }
}
