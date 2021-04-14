use std::sync::{Arc, Mutex};

use js_sys::WebAssembly::Memory;
use rune_runtime::Function;
use wasm_bindgen::{JsValue, UnwrapThrowExt, prelude::Closure};

/// Wrap a [`Function`] in a [`Closure`] that can be called from JavaScript,
/// passing a reference to the provided closure and returning *something* that
/// can be dropped.
///
/// This is a bit more cumbersome to use as you'd expect because [`Closure`]
/// doesn't give you a way to erase the Rust function's signature.
pub(crate) fn with_wrapped_closure(
    function: Function,
    memory: Arc<Mutex<Option<Memory>>>,
    register: impl FnOnce(&JsValue),
) -> Box<dyn Drop> {
    let signature = function.signature();

    match signature.parameters() {
        [] => {
            let closure = move || {
                let ctx = Context {
                    memory: Arc::clone(&memory),
                };
                function.call(&ctx, &[]).expect_throw("Call failed");
            };
            let closure = Closure::wrap(Box::new(closure) as Box<dyn Fn()>);
            register(closure.as_ref());
            Box::new(closure)
        },
        _ => panic!("Unable to wrap a `{}`", signature),
    }
}

#[derive(Clone)]
struct Context {
    memory: Arc<Mutex<Option<Memory>>>,
}

impl rune_runtime::CallContext for Context {
    fn memory(&self, _address: u32, _len: u32) -> Result<&[u8], anyhow::Error> {
        todo!()
    }

    unsafe fn memory_mut(
        &self,
        _address: u32,
        _len: u32,
    ) -> Result<&mut [u8], anyhow::Error> {
        todo!()
    }
}
