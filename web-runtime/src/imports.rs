use js_sys::Function;
use runicos_base::BaseImage;
use wasm_bindgen::{JsCast, prelude::*};
use anyhow::Error;
use crate::{
    external_types::{CapabilityConstructor, OutputConstructor},
    rust_error,
};

/// A table of host functions that will be invoked by the Rune at runtime.
#[wasm_bindgen]
#[derive(Default)]
pub struct Imports {
    capabilities: CapabilityConstructors,
    outputs: OutputConstructors,
}

impl Imports {
    pub(crate) fn to_image(self) -> BaseImage {
        let mut image = BaseImage::default();

        let Imports {
            capabilities: CapabilityConstructors { rand },
            outputs: OutputConstructors { serial },
        } = self;

        if let Some(rand) = rand {
            image.with_rand(cap(rand));
        }
        if let Some(serial) = serial {
            image.with_serial(out(serial));
        }


        image
    }
}

#[wasm_bindgen]
impl Imports {
    pub fn new() -> Imports { Imports::default() }

    #[wasm_bindgen(setter)]
    pub fn set_rand(&mut self, constructor: CapabilityConstructor) {
        self.capabilities.rand = Some(constructor);
    }
}

#[derive(Default)]
pub(crate) struct CapabilityConstructors {
    rand: Option<CapabilityConstructor>,
}

#[derive(Default)]
pub(crate) struct OutputConstructors {
    serial: Option<OutputConstructor>,
}

fn cap(
    constructor: CapabilityConstructor,
) -> impl Fn() -> Result<Box<dyn rune_runtime::Capability>, Error>
       + Send
       + Sync
       + 'static {
    let constructor = AssertSyncBecauseWasmIsSingleThreaded(constructor);

    move || {
        let function: &Function = constructor.0.as_ref();
        let capability = function.call0(&JsValue::NULL).map_err(rust_error)?;

        Ok(Box::new(
            crate::external_types::Capability::unchecked_from_js(capability),
        ))
    }
}

fn out(
    constructor: OutputConstructor,
) -> impl Fn() -> Result<Box<dyn rune_runtime::Output>, Error>
       + Send
       + Sync
       + 'static {
    let constructor = AssertSyncBecauseWasmIsSingleThreaded(constructor);

    move || {
        let function: &Function = constructor.0.as_ref();
        let capability = function.call0(&JsValue::NULL).map_err(rust_error)?;

        Ok(Box::new(
            crate::external_types::Output::unchecked_from_js(capability),
        ))
    }
}

struct AssertSyncBecauseWasmIsSingleThreaded<T>(T);

unsafe impl<T> Send for AssertSyncBecauseWasmIsSingleThreaded<T> {}
unsafe impl<T> Sync for AssertSyncBecauseWasmIsSingleThreaded<T> {}
